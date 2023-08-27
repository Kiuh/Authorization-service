using AuthorizationService.Models;
using Microsoft.Extensions.Options;

namespace AuthorizationService.Services.Mail;

public class MailBodyBuilderSettings
{
    public required string VerificationLink { get; set; }
}

public interface IMailBodyBuilder
{
    public MailData CreateWelcomeMail(User user);
    public MailData CreateVerificationMail(EmailVerification emailVerification);
    public MailData CreateAccessCodeMail(User user, int accessCode);
}

public class MailBodyBuilderService : IMailBodyBuilder
{
    private string welcomeMailTemplate;
    private string verificationMailTemplate;
    private string accessCodeMailTemplate;
    private string verificationLink;

    public MailBodyBuilderService(IOptions<MailBodyBuilderSettings> mailBodyBuilderSettings)
    {
        string basePath = "./Mails/";
        welcomeMailTemplate = File.ReadAllText(basePath + "Welcome.html");
        verificationMailTemplate = File.ReadAllText(basePath + "Verification.html");
        accessCodeMailTemplate = File.ReadAllText(basePath + "AccessCode.html");
        verificationLink = mailBodyBuilderSettings.Value.VerificationLink;
    }

    public MailData CreateWelcomeMail(User user)
    {
        return new MailData()
        {
            ReceiverName = user.Login,
            ReceiverEmail = user.Email,
            Subject = "Welcome to Life Creator!",
            HtmlContent = welcomeMailTemplate
        };
    }

    public MailData CreateVerificationMail(EmailVerification emailVerification)
    {
        string htmlContent = verificationMailTemplate.Replace("|--JWTTOKEN--|", emailVerification.JwtToken);
        htmlContent = htmlContent.Replace("|--LINK--|", verificationLink);
        return new MailData()
        {
            ReceiverName = emailVerification.User.Login,
            ReceiverEmail = emailVerification.User.Email,
            Subject = "Verify your email!",
            HtmlContent = htmlContent
        };
    }

    public MailData CreateAccessCodeMail(User user, int accessCode)
    {
        string htmlContent = accessCodeMailTemplate.Replace(
            "|--ACCESSCODE--|",
            Convert.ToString(accessCode)
        );
        return new MailData()
        {
            ReceiverName = user.Login,
            ReceiverEmail = user.Email,
            Subject = "Access code for Password!",
            HtmlContent = htmlContent
        };
    }
}
