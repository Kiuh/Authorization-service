using AuthorizationService.Common;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Mvc;

namespace AuthorizationService.Controllers;

public class PublicKeyController : Controller
{
    private readonly ICryptographyService cryptographyService;
    private readonly ILogger<PublicKeyController> logger;

    public PublicKeyController(
        ICryptographyService cryptographyService,
        ILogger<PublicKeyController> logger
    )
    {
        this.cryptographyService = cryptographyService;
        this.logger = logger;
    }

    [HttpGet("/PublicKey")]
    public IActionResult GetPublicKey()
    {
        logger.LogDefaultInfo(Request);
        return Ok(cryptographyService.GetPublicKey());
    }
}
