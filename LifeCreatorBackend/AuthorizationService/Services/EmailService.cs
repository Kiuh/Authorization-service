using AuthorizationService.Common;
using Microsoft.Extensions.Options;
using sib_api_v3_sdk.Api;
using sib_api_v3_sdk.Client;
using sib_api_v3_sdk.Model;

namespace AuthorizationService.Services;

public class MailServiceSettings
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

public interface IMailService
{
    Task<Result> SendAsync(MailData mailData);
}

/// <summary>
/// Visit https://app.brevo.com/ to see data about sent emails
/// </summary>
public class MailService : IMailService
{
    private readonly MailServiceSettings mailSettings;

    public MailService(IOptions<MailServiceSettings> mailSettings)
    {
        this.mailSettings = mailSettings.Value;
    }

    public async Task<Result> SendAsync(MailData mailData)
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

            return new SuccessResult();
        }
        catch (Exception e)
        {
            return new FailResult(e.Message);
        }
    }
}
