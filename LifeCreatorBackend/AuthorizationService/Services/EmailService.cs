using AuthorizationService.Common;
using AuthorizationService.Models;
using Microsoft.Extensions.Options;
using sib_api_v3_sdk.Api;
using sib_api_v3_sdk.Client;
using sib_api_v3_sdk.Model;

namespace AuthorizationService.Services;

public interface IMailService
{
    Task<Result> SendAsync(MailData mailData);
}

// Visit https://app.brevo.com/ to see data about sent emails
public class MailService : IMailService
{
    private readonly MailSettings mailSettings;

    public MailService(IOptions<MailSettings> mailSettings)
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
