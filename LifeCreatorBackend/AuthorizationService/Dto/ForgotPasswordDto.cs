namespace AuthorizationService.Dto;

public sealed class ForgotPasswordDto
{
    public required string EncryptedNonceWithEmail { get; set; }
    public required string Nonce { get; set; }
}
