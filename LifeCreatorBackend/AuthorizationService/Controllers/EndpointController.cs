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
    private readonly IJwtTokenToolsService jwtTokenToolsService;
    private readonly IHttpClientFactory httpClientFactory;
    private readonly ILogger<EndpointController> logger;
    private readonly RedirectionSettings redirectionSettings;

    public EndpointController(
        IJwtTokenToolsService jwtTokenToolsService,
        IHttpClientFactory httpClientFactory,
        ILogger<EndpointController> logger,
        IOptions<RedirectionSettings> redirectionSettings
    )
    {
        this.redirectionSettings = redirectionSettings.Value;
        this.jwtTokenToolsService = jwtTokenToolsService;
        this.httpClientFactory = httpClientFactory;
        this.logger = logger;
    }

    [Route("{*url}")]
    public async Task<IActionResult> Redirect()
    {
        logger.LogDefaultInfo(Request);
        if (
            Request.Headers.TryGetValue("JwtToken", out StringValues token)
            && jwtTokenToolsService.ValidateToken(token.ToString())
        )
        {
            HttpRequestMessage httpRequestMessage =
                new(HttpMethod.Get, redirectionSettings.CoreApiPath);

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
