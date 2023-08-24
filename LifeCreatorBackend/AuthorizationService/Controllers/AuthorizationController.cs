using AuthorizationService.Common;
using AuthorizationService.Data;
using AuthorizationService.Models;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Options;
using System.ComponentModel.DataAnnotations;
using System.Text.Json;

namespace AuthorizationService.Controllers;

[ApiController]
[Route("[controller]")]
public sealed class AuthorizationController : ControllerBase
{
    private readonly AuthorizationDbContext authorizationDbContext;
    private readonly ICryptographyService cryptographyService;
    private readonly IMailService mailService;
    private readonly IJwtTokenService jwtTokenService;
    private readonly TokensLifeTimeSettings tokensLifeTimeSettings;

    public AuthorizationController(
        AuthorizationDbContext dbContext,
        IMailService mailService,
        ICryptographyService cryptographyService,
        IJwtTokenService jwtTokenService,
        IOptions<TokensLifeTimeSettings> tokensLifeTimeSettings
    )
    {
        authorizationDbContext = dbContext;
        this.mailService = mailService;
        this.cryptographyService = cryptographyService;
        this.jwtTokenService = jwtTokenService;
        this.tokensLifeTimeSettings = tokensLifeTimeSettings.Value;
    }

    [HttpPost("/Authorization/LogIn")]
    public IActionResult Login([FromBody] JsonElement bodyJson)
    {
        LogInData? logInData = bodyJson.Deserialize<LogInData>();
        if (logInData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }
        List<User> users = authorizationDbContext.Users.ToList();
        User? user = users.Find(
            x =>
                cryptographyService.HashString(x.Login + logInData.Nonce + x.HashedPassword)
                == logInData.Signature
        );
        if (user is null)
        {
            return NotFound("User is not exist.");
        }
        else
        {
            Response.Headers.Add(
                "JwtBearerToken",
                jwtTokenService.GenerateToken(user.Login, tokensLifeTimeSettings.LoginTokenDuration)
            );
            return Accepted();
        }
    }

