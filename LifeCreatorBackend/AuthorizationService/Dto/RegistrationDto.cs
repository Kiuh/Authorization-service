namespace AuthorizationService.Dto;

public sealed class RegistrationDto
{
    public required string Login { get; set; }
    public required string EncryptedNonceWithEmail { get; set; }
    public required string Nonce { get; set; }
    public required string EncryptedHashedPassword { get; set; }
}
