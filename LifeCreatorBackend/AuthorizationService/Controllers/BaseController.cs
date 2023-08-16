using AuthorizationService.Data;
using AuthorizationService.Models;
using Microsoft.AspNetCore.Mvc;
using System.Text.Json;

namespace AuthorizationService.Controllers;

[Route("api")]
public sealed class BaseController : ControllerBase
{
    private readonly AuthorizationDbContext authorizationDbContext;

    public BaseController(AuthorizationDbContext dbContext)
    {
        authorizationDbContext = dbContext;
    }

    [HttpGet("/PublicKey")]
    public IActionResult GetPublicKey()
    {
        return Ok(Cryptography.PublicKey);
    }

    [HttpGet("/UsersTable")]
    public IActionResult GetUsersTable()
    {
        return Ok(JsonSerializer.Serialize(authorizationDbContext.Users.ToList()));
    }
}
