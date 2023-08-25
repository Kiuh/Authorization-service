using AuthorizationService.Common;
using AuthorizationService.Data;
using AuthorizationService.Models;
using AuthorizationService.Pages;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Options;
using System.ComponentModel.DataAnnotations;

namespace AuthorizationService.Controllers;

[ApiController]
[Route("[controller]")]
public sealed class AuthorizationController : ControllerBase
{
    private readonly AuthorizationDbContext authorizationDbContext;
    private readonly ICryptographyService cryptographyService;
    private readonly IMailSenderService mailService;
    private readonly IJwtTokenToolsService jwtTokenService;
    private readonly IMailBodyBuilder mailBodyBuilder;
    private readonly TokensLifeTimeSettings tokensLifeTimeSettings;

    public AuthorizationController(
        AuthorizationDbContext dbContext,
        IMailSenderService mailService,
        ICryptographyService cryptographyService,
        IJwtTokenToolsService jwtTokenService,
        IMailBodyBuilder mailBodyBuilder,
        IOptions<TokensLifeTimeSettings> tokensLifeTimeSettings
    )
    {
        authorizationDbContext = dbContext;
        this.mailService = mailService;
        this.cryptographyService = cryptographyService;
        this.jwtTokenService = jwtTokenService;
        this.mailBodyBuilder = mailBodyBuilder;
        this.tokensLifeTimeSettings = tokensLifeTimeSettings.Value;
    }

