using AuthorizationService.Models;
using Microsoft.EntityFrameworkCore;

namespace AuthorizationService.Data;

public class AuthorizationDbContext : DbContext
{
    public DbSet<User> Users { get; set; } = null!;
    public DbSet<PasswordRecover> PasswordRecovers { get; set; } = null!;
    public DbSet<EmailVerification> EmailVerifications { get; set; } = null!;

    public AuthorizationDbContext(DbContextOptions<AuthorizationDbContext> options)
        : base(options) { }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        _ = modelBuilder
            .Entity<User>()
            .HasMany(e => e.PasswordRecovers)
            .WithOne(e => e.User)
            .HasForeignKey(e => e.UserId)
            .IsRequired();
        _ = modelBuilder
            .Entity<User>()
            .HasMany(e => e.EmailVerifications)
            .WithOne(e => e.User)
            .HasForeignKey(e => e.UserId)
            .IsRequired();
    }
}
