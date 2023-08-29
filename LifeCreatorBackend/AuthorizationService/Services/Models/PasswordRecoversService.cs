using AuthorizationService.Common;
using AuthorizationService.Data;
using AuthorizationService.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Options;

namespace AuthorizationService.Services.Models;

public interface IPasswordRecoversService
{
    public PasswordRecover CreatePasswordRecover(User user);
    public Task AddPasswordRecover(PasswordRecover passwordRecover);
    public Task<User> FindUserWithValidAccessCode(int accessCode);
    public bool IsValidPasswordRecover(PasswordRecover passwordRecover);
}

public class PasswordRecoversService : IPasswordRecoversService
{
    private readonly AuthorizationDbContext authorizationDbContext;
    private readonly TokensLifeTimeSettings tokensLifeTimeSettings;

    public PasswordRecoversService(
        AuthorizationDbContext authorizationDbContext,
        IOptions<TokensLifeTimeSettings> tokensLifeTimeSettings
    )
    {
        this.authorizationDbContext = authorizationDbContext;
        this.tokensLifeTimeSettings = tokensLifeTimeSettings.Value;
    }

    public async Task AddPasswordRecover(PasswordRecover passwordRecover)
    {
        _ = await authorizationDbContext.PasswordRecovers.AddAsync(passwordRecover);
        _ = await authorizationDbContext.SaveChangesAsync();
    }

    public PasswordRecover CreatePasswordRecover(User user)
    {
        return new()
        {
            User = user,
            AccessCode = new Random().Next(100001, 999999),
            RequestDate = DateTime.UtcNow
        };
    }

    public async Task<User> FindUserWithValidAccessCode(int accessCode)
    {
        IEnumerable<PasswordRecover> codes = await authorizationDbContext.PasswordRecovers
            .Where(x => x.AccessCode == accessCode)
            .ToListAsync();

        if (!codes.Any())
        {
            throw new ApiException(404, "No such Access code.");
        }

        codes = codes.Where(IsValidPasswordRecover);

        if (!codes.Any())
        {
            throw new ApiException(400, "Access code duration expired.");
        }
        else if (codes.Count() > 1)
        {
            throw new ApiException(500, "Internal error, try again.");
        }

        return authorizationDbContext.Users.FirstOrDefault(
                x => x.PasswordRecovers.Contains(codes.First())
            ) ?? throw new ApiException(500, "Internal error, try again.");
    }

    public bool IsValidPasswordRecover(PasswordRecover passwordRecover)
    {
        return DateTime.UtcNow.Subtract(passwordRecover.RequestDate)
            <= tokensLifeTimeSettings.AccessCodeDuration;
    }
}
