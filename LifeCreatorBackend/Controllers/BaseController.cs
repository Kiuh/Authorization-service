using LifeCreatorBackend.Data;
using LifeCreatorBackend.Models;
using Microsoft.AspNetCore.Mvc;
using System.Text.Json;

namespace LifeCreatorBackend.Controllers;

[ApiController]
[Route("api")]
public sealed class BaseController : ControllerBase
{
    private readonly ApplicationDbContext appDbContext;

    public BaseController(ApplicationDbContext appDbContext)
    {
        this.appDbContext = appDbContext;
    }

    [HttpGet("/PublicKey")]
    public IActionResult GetPublicKey()
    {
        return Ok(Cryptography.PublicKey);
    }

    [HttpGet("/UsersTable")]
    public IActionResult GetUsersTable()
    {
        return appDbContext.Users is null
            ? Problem("Internal error")
            : (IActionResult)Ok(JsonSerializer.Serialize(appDbContext.Users));
    }
}
