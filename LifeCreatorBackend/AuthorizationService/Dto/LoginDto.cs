namespace AuthorizationService.Dto;

public sealed class LoginDto
{
    public required string Signature { get; set; }
    public required string Nonce { get; set; }
}
