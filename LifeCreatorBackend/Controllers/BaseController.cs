using LifeCreatorBackend.Models;
using Microsoft.AspNetCore.Mvc;

namespace LifeCreatorBackend.Controllers;

[ApiController]
[Route("api")]
public sealed class BaseController : ControllerBase
{
    [HttpGet("/PublicKey")]
    public IActionResult GetPublicKey()
    {
        return Ok(Cryptography.PublicKey);
    }
}
