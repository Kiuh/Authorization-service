using LifeCreatorBackend.DataBase.DBClasses;
using Microsoft.EntityFrameworkCore;

namespace LifeCreatorBackend.DataBase;

public class ApplicationContext : DbContext
{
    public DbSet<User> Users { get; set; } = null!;

    public ApplicationContext(DbContextOptions<ApplicationContext> options)
        : base(options)
    {
        _ = Database.EnsureCreated();
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        _ = modelBuilder
            .Entity<User>()
            .HasData(
                new User
                {
                    Id = 1,
                    Login = "Tom",
                    Email = "someone@some.com",
                    Password = "fff"
                },
                new User
                {
                    Id = 2,
                    Login = "Bob",
                    Email = "someone1@some.com",
                    Password = "ggg"
                }
            );
    }
}
