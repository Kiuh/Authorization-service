using AuthorizationService.Data;
using AuthorizationService.Dto;
using AuthorizationService.Models;
using Microsoft.EntityFrameworkCore;
using System.ComponentModel.DataAnnotations;

namespace AuthorizationService.Services.Models;

public interface IUsersService
{
    public User CreateUserFromRegistration(RegistrationDto registrationDto);
    public Task<User> FindUserBySignatureAndNonce(string signature, string nonce);
    public Task<User> FindUserByEmailVerification(EmailVerification emailVerification);
    public Task<User> FindUserByEncryptedNonceWithEmail(
        string encryptedNonceWithEmail,
        string nonce
    );
    public Task AddNewUser(User user);
    public Task SetNewUserPassword(User user, string encryptedHashedPassword);
}

public class UsersService : IUsersService
{
    private readonly AuthorizationDbContext authorizationDbContext;
    private readonly ICryptographyService cryptographyService;

    public UsersService(
        ICryptographyService cryptographyService,
        AuthorizationDbContext authorizationDbContext
    )
    {
        this.cryptographyService = cryptographyService;
        this.authorizationDbContext = authorizationDbContext;
    }

    public async Task AddNewUser(User user)
    {
        if (authorizationDbContext.Users.Any(x => x.Login == user.Login))
        {
            throw new Exception("This login is already taken.");
        }
        if (!new EmailAddressAttribute().IsValid(user))
        {
            throw new Exception("Invalid email.");
        }
        if (authorizationDbContext.Users.Any(x => x.Email == user.Email))
        {
            throw new Exception("This email is already in use.");
        }

        _ = await authorizationDbContext.Users.AddAsync(user);
        _ = await authorizationDbContext.SaveChangesAsync();
    }

    public User CreateUserFromRegistration(RegistrationDto registrationDto)
    {
        string email = cryptographyService
            .DecryptString(registrationDto.EncryptedNonceWithEmail)
            .Replace(registrationDto.Nonce, "");

        return new()
        {
            Login = registrationDto.Login,
            Email = email,
            HashedPassword = cryptographyService.DecryptString(
                registrationDto.EncryptedHashedPassword
            ),
            RegistrationDate = DateTime.UtcNow,
            EmailVerification = EmailVerificationState.NotVerified
        };
    }

    public async Task<User> FindUserBySignatureAndNonce(string signature, string nonce)
    {
        User? foundUser = null;
        foreach (User? user in await authorizationDbContext.Users.ToListAsync())
        {
            if (
                cryptographyService.HashString(user.Login + nonce + user.HashedPassword)
                == signature
            )
            {
                foundUser = user;
                break;
            }
        }
        return foundUser is null ? throw new Exception("User not Found") : foundUser;
    }

    public async Task<User> FindUserByEmailVerification(EmailVerification emailVerification)
    {
        return await authorizationDbContext.Users.FirstOrDefaultAsync(
                x => x.EmailVerifications.Contains(emailVerification)
            ) ?? throw new Exception("User not found");
    }

    public async Task<User> FindUserByEncryptedNonceWithEmail(
        string encryptedNonceWithEmail,
        string nonce
    )
    {
        string email = cryptographyService
            .DecryptString(encryptedNonceWithEmail)
            .Replace(nonce, "");

        return await authorizationDbContext.Users.FirstOrDefaultAsync(x => x.Email == email)
            ?? throw new Exception("User with this email is not exist.");
    }

    public async Task SetNewUserPassword(User user, string encryptedHashedPassword)
    {
        user.HashedPassword = cryptographyService.DecryptString(encryptedHashedPassword);
        _ = await authorizationDbContext.SaveChangesAsync();
    }
}
