using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace AuthorizationService.Models;

public sealed class User
{
    [Key]
    [DatabaseGenerated(DatabaseGeneratedOption.Identity)]
    public long Id { get; set; }
    public string Login { get; set; } = "";
    public string Email { get; set; } = "";
    public string HashedPassword { get; set; } = "";
}
