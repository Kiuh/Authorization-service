using AuthorizationService.Common;
using AuthorizationService.Data;
using AuthorizationService.Models;
using Microsoft.AspNetCore.Mvc;
using System.ComponentModel.DataAnnotations;
using System.IdentityModel.Tokens.Jwt;
using System.Text.Json;

namespace AuthorizationService.Controllers;

[ApiController]
[Route("[controller]")]
public sealed class UsersController : ControllerBase
{
    private readonly AuthorizationDbContext authorizationDbContext;

    public UsersController(AuthorizationDbContext dbContext)
    {
        authorizationDbContext = dbContext;
    }

    //Signature: SHA256(Login + Nonce + HashedPassword)
    [HttpPost("/Users/LogIn")]
    public IActionResult Login([FromBody] JsonElement bodyJson)
    {
        LogInData? logInData = bodyJson.Deserialize<LogInData>();
        if (logInData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }
        List<User> users = authorizationDbContext.Users.ToList();
        User? user = users.Find(
            x => (x.Login + logInData.Nonce + x.HashedPassword).GetHash() == logInData.Signature
        );
        if (user is null)
        {
            return NotFound("User is not exist.");
        }
        else
        {
            JwtSecurityToken jwtToken = JwtToken.GetJwtSecurityToken(user.Login);
            Response.Headers.Add(
                "JwtBearerToken",
                new JwtSecurityTokenHandler().WriteToken(jwtToken)
            );
            return Accepted();
        }
    }

    [Serializable]
    public sealed class LogInData
    {
        public required string Signature { get; set; }
        public required string Nonce { get; set; }
    }

    [HttpPost]
    public IActionResult Registration([FromBody] JsonElement bodyJson)
    {
        RegistrationData? registrationData = bodyJson.Deserialize<RegistrationData>();
        if (registrationData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }
        if (authorizationDbContext.Users is null)
        {
            return Problem("Internal error");
        }
        List<User> users = authorizationDbContext.Users.ToList();
        if (users.Any(x => x.Login == registrationData.Login))
        {
            return BadRequest("This login is already taken.");
        }

        string email = registrationData.EncryptedNonceWithEmail
            .GetDecrypted(Cryptography.CryptoService)
            .Replace(registrationData.Nonce, "");

        if (!new EmailAddressAttribute().IsValid(email))
        {
            return BadRequest("Invalid email.");
        }
        if (users.Any(x => x.Email == email))
        {
            return BadRequest("This email is already in use.");
        }
        User newUser =
            new()
            {
                Login = registrationData.Login,
                Email = email,
                HashedPassword = registrationData.EncryptedHashedPassword.GetDecrypted(
                    Cryptography.CryptoService
                )
            };

        _ = authorizationDbContext.Users.Add(newUser);
        _ = authorizationDbContext.SaveChanges();
        return Ok(
            $"Added and saved user: {JsonSerializer.Serialize(authorizationDbContext.Users.ToList())}"
        );
    }

    [Serializable]
    private sealed class RegistrationData
    {
        public required string Login { get; set; }
        public required string EncryptedNonceWithEmail { get; set; }
        public required string Nonce { get; set; }
        public required string EncryptedHashedPassword { get; set; }
    }
}
