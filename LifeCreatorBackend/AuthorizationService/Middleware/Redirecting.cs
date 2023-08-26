using AuthorizationService.Services;
using Microsoft.Extensions.Primitives;

namespace AuthorizationService.Middleware;

public class RedirectingMiddleware
{
    private readonly RequestDelegate next;
    private readonly IJwtTokenToolsService jwtTokenTools;
    private readonly IHttpClientFactory httpClientFactory;

    public RedirectingMiddleware(
        RequestDelegate next,
        IJwtTokenToolsService jwtTokenTools,
        IHttpClientFactory httpClientFactory
    )
    {
        this.next = next;
        this.jwtTokenTools = jwtTokenTools;
        this.httpClientFactory = httpClientFactory;
    }

    public async Task InvokeAsync(HttpContext context)
    {
        if (
            context.Request.Headers.TryGetValue("JwtToken", out StringValues token)
            && jwtTokenTools.ValidateToken(token.ToString()).Success
        )
        {
            HttpRequestMessage httpRequestMessage =
                new(HttpMethod.Get, "http://localhost:5132/Core");

            HttpClient httpClient = httpClientFactory.CreateClient();
            HttpResponseMessage httpResponseMessage = await httpClient.SendAsync(
                httpRequestMessage
            );

            await context.Response.WriteAsync(
                httpResponseMessage.Content.ReadAsStringAsync().Result
            );
        }
        else
        {
            context.Response.StatusCode = 403;
            await context.Response.WriteAsync("Token is invalid");
        }
    }
}
