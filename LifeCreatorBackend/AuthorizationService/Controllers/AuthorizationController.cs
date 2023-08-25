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
    private readonly ICryptographyService cryptography;
    private readonly IMailSenderService mailSender;
    private readonly IJwtTokenToolsService jwtTokenTools;
    private readonly IMailBodyBuilder mailBodyBuilder;
    private readonly TokensLifeTimeSettings tokensLifeTime;
    private readonly ILogger<AuthorizationController> logger;

    public AuthorizationController(
        AuthorizationDbContext dbContext,
        IMailSenderService mailSender,
        ICryptographyService cryptography,
        IJwtTokenToolsService jwtTokenTools,
        IMailBodyBuilder mailBodyBuilder,
        IOptions<TokensLifeTimeSettings> tokensLifeTime,
        ILogger<AuthorizationController> logger
    )
    {
        authorizationDbContext = dbContext;
        this.logger = logger;
        this.mailSender = mailSender;
        this.cryptography = cryptography;
        this.jwtTokenTools = jwtTokenTools;
        this.mailBodyBuilder = mailBodyBuilder;
        this.tokensLifeTime = tokensLifeTime.Value;
    }

    [HttpGet("/PublicKey")]
    public IActionResult GetPublicKey()
    {
        logger.LogDefaultInfo(Request);
        return Ok(cryptography.GetPublicKey());
    }

    public sealed class LogInData
    {
        public required string Signature { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost("/Login")]
    public async Task<IActionResult> Login([FromBody] LogInData logInData)
    {
        logger.LogDefaultInfo(Request);
        User? foundUser = null;
        foreach (User? user in await authorizationDbContext.Users.ToListAsync())
        {
            if (
                cryptography.HashString(user.Login + logInData.Nonce + user.HashedPassword)
                == logInData.Signature
            )
            {
                foundUser = user;
                break;
            }
        }
        if (foundUser is null)
        {
            return BadRequest("Invalid login or password.".ToErrorBody());
        }
        else if (foundUser.EmailVerification is EmailVerificationState.NotVerified)
        {
            return BadRequest("Email not verified.".ToErrorBody());
        }
        else
        {
            Response.Headers.Add(
                "JwtBearerToken",
                jwtTokenTools.GenerateToken(foundUser.Login, tokensLifeTime.LoginTokenDuration)
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
        logger.LogDefaultInfo(Request);
        if (authorizationDbContext.Users.Any(x => x.Login == registrationData.Login))
        {
            return BadRequest("This login is already taken.".ToErrorBody());
        }

        string email = cryptography
            .DecryptString(registrationData.EncryptedNonceWithEmail)
            .Replace(registrationData.Nonce, "");

        if (!new EmailAddressAttribute().IsValid(email))
        {
            return BadRequest("Invalid email.".ToErrorBody());
        }
        if (authorizationDbContext.Users.Any(x => x.Email == email))
        {
            return BadRequest("This email is already in use.".ToErrorBody());
        }

        User newUser =
            new()
            {
                Login = registrationData.Login,
                Email = email,
                HashedPassword = cryptography.DecryptString(
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
            return BadRequest("Internal error.".ToErrorBody());
        }

        EmailVerification emailVerification =
            new()
            {
                User = addedUser,
                JwtToken = jwtTokenTools.GenerateToken(
                    addedUser.Login,
                    tokensLifeTime.EmailValidationTokenDuration
                ),
                RequestDate = DateTime.UtcNow
            };

        _ = await authorizationDbContext.EmailVerifications.AddAsync(emailVerification);
        _ = await authorizationDbContext.SaveChangesAsync();

        Result result = await mailSender.SendAsync(
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
        logger.LogDefaultInfo(Request);
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
        if (jwtTokenTools.ValidateToken(JwtToken).Failure)
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

        Result result = await mailSender.SendAsync(
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
        logger.LogDefaultInfo(Request);
        string email = cryptography
            .DecryptString(resendEmailVerificationData.EncryptedNonceWithEmail)
            .Replace(resendEmailVerificationData.Nonce, "");

        User? user = await authorizationDbContext.Users.FirstOrDefaultAsync(x => x.Email == email);

        if (user is null)
        {
            return BadRequest("User with this email is not exist.".ToErrorBody());
        }

        EmailVerification emailVerification =
            new()
            {
                User = user,
                JwtToken = jwtTokenTools.GenerateToken(
                    user.Login,
                    tokensLifeTime.EmailValidationTokenDuration
                ),
                RequestDate = DateTime.UtcNow
            };

        _ = authorizationDbContext.EmailVerifications.Add(emailVerification);
        _ = authorizationDbContext.SaveChanges();

        Result result = await mailSender.SendAsync(
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
        logger.LogDefaultInfo(Request);
        string email = cryptography
            .DecryptString(forgotPasswordData.EncryptedNonceWithEmail)
            .Replace(forgotPasswordData.Nonce, "");

        User? user = await authorizationDbContext.Users.FirstOrDefaultAsync(x => x.Email == email);

        if (user is null)
        {
            return BadRequest("User with this email is not exist.".ToErrorBody());
        }

        if (user.EmailVerification is EmailVerificationState.NotVerified)
        {
            return BadRequest("Email not verified.".ToErrorBody());
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

        Result result = await mailSender.SendAsync(
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
        logger.LogDefaultInfo(Request);
        IEnumerable<PasswordRecover> codes = await authorizationDbContext.PasswordRecovers
            .Where(x => x.AccessCode == recoverPasswordData.AccessCode)
            .ToListAsync();

        if (!codes.Any())
        {
            return BadRequest("Invalid Access code.".ToErrorBody());
        }

        codes = codes.Where(x => x.IsValid(tokensLifeTime.AccessCodeDuration));

        if (!codes.Any())
        {
            return BadRequest("Access code duration expired.".ToErrorBody());
        }
        else if (codes.Count() > 1)
        {
            return BadRequest("Internal error, try again.".ToErrorBody());
        }

        User? user = authorizationDbContext.Users.FirstOrDefault(
            x => x.PasswordRecovers.Contains(codes.First())
        );

        if (user == null)
        {
            return BadRequest("Internal error, try again.".ToErrorBody());
        }

        if (user.EmailVerification is EmailVerificationState.NotVerified)
        {
            return BadRequest("Email not verified.".ToErrorBody());
        }

        user.HashedPassword = cryptography.DecryptString(
            recoverPasswordData.EncryptedHashedPassword
        );
        _ = await authorizationDbContext.SaveChangesAsync();

        return Accepted();
    }
}
