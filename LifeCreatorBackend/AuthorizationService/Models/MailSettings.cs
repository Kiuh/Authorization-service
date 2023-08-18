namespace AuthorizationService.Models;

public class MailSettings
{
    public required string ApiKey { get; set; }
    public required string SenderName { get; set; }
    public required string SenderEmail { get; set; }
}
