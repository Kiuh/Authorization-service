namespace AuthorizationService.Models;

public class MailData
{
    public required string ReceiverName { get; set; }
    public required string ReceiverEmail { get; set; }
    public required string Subject { get; set; }
    public required string HtmlContent { get; set; }
}
