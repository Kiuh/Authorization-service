using AuthorizationService.Data;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Mvc;
using System.Text.Json;

namespace AuthorizationService.Controllers;

[Route("[api]")]
public sealed class CoreController : ControllerBase
{
    private readonly AuthorizationDbContext authorizationDbContext;
    private readonly ICryptographyService cryptographyService;

    public CoreController(
        AuthorizationDbContext dbContext,
        ICryptographyService cryptographyService
    )
    {
        authorizationDbContext = dbContext;
        this.cryptographyService = cryptographyService;
    }

    [HttpGet("/PublicKey")]
    public IActionResult GetPublicKey()
    {
        return Ok(cryptographyService.GetPublicKey());
    }

    [HttpGet("/UsersTable")]
    public IActionResult GetUsersTable()
    {
        return Ok(JsonSerializer.Serialize(authorizationDbContext.Users.ToList()));
    }

    [HttpGet("/EmailVerifications")]
    public IActionResult GetEmailVerificationsTable()
    {
        return Ok(JsonSerializer.Serialize(authorizationDbContext.EmailVerifications.ToList()));
    }

    [HttpGet("/PasswordRecovers")]
    public IActionResult GetPasswordRecoversTable()
    {
        return Ok(JsonSerializer.Serialize(authorizationDbContext.PasswordRecovers.ToList()));
    }
}
