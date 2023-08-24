using Microsoft.AspNetCore.Mvc.RazorPages;
using System.Text.Json;

namespace AuthorizationService.Views.ErrorPage;

public class ErrorPageInfo
{
    public string StatusCodeNumber { get; set; }
    public string Title { get; set; }
    public List<string> Labels { get; set; }

    public ErrorPageInfo(string statusCodeNumber, string title, params string[] labels)
    {
        StatusCodeNumber = statusCodeNumber;
        Title = title;
        Labels = labels.ToList();
    }
}

public static class ErrorPageInfoTools
{
    public static string ToJson(this ErrorPageInfo errorPageInfo)
    {
        return JsonSerializer.Serialize(errorPageInfo);
    }
}

public class Model : PageModel
{
    public string StatusCodeNumber { get; set; } = "";
    public string Title { get; set; } = "";
    public List<string> Labels { get; set; } = new();

    public void OnGet(string pageInfo)
    {
        ErrorPageInfo? errorPageInfo = JsonSerializer.Deserialize<ErrorPageInfo>(pageInfo);
        if (errorPageInfo is not null)
        {
            StatusCodeNumber = errorPageInfo.StatusCodeNumber;
            Title = errorPageInfo.Title;
            Labels = errorPageInfo.Labels;
        }
        else
        {
            StatusCodeNumber = "404";
            Title = "Not Found";
            Labels = new List<string>() { "In real deserialization error." };
        }
    }
}
