namespace AuthorizationService.Pages;

public class ErrorPageInfo
{
    public required string StatusCode { get; set; }
    public required string Title { get; set; }
    public required List<string> Labels { get; set; }
}
