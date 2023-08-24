namespace AuthorizationService.Models;

public sealed class PasswordRecover : EntityBase
{
    public long UserId { get; set; }
    public required User User { get; set; }
    public int AccessCode { get; set; } = -1;
    public DateTime RequestDate { get; set; }
}

public static class PasswordRecoverTools
{
    public static bool IsValid(this PasswordRecover passwordRecover, TimeSpan duration)
    {
        return DateTime.UtcNow.Subtract(passwordRecover.RequestDate) <= duration;
    }
}
