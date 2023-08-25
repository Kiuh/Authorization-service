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

    [HttpGet("/PublicKey")]
    public IActionResult GetPublicKey()
    {
        return Ok(cryptographyService.GetPublicKey());
    }

    public sealed class LogInData
    {
        public required string Signature { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Login")]
    public async Task<IActionResult> Login([FromBody] LogInData logInData)
    {
        User? foundUser = null;
        foreach (User? user in await authorizationDbContext.Users.ToListAsync())
        {
            if (
                cryptographyService.HashString(user.Login + logInData.Nonce + user.HashedPassword)
                == logInData.Signature
            )
            {
                foundUser = user;
                break;
            }
        }
        if (foundUser is null)
        {
            return NotFound();
        }
        else if (foundUser.EmailVerification is EmailVerificationState.NotVerified)
        {
            return BadRequest(error: "Email not verified.");
        }
        else
        {
            Response.Headers.Add(
                "JwtBearerToken",
                jwtTokenService.GenerateToken(
                    foundUser.Login,
                    tokensLifeTimeSettings.LoginTokenDuration
                )
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

    [HttpPut("/Registration")]
    public async Task<IActionResult> Registration([FromBody] RegistrationData registrationData)
    {
        if (authorizationDbContext.Users.Any(x => x.Login == registrationData.Login))
        {
            return BadRequest(error: "This login is already taken.");
        }

        string email = cryptographyService
            .DecryptString(registrationData.EncryptedNonceWithEmail)
            .Replace(registrationData.Nonce, "");

        if (!new EmailAddressAttribute().IsValid(email))
        {
            return BadRequest(error: "Invalid email.");
        }
        if (authorizationDbContext.Users.Any(x => x.Email == email))
        {
            return BadRequest(error: "This email is already in use.");
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
                EmailVerification = EmailVerificationState.NotVerified
            };
        _ = await authorizationDbContext.Users.AddAsync(newUser);
        _ = await authorizationDbContext.SaveChangesAsync();

        User? addedUser = await authorizationDbContext.Users.FirstOrDefaultAsync(
            x => x.Login == newUser.Login
        );

        if (addedUser == null)
        {
            return Problem();
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
                email,
                emailVerification.JwtToken
            )
        );

        return result.Success ? Ok() : Problem();
    }

    [HttpPost("/Verification")]
    public async Task<IActionResult> Verification([FromForm] string JwtToken)
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
        if (user.EmailVerification is EmailVerificationState.Verified)
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
        user.EmailVerification = EmailVerificationState.Verified;
        _ = await authorizationDbContext.SaveChangesAsync();

        Result result = await mailService.SendAsync(
            mailBodyBuilder.WelcomeMail(user.Login, user.Email)
        );

        return RedirectToPage("/SuccessVerification");
    }

    public sealed class ResendVerificationData
    {
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/ResendRegistration")]
    public async Task<IActionResult> ResendVerification(
        [FromBody] ResendVerificationData resendEmailVerificationData
    )
    {
        string email = cryptographyService
            .DecryptString(resendEmailVerificationData.EncryptedNonceWithEmail)
            .Replace(resendEmailVerificationData.Nonce, "");

        User? user = await authorizationDbContext.Users.FirstOrDefaultAsync(x => x.Email == email);

        if (user is null)
        {
            return BadRequest(error: "User with this email is not exist.");
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
            mailBodyBuilder.VerificationMail(user.Login, email, emailVerification.JwtToken)
        );

        return result.Success ? Ok() : Problem();
    }

    public sealed class ForgotPasswordData
    {
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/ForgotPassword")]
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
            return BadRequest(error: "User with this email is not exist.");
        }

        if (user.EmailVerification is EmailVerificationState.NotVerified)
        {
            return BadRequest(error: "Email not verified.");
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

        return result.Success ? Ok() : Problem();
    }

    public sealed class RecoverPasswordData
    {
        public required int AccessCode { get; set; }
        public required string EncryptedHashedPassword { get; set; }
    }

    [HttpPost("/RecoverPassword")]
    public async Task<IActionResult> RecoverPassword(
        [FromBody] RecoverPasswordData recoverPasswordData
    )
    {
        IEnumerable<PasswordRecover> codes = await authorizationDbContext.PasswordRecovers
            .Where(x => x.AccessCode == recoverPasswordData.AccessCode)
            .ToListAsync();

        if (!codes.Any())
        {
            return BadRequest(error: "Invalid Access code.");
        }

        codes = codes.Where(x => x.IsValid(tokensLifeTimeSettings.AccessCodeDuration));

        if (!codes.Any())
        {
            return BadRequest(error: "Access code duration expired.");
        }
        else if (codes.Count() > 1)
        {
            return BadRequest(error: "Internal error, try again.");
        }

        User? user = authorizationDbContext.Users.FirstOrDefault(
            x => x.PasswordRecovers.Contains(codes.First())
        );

        if (user == null)
        {
            return BadRequest(error: "Internal error, try again.");
        }

        if (user.EmailVerification is EmailVerificationState.NotVerified)
        {
            return BadRequest(error: "Email not verified.");
        }

        user.HashedPassword = cryptographyService.DecryptString(
            recoverPasswordData.EncryptedHashedPassword
        );
        _ = await authorizationDbContext.SaveChangesAsync();

        return Accepted("Password has been changed");
    }
}
