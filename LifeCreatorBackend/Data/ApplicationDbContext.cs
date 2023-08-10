using LifeCreatorBackend.Models;
using Microsoft.EntityFrameworkCore;

namespace LifeCreatorBackend.Data;

public class ApplicationDbContext : DbContext
{
    public ApplicationDbContext(DbContextOptions<ApplicationDbContext> options)
        : base(options) { }

    public DbSet<User>? Users { get; set; }
}
