using Microsoft.Extensions.Options;
using sib_api_v3_sdk.Api;
using sib_api_v3_sdk.Client;
using sib_api_v3_sdk.Model;

namespace AuthorizationService.Services.Mail;

public class MailSenderSettings
{
    public required string ApiKey { get; set; }
    public required string SenderName { get; set; }
    public required string SenderEmail { get; set; }
}

public class MailData
{
    public required string ReceiverName { get; set; }
    public required string ReceiverEmail { get; set; }
    public required string Subject { get; set; }
    public required string HtmlContent { get; set; }
}

public interface IMailSenderService
{
    public System.Threading.Tasks.Task SendAsync(MailData mailData);
}

/// <summary>
/// Visit https://app.brevo.com/ to see data about sent emails
/// </summary>
public class MailSenderService : IMailSenderService
{
    private readonly MailSenderSettings mailSettings;

    public MailSenderService(IOptions<MailSenderSettings> mailSettings)
    {
        this.mailSettings = mailSettings.Value;
    }

    public async System.Threading.Tasks.Task SendAsync(MailData mailData)
    {
        TransactionalEmailsApi api = new();
        api.Configuration.AddApiKey("api-key", mailSettings.ApiKey);
        try
        {
            SendSmtpEmail email =
                new()
                {
                    To = new List<SendSmtpEmailTo>
                    {
                        new SendSmtpEmailTo(mailData.ReceiverEmail, mailData.ReceiverName)
                    },
                    Sender = new SendSmtpEmailSender(
                        name: mailSettings.SenderName,
                        email: mailSettings.SenderEmail
                    ),
                    Subject = mailData.Subject,
                    HtmlContent = mailData.HtmlContent
                };
            ApiResponse<CreateSmtpEmail> result = await api.SendTransacEmailAsyncWithHttpInfo(
                email
            );

        }
        catch
        {
            throw new ApiException(500, "Fail to send mail.");
        }
    }
}
