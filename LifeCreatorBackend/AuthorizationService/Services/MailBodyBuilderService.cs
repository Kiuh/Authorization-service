namespace AuthorizationService.Services;

public interface IMailBodyBuilder
{
    public MailData WelcomeMail(string login, string email);
    public MailData VerificationMail(string login, string email, string jwtToken);
    public MailData AccessCodeMail(string login, string email, int accessCode);
}

public class MailBodyBuilderService : IMailBodyBuilder
{
    private string welcomeMailTemplate;
    private string verificationMailTemplate;
    private string accessCodeMailTemplate;

    public MailBodyBuilderService()
    {
        string basePath = "./Views/EmailBodyPrototypes/";
        welcomeMailTemplate = File.ReadAllText(basePath + "Welcome.html");
        verificationMailTemplate = File.ReadAllText(basePath + "Verification.html");
        accessCodeMailTemplate = File.ReadAllText(basePath + "AccessCode.html");
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
        string htmlContent = verificationMailTemplate;
        _ = htmlContent.Replace("|--JWTTOKEN--|", jwtToken);
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
        string htmlContent = accessCodeMailTemplate;
        _ = htmlContent.Replace("|--ACCESSCODE--|", Convert.ToString(accessCode));
        return new MailData()
        {
            ReceiverName = login,
            ReceiverEmail = email,
            Subject = "Access code for Password!",
            HtmlContent = htmlContent
        };
    }
}
