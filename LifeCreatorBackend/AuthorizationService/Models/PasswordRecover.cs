namespace AuthorizationService.Models;

public sealed class PasswordRecover : EntityBase
{
    public long UserId { get; set; }
    public required User User { get; set; }
    public int AccessCode { get; set; } = -1;
    public DateTime RequestDate { get; set; }
}
