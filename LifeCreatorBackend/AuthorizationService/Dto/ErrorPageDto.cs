namespace AuthorizationService.Dto;

public class ErrorPageDto
{
    public string StatusCode { get; set; } = "500";
    public string Title { get; set; } = "Internal Server Error";
    public List<string> Labels { get; set; } = new();

    public ErrorPageDto(string label)
    {
        Labels.Add(label);
    }
}
