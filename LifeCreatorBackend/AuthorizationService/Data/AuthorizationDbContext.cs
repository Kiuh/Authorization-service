using AuthorizationService.Models;
using Microsoft.EntityFrameworkCore;

namespace AuthorizationService.Data;

public class AuthorizationDbContext : DbContext
{
    public DbSet<User> Users { get; set; } = null!;

    public AuthorizationDbContext(DbContextOptions<AuthorizationDbContext> options)
        : base(options) { }
}