    [Serializable]
    public sealed class LogInData
    {
        public required string Signature { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Authorization/Registration")]
    public IActionResult Registration([FromBody] JsonElement bodyJson)
    {
        RegistrationData? registrationData = bodyJson.Deserialize<RegistrationData>();
        if (registrationData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }

        if (authorizationDbContext.Users.Any(x => x.Login == registrationData.Login))
        {
            return BadRequest("This login is already taken.");
        }

        string email = cryptographyService
            .DecryptString(registrationData.EncryptedNonceWithEmail)
            .Replace(registrationData.Nonce, "");

        if (!new EmailAddressAttribute().IsValid(email))
        {
            return BadRequest("Invalid email.");
        }
        if (authorizationDbContext.Users.Any(x => x.Email == email))
        {
            return BadRequest("This email is already in use.");
        }

        User newUser =
            new()
            {
                Login = registrationData.Login,
                Email = email,
                HashedPassword = cryptographyService.DecryptString(
                    registrationData.EncryptedHashedPassword
                ),
                RegistrationDate = DateTime.UtcNow,
                EmailVerification = EmailVerificationState.NotVitrificated
            };
        _ = authorizationDbContext.Users.Add(newUser);
        _ = authorizationDbContext.SaveChanges();

        User? addedUser = authorizationDbContext.Users.FirstOrDefault(
            x => x.Login == newUser.Login
        );

        if (addedUser == null)
        {
            return Problem("Cannot add new User");
        }

        EmailVerification emailVerification =
            new()
            {
                User = addedUser,
                JwtToken = jwtTokenService.GenerateToken(
                    addedUser.Login,
                    tokensLifeTimeSettings.EmailValidationTokenDuration
                ),
                RequestDate = DateTime.UtcNow
            };

        _ = authorizationDbContext.EmailVerifications.Add(emailVerification);
        _ = authorizationDbContext.SaveChanges();

        string htmlContent = System.IO.File.ReadAllText(
            "./View/EmailBodyPrototypes/Registration.html"
        );
        htmlContent = htmlContent.Replace("--JWTTOKEN--", emailVerification.JwtToken);

        Result result = mailService
            .SendAsync(
                new MailData()
                {
                    ReceiverName = registrationData.Login,
                    ReceiverEmail = email,
                    Subject = "Registration confirm into LifeCreator",
                    HtmlContent = htmlContent
                }
            )
            .Result;

        return result.Success
            ? Ok("Mail has successfully been sent.")
            : Problem($"An error occurred. The Mail could not be sent. Problem: {result.Error}");
    }

    [Serializable]
    private sealed class RegistrationData
    {
        public required string Login { get; set; }
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
        public required string EncryptedHashedPassword { get; set; }
    }

    [HttpPost("/Authorization/EmailRegistration")]
    public IActionResult RegistrationByEmail([FromForm] string JwtToken)
    {
        EmailVerification? emailVerification =
            authorizationDbContext.EmailVerifications.FirstOrDefault(x => x.JwtToken == JwtToken);
        if (emailVerification is null)
        {
            return Problem("No such email verification request.");
        }
        User? user = authorizationDbContext.Users.FirstOrDefault(
            x => x.EmailVerifications.Contains(emailVerification)
        );
        if (user is null)
        {
            return Problem("No such user with verification request.");
        }
        if (user.EmailVerification is EmailVerificationState.Vitrificated)
        {
            return Accepted("Your email are Already verified, you can login.");
        }
        if (jwtTokenService.ValidateToken(JwtToken).Failure)
        {
            return Problem("Validation failed, try ReRegister.");
        }
        user.EmailVerification = EmailVerificationState.Vitrificated;
        _ = authorizationDbContext.SaveChanges();
        return Ok("Successful registration.");
    }

    [Serializable]
    private sealed class ResendEmailVerificationData
    {
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Authorization/ResendEmailVerification")]
    public IActionResult ResendEmailVerification([FromBody] JsonElement bodyJson)
    {
        ResendEmailVerificationData? resendEmailVerificationData =
            bodyJson.Deserialize<ResendEmailVerificationData>();
        if (resendEmailVerificationData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }

        string email = cryptographyService
            .DecryptString(resendEmailVerificationData.EncryptedNonceWithEmail)
            .Replace(resendEmailVerificationData.Nonce, "");

        User? user = authorizationDbContext.Users.FirstOrDefault(x => x.Email == email);

        if (user is null)
        {
            return BadRequest("User with this email is not exist.");
        }

        EmailVerification emailVerification =
            new()
            {
                User = user,
                JwtToken = jwtTokenService.GenerateToken(
                    user.Login,
                    tokensLifeTimeSettings.EmailValidationTokenDuration
                ),
                RequestDate = DateTime.UtcNow
            };

        _ = authorizationDbContext.EmailVerifications.Add(emailVerification);
        _ = authorizationDbContext.SaveChanges();

        string htmlContent = System.IO.File.ReadAllText(
            "./View/EmailBodyPrototypes/Registration.html"
        );
        htmlContent = htmlContent.Replace("--JWTTOKEN--", emailVerification.JwtToken);

        Result result = mailService
            .SendAsync(
                new MailData()
                {
                    ReceiverName = user.Login,
                    ReceiverEmail = email,
                    Subject = "Duplicated Registration confirm into LifeCreator",
                    HtmlContent = htmlContent
                }
            )
            .Result;

        return result.Success
            ? Ok("Mail has successfully been sent.")
            : Problem($"An error occurred. The Mail could not be sent. Problem: {result.Error}");
    }

    [Serializable]
    private sealed class ForgotPasswordData
    {
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Authorization/ForgotPassword")]
    public IActionResult ForgotPassword([FromBody] JsonElement bodyJson)
    {
        ForgotPasswordData? forgotPasswordData = bodyJson.Deserialize<ForgotPasswordData>();
        if (forgotPasswordData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }

        string email = cryptographyService
            .DecryptString(forgotPasswordData.EncryptedNonceWithEmail)
            .Replace(forgotPasswordData.Nonce, "");

        User? user = authorizationDbContext.Users.FirstOrDefault(x => x.Email == email);

        if (user is null)
        {
            return BadRequest("User with this email is not exist.");
        }

        PasswordRecover passwordRecover =
            new()
            {
                User = user,
                AccessCode = new Random().Next(10001, 99999),
                RequestDate = DateTime.UtcNow
            };

        _ = authorizationDbContext.PasswordRecovers.Add(passwordRecover);
        _ = authorizationDbContext.SaveChanges();

        string htmlContent = System.IO.File.ReadAllText(
            "./View/EmailBodyPrototypes/AccessCode.html"
        );
        htmlContent = htmlContent.Replace(
            "--ACCESSCODE--",
            Convert.ToString(passwordRecover.AccessCode)
        );

        Result result = mailService
            .SendAsync(
                new MailData()
                {
                    ReceiverName = user.Login,
                    ReceiverEmail = email,
                    Subject = "AccessCode to recover password",
                    HtmlContent = htmlContent
                }
            )
            .Result;

        return result.Success
            ? Ok("Mail has successfully been sent.")
            : Problem($"An error occurred. The Mail could not be sent. Problem: {result.Error}");
    }

    [Serializable]
    private sealed class RecoverPasswordData
    {
        public required int AccessCode { get; set; }
        public required string EncryptedHashedPassword { get; set; }
    }

    [HttpPost("/Authorization/RecoverPassword")]
    public IActionResult RecoverPassword([FromBody] JsonElement bodyJson)
    {
        RecoverPasswordData? recoverPasswordData = bodyJson.Deserialize<RecoverPasswordData>();
        if (recoverPasswordData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }

        List<PasswordRecover> codes = authorizationDbContext.PasswordRecovers
            .Where(x => x.AccessCode == recoverPasswordData.AccessCode)
            .ToList();

        if (codes.Count == 0)
        {
            return BadRequest("Invalid Access code.");
        }

        codes = codes
            .Where(
                x =>
                    DateTime.UtcNow.Subtract(x.RequestDate)
                    <= tokensLifeTimeSettings.AccessCodeDuration
            )
            .ToList();

        if (codes.Count == 0)
        {
            return BadRequest("Access code duration expired.");
        }
        else if (codes.Count > 1)
        {
            return BadRequest("Internal error, try again.");
        }

        User? user = authorizationDbContext.Users.FirstOrDefault(
            x => x.PasswordRecovers.Contains(codes.Last())
        );

        if (user == null)
        {
            return BadRequest("Internal error, try again.");
        }

        user.HashedPassword = cryptographyService.DecryptString(
            recoverPasswordData.EncryptedHashedPassword
        );
        _ = authorizationDbContext.SaveChanges();

        return Accepted("Password has been changed");
    }
}
