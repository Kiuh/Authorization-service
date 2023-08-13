using LifeCreatorBackend.Common;
using LifeCreatorBackend.Data;
using LifeCreatorBackend.Models;
using Microsoft.AspNetCore.Mvc;
using System.ComponentModel.DataAnnotations;
using System.Text.Json;

namespace LifeCreatorBackend.Controllers;

[ApiController]
[Route("[controller]")]
public sealed class UsersController : ControllerBase
{
    private readonly ApplicationDbContext appDbContext;

    public UsersController(ApplicationDbContext appDbContext)
    {
        this.appDbContext = appDbContext;
    }

    //Signature: SHA256(Login + Nonce + HashedPassword)
    [HttpPost("/LogIn")]
    public IActionResult Login([FromHeader] string Nonce, [FromBody] JsonElement bodyJson)
    {
        LogInData? logInData = JsonSerializer.Deserialize<LogInData>(bodyJson);
        if (logInData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }
        if (appDbContext.Users is null)
        {
            return Problem("Internal error");
        }
        User? user = appDbContext.Users
            ?.ToList()
            .Find(x => (x.Login + Nonce + x.HashedPassword).GetHash() == logInData.Signature);
        return user is null ? NotFound() : Accepted();
    }

    [Serializable]
    public sealed class LogInData
    {
        public required string Signature { get; set; }
    }

    [HttpPost]
    public IActionResult Registration([FromBody] JsonElement bodyJson)
    {
        RegistrationData? registrationData = JsonSerializer.Deserialize<RegistrationData>(bodyJson);
        if (registrationData is null)
        {
            return BadRequest("Cannot deserialize body.");
        }
        if (appDbContext.Users is null)
        {
            return Problem("Internal error");
        }
        List<User> users = appDbContext.Users.ToList();
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
        _ = appDbContext.Users.Add(newUser);
        _ = appDbContext.SaveChanges();
        return Accepted();
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
