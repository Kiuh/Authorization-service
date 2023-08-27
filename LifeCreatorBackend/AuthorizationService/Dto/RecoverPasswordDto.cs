namespace AuthorizationService.Dto;

public sealed class RecoverPasswordDto
{
    public required int AccessCode { get; set; }
    public required string EncryptedHashedPassword { get; set; }
}
