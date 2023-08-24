namespace AuthorizationService.Models;

public sealed class EmailVerification : EntityBase
{
    public long UserId { get; set; }
    public required User User { get; set; }
    public string JwtToken { get; set; } = "";
    public DateTime RequestDate { get; set; }
}