    [Serializable]
    public sealed class LogInData
    {
        public required string Signature { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Authorization/LogIn")]
    public IActionResult Login([FromBody] LogInData logInData)
    {
        User? user = authorizationDbContext.Users.FirstOrDefault(
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

    public sealed class RegistrationData
    {
        public required string Login { get; set; }
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
        public required string EncryptedHashedPassword { get; set; }
    }

    [HttpPost("/Authorization/Registration")]
    public async Task<IActionResult> Registration([FromBody] RegistrationData registrationData)
    {
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
        _ = await authorizationDbContext.Users.AddAsync(newUser);
        _ = await authorizationDbContext.SaveChangesAsync();

        User? addedUser = await authorizationDbContext.Users.FirstOrDefaultAsync(
            x => x.Login == newUser.Login
        );

        if (addedUser == null)
        {
            return Problem("Internal server error");
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

        _ = await authorizationDbContext.EmailVerifications.AddAsync(emailVerification);
        _ = await authorizationDbContext.SaveChangesAsync();

        Result result = await mailService.SendAsync(
            mailBodyBuilder.VerificationMail(
                registrationData.Login,
                "",
                email,
                emailVerification.JwtToken
            )
        );

        return result.Success
            ? Ok("Mail has successfully been sent.")
            : Problem($"An error occurred. The Mail could not be sent. Problem: {result.Error}");
    }

    [HttpPost("/Authorization/EmailRegistration")]
    public async Task<IActionResult> RegistrationByEmail([FromForm] string JwtToken)
    {
        EmailVerification? emailVerification =
            await authorizationDbContext.EmailVerifications.FirstOrDefaultAsync(
                x => x.JwtToken == JwtToken
            );
        if (emailVerification is null)
        {
            return RedirectToPage(
                "/ErrorPage",
                new ErrorPageInfo()
                {
                    StatusCode = "500",
                    Title = "Internal Server Error",
                    Labels = new() { $"No such email verification request." }
                }
            );
        }
        User? user = await authorizationDbContext.Users.FirstOrDefaultAsync(
            x => x.EmailVerifications.Contains(emailVerification)
        );
        if (user is null)
        {
            return RedirectToPage(
                "/ErrorPage",
                new ErrorPageInfo()
                {
                    StatusCode = "500",
                    Title = "Internal Server Error",
                    Labels = new() { "Validation failed, try ReRegister." }
                }
            );
        }
        if (user.EmailVerification is EmailVerificationState.Vitrificated)
        {
            return RedirectToPage("/AlreadyVerified");
        }
        if (jwtTokenService.ValidateToken(JwtToken).Failure)
        {
            return RedirectToPage(
                "/ErrorPage",
                new ErrorPageInfo()
                {
                    StatusCode = "500",
                    Title = "Internal Server Error",
                    Labels = new() { "Validation failed, request to send email again." }
                }
            );
        }
        user.EmailVerification = EmailVerificationState.Vitrificated;
        _ = await authorizationDbContext.SaveChangesAsync();

        Result result = await mailService.SendAsync(
            mailBodyBuilder.WelcomeMail(user.Login, user.Email)
        );

        return RedirectToPage("/SuccessVerification");
    }

    public sealed class ResendEmailVerificationData
    {
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Authorization/ResendEmailVerification")]
    public async Task<IActionResult> ResendEmailVerification(
        [FromBody] ResendEmailVerificationData resendEmailVerificationData
    )
    {
        string email = cryptographyService
            .DecryptString(resendEmailVerificationData.EncryptedNonceWithEmail)
            .Replace(resendEmailVerificationData.Nonce, "");

        User? user = await authorizationDbContext.Users.FirstOrDefaultAsync(x => x.Email == email);

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

        Result result = await mailService.SendAsync(
            mailBodyBuilder.VerificationMail(user.Login, "", email, emailVerification.JwtToken)
        );

        return result.Success
            ? Ok("Mail has successfully been sent.")
            : Problem($"An error occurred. The Mail could not be sent. Problem: {result.Error}");
    }

    public sealed class ForgotPasswordData
    {
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Authorization/ForgotPassword")]
    public async Task<IActionResult> ForgotPassword(
        [FromBody] ForgotPasswordData forgotPasswordData
    )
    {
        string email = cryptographyService
            .DecryptString(forgotPasswordData.EncryptedNonceWithEmail)
            .Replace(forgotPasswordData.Nonce, "");

        User? user = await authorizationDbContext.Users.FirstOrDefaultAsync(x => x.Email == email);

        if (user is null)
        {
            return BadRequest("User with this email is not exist.");
        }

        PasswordRecover passwordRecover =
            new()
            {
                User = user,
                AccessCode = new Random().Next(100001, 999999),
                RequestDate = DateTime.UtcNow
            };

        _ = await authorizationDbContext.PasswordRecovers.AddAsync(passwordRecover);
        _ = await authorizationDbContext.SaveChangesAsync();

        Result result = await mailService.SendAsync(
            mailBodyBuilder.AccessCodeMail(user.Login, email, passwordRecover.AccessCode)
        );

        return result.Success
            ? Ok("Mail has successfully been sent.")
            : Problem($"An error occurred. The Mail could not be sent. Problem: {result.Error}");
    }

    public sealed class RecoverPasswordData
    {
        public required int AccessCode { get; set; }
        public required string EncryptedHashedPassword { get; set; }
    }

    [HttpPost("/Authorization/RecoverPassword")]
    public async Task<IActionResult> RecoverPassword(
        [FromBody] RecoverPasswordData recoverPasswordData
    )
    {
        IEnumerable<PasswordRecover> codes = await authorizationDbContext.PasswordRecovers
            .Where(x => x.AccessCode == recoverPasswordData.AccessCode)
            .ToListAsync();

        if (codes.Any())
        {
            return BadRequest("Invalid Access code.");
        }

        codes = codes.Where(x => x.IsValid(tokensLifeTimeSettings.AccessCodeDuration));

        if (codes.Any())
        {
            return BadRequest("Access code duration expired.");
        }
        else if (codes.Count() > 1)
        {
            return BadRequest("Internal error, try again.");
        }

        User? user = await authorizationDbContext.Users.FirstOrDefaultAsync(
            x => x.PasswordRecovers.Contains(codes.Last())
        );

        if (user == null)
        {
            return BadRequest("Internal error, try again.");
        }

        user.HashedPassword = cryptographyService.DecryptString(
            recoverPasswordData.EncryptedHashedPassword
        );
        _ = await authorizationDbContext.SaveChangesAsync();

        return Accepted("Password has been changed");
    }
}
