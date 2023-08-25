using Microsoft.Extensions.Options;

namespace AuthorizationService.Services;

public class MailBodyBuilderSettings
{
    public required string VerificationLink { get; set; }
}

public interface IMailBodyBuilder
{
    public MailData WelcomeMail(string login, string email);
    public MailData VerificationMail(string login, string email, string jwtToken);
    public MailData AccessCodeMail(string login, string email, int accessCode);
}

public class MailBodyBuilder : IMailBodyBuilder
{
    private string welcomeMailTemplate;
    private string verificationMailTemplate;
    private string accessCodeMailTemplate;
    private string verificationLink;

    public MailBodyBuilder(IOptions<MailBodyBuilderSettings> mailBodyBuilderSettings)
    {
        string basePath = "./Mails/";
        welcomeMailTemplate = File.ReadAllText(basePath + "Welcome.html");
        verificationMailTemplate = File.ReadAllText(basePath + "Verification.html");
        accessCodeMailTemplate = File.ReadAllText(basePath + "AccessCode.html");
        verificationLink = mailBodyBuilderSettings.Value.VerificationLink;
    }

    public MailData WelcomeMail(string login, string email)
    {
        return new MailData()
        {
            ReceiverName = login,
            ReceiverEmail = email,
            Subject = "Welcome to Life Creator!",
            HtmlContent = welcomeMailTemplate
        };
    }

    public MailData VerificationMail(string login, string email, string jwtToken)
    {
        string htmlContent = verificationMailTemplate.Replace("|--JWTTOKEN--|", jwtToken);
        htmlContent = htmlContent.Replace("|--LINK--|", verificationLink);
        return new MailData()
        {
            ReceiverName = login,
            ReceiverEmail = email,
            Subject = "Verify your email!",
            HtmlContent = htmlContent
        };
    }

    public MailData AccessCodeMail(string login, string email, int accessCode)
    {
        string htmlContent = accessCodeMailTemplate.Replace(
            "|--ACCESSCODE--|",
            Convert.ToString(accessCode)
        );
        return new MailData()
        {
            ReceiverName = login,
            ReceiverEmail = email,
            Subject = "Access code for Password!",
            HtmlContent = htmlContent
        };
    }
}
