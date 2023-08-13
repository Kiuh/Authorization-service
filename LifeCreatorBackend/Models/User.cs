namespace LifeCreatorBackend.Models;

public sealed class User
{
    public int Id { get; set; }
    public string? Login { get; set; }
    public string? Email { get; set; }
    public string? HashedPassword { get; set; }
}
