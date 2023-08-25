using Microsoft.EntityFrameworkCore;

namespace AuthorizationService.Models;

public enum EmailVerificationState
{
    Verified,
    NotVerified
}

[Index(nameof(Login), IsUnique = true)]
[Index(nameof(Email), IsUnique = true)]
public sealed class User : EntityBase
{
    public string Login { get; set; } = "";
    public string Email { get; set; } = "";
    public DateTime RegistrationDate { get; set; }
    public EmailVerificationState EmailVerification { get; set; } =
        EmailVerificationState.NotVerified;
    public string HashedPassword { get; set; } = "";
    public ICollection<PasswordRecover> PasswordRecovers { get; } = new List<PasswordRecover>();
    public ICollection<EmailVerification> EmailVerifications { get; } =
        new List<EmailVerification>();
}
