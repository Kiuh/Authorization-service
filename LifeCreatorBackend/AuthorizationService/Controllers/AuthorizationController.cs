using AuthorizationService.Common;
using AuthorizationService.Dto;
using AuthorizationService.Models;
using AuthorizationService.Services;
using AuthorizationService.Services.Mail;
using AuthorizationService.Services.Models;
using Microsoft.AspNetCore.Mvc;

namespace AuthorizationService.Controllers;

[ApiController]
public sealed class AuthorizationController : ControllerBase
{
    private readonly IMailSenderService mailSenderService;
    private readonly IJwtTokenToolsService jwtTokenToolsService;
    private readonly IMailBodyBuilder mailBodyBuilderService;
    private readonly ILogger<AuthorizationController> logger;
    private readonly IUsersService usersService;
    private readonly IEmailVerificationsService emailVerificationsService;
    private readonly IPasswordRecoversService passwordRecoversService;

    public AuthorizationController(
        IMailSenderService mailSenderService,
        IJwtTokenToolsService jwtTokenToolsService,
        IMailBodyBuilder mailBodyBuilderService,
        ILogger<AuthorizationController> logger,
        IUsersService usersService,
        IEmailVerificationsService emailVerificationsService,
        IPasswordRecoversService passwordRecoversService
    )
    {
        this.logger = logger;
        this.mailSenderService = mailSenderService;
        this.jwtTokenToolsService = jwtTokenToolsService;
        this.mailBodyBuilderService = mailBodyBuilderService;
        this.usersService = usersService;
        this.emailVerificationsService = emailVerificationsService;
        this.passwordRecoversService = passwordRecoversService;
    }

    [HttpPost("/Login")]
    public async Task<IActionResult> Login([FromBody] LoginDto loginDto)
    {
        logger.LogDefaultInfo(Request);
        User user = await usersService.FindUserBySignatureAndNonce(
            loginDto.Signature,
            loginDto.Nonce
        );

        if (user.EmailVerification is EmailVerificationState.NotVerified)
        {
            throw new Exception("Email not verified.");
        }

        jwtTokenToolsService.SetLoginJwtTokenHeader(user, Response.Headers);
        return Ok();
    }

    [HttpPut("/Registration")]
    public async Task<IActionResult> Registration([FromBody] RegistrationDto registrationDto)
    {
        logger.LogDefaultInfo(Request);

        User user = usersService.CreateUserFromRegistration(registrationDto);

        await usersService.AddNewUser(user);

        EmailVerification emailVerification = jwtTokenToolsService.CreateEmailVerification(user);

        await emailVerificationsService.AddEmailVerification(emailVerification);

        await mailSenderService.SendAsync(
            mailBodyBuilderService.CreateVerificationMail(emailVerification)
        );

        return Ok();
    }

    [HttpPost("/Verification")]
    public async Task<IActionResult> Verification([FromForm] string JwtToken)
    {
        logger.LogDefaultInfo(Request);

        EmailVerification emailVerification;
        try
        {
            emailVerification = await emailVerificationsService.FindEmailVerificationByJwtToken(
                JwtToken
            );
        }
        catch (Exception ex)
        {
            return RedirectToPage("/ErrorPage", new ErrorPageDto(ex.Message));
        }

        User user;
        try
        {
            user = await usersService.FindUserByEmailVerification(emailVerification);
        }
        catch (Exception ex)
        {
            return RedirectToPage("/ErrorPage", new ErrorPageDto(ex.Message));
        }

        if (user.EmailVerification is EmailVerificationState.Verified)
        {
            return RedirectToPage("/AlreadyVerified");
        }
        if (jwtTokenToolsService.ValidateToken(JwtToken))
        {
            return RedirectToPage(
                "/ErrorPage",
                new ErrorPageDto("Validation failed, request to send email again.")
            );
        }

        await emailVerificationsService.VerifyUserEmail(user);

        await mailSenderService.SendAsync(mailBodyBuilderService.CreateWelcomeMail(user));

        return RedirectToPage("/SuccessVerification");
    }

    [HttpPost("/ResendRegistration")]
    public async Task<IActionResult> ResendVerification(
        [FromBody] ResendVerificationDto resendEmailVerificationDto
    )
    {
        logger.LogDefaultInfo(Request);

        User user = await usersService.FindUserByEncryptedNonceWithEmail(
            resendEmailVerificationDto.EncryptedNonceWithEmail,
            resendEmailVerificationDto.Nonce
        );

        EmailVerification emailVerification = jwtTokenToolsService.CreateEmailVerification(user);

        await emailVerificationsService.AddEmailVerification(emailVerification);

        await mailSenderService.SendAsync(
            mailBodyBuilderService.CreateVerificationMail(emailVerification)
        );

        return Ok();
    }

    [HttpPost("/ForgotPassword")]
    public async Task<IActionResult> ForgotPassword([FromBody] ForgotPasswordDto forgotPasswordDto)
    {
        logger.LogDefaultInfo(Request);

        User user = await usersService.FindUserByEncryptedNonceWithEmail(
            forgotPasswordDto.EncryptedNonceWithEmail,
            forgotPasswordDto.Nonce
        );

        if (user.EmailVerification is EmailVerificationState.NotVerified)
        {
            throw new Exception("Email not verified.");
        }

        PasswordRecover passwordRecover = passwordRecoversService.CreatePasswordRecover(user);

        await passwordRecoversService.AddPasswordRecover(passwordRecover);

        await mailSenderService.SendAsync(
            mailBodyBuilderService.CreateAccessCodeMail(user, passwordRecover.AccessCode)
        );

        return Ok();
    }

    [HttpPost("/RecoverPassword")]
    public async Task<IActionResult> RecoverPassword(
        [FromBody] RecoverPasswordDto recoverPasswordDto
    )
    {
        logger.LogDefaultInfo(Request);

        User user = await passwordRecoversService.FindUserWithValidAccessCode(
            recoverPasswordDto.AccessCode
        );

        if (user.EmailVerification is EmailVerificationState.NotVerified)
        {
            throw new Exception("Email not verified.");
        }

        await usersService.SetNewUserPassword(user, recoverPasswordDto.EncryptedHashedPassword);

        return Ok();
    }
}
