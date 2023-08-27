namespace AuthorizationService.Dto;

public sealed class ResendVerificationDto
{
    public required string EncryptedNonceWithEmail { get; set; }
    public required string Nonce { get; set; }
}
