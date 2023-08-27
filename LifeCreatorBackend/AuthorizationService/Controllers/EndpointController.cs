using AuthorizationService.Common;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Options;
using Microsoft.Extensions.Primitives;

namespace AuthorizationService.Controllers;

public class RedirectionSettings
{
    public required string CoreApiPath { get; set; }
}

[ApiController]
[ApiExplorerSettings(IgnoreApi = true)]
public class EndpointController : Controller
{
    private IJwtTokenToolsService jwtTokenTools;
    private IHttpClientFactory httpClientFactory;
    private ILogger<EndpointController> logger;
    private RedirectionSettings redirectionInfo;

    public EndpointController(
        IJwtTokenToolsService jwtTokenTools,
        IHttpClientFactory httpClientFactory,
        ILogger<EndpointController> logger,
        IOptions<RedirectionSettings> redirectionInfo
    )
    {
        this.redirectionInfo = redirectionInfo.Value;
        this.jwtTokenTools = jwtTokenTools;
        this.httpClientFactory = httpClientFactory;
        this.logger = logger;
    }

    [Route("{*url}")]
    public async Task<IActionResult> Redirect()
    {
        logger.LogDefaultInfo(Request);
        if (
            (
                Request.Headers.TryGetValue("JwtToken", out StringValues token)
                && jwtTokenTools.ValidateToken(token.ToString()).Success
            ) || true
        )
        {
            HttpRequestMessage httpRequestMessage =
                new(HttpMethod.Get, redirectionInfo.CoreApiPath);

            HttpClient httpClient = httpClientFactory.CreateClient();
            HttpResponseMessage httpResponseMessage = await httpClient.SendAsync(
                httpRequestMessage
            );

            return Ok(httpResponseMessage.Content.ReadAsStream());
        }
        else
        {
            return Unauthorized();
        }
    }
}
