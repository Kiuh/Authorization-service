using System.Text.Json;

namespace AuthorizationService.Middleware;

public class ErrorBody
{
    public string Error { get; set; }

    public ErrorBody(string error)
    {
        Error = error;
    }
}

public class ErrorHandlingMiddleware
{
    private readonly RequestDelegate next;

    public ErrorHandlingMiddleware(RequestDelegate next)
    {
        this.next = next;
    }

    public async Task Invoke(HttpContext context)
    {
        try
        {
            await next.Invoke(context);
        }
        catch (Exception ex)
        {
            HttpResponse response = context.Response;
            response.StatusCode = 500;
            response.ContentType = "application/json";
            string json = JsonSerializer.Serialize(new ErrorBody(ex.Message));
            await response.WriteAsync(json);
        }
    }
}
