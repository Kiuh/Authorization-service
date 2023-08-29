using AuthorizationService.Common;
using AuthorizationService.Data;
using AuthorizationService.Models;
using Microsoft.EntityFrameworkCore;

namespace AuthorizationService.Services.Models;

public interface IEmailVerificationsService
{
    public Task AddEmailVerification(EmailVerification emailVerification);
    public Task<EmailVerification> FindEmailVerificationByJwtToken(string jwtToken);
    public Task VerifyUserEmail(User user);
}

public class EmailVerificationsService : IEmailVerificationsService
{
    private readonly AuthorizationDbContext authorizationDbContext;

    public EmailVerificationsService(AuthorizationDbContext authorizationDbContext)
    {
        this.authorizationDbContext = authorizationDbContext;
    }

    public async Task AddEmailVerification(EmailVerification emailVerification)
    {
        _ = await authorizationDbContext.EmailVerifications.AddAsync(emailVerification);
        _ = await authorizationDbContext.SaveChangesAsync();
    }

    public async Task<EmailVerification> FindEmailVerificationByJwtToken(string jwtToken)
    {
        EmailVerification? emailVerification =
            await authorizationDbContext.EmailVerifications.FirstOrDefaultAsync(
                x => x.JwtToken == jwtToken
            );
        return emailVerification is null
            ? throw new ApiException(404, "No such email verification request.")
            : emailVerification;
    }

    public async Task VerifyUserEmail(User user)
    {
        user.EmailVerification = EmailVerificationState.Verified;
        _ = await authorizationDbContext.SaveChangesAsync();
    }
}
