using AuthorizationService.Common;
using System.Text.Json;

namespace AuthorizationService.Middleware;

public class ErrorBody
{
    public string Error { get; set; } = string.Empty;

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
        catch (Exception ex) when ((string)context.Request.Path is not "/ErrorPage")
        {
            HttpResponse response = context.Response;
            response.ContentType = "application/json";
            response.StatusCode = 500;
            ErrorBody errorBody = new(response.StatusCode + ": Unknown internal error.");
            if (ex is ApiException apiEx)
            {
                response.StatusCode = apiEx.StatusCode;
                errorBody = new ErrorBody(apiEx.StatusCode + ": " + apiEx.Message);
            }
            await response.WriteAsync(JsonSerializer.Serialize(errorBody));
        }
    }
}
