using LifeCreatorBackend.Data;
using LifeCreatorBackend.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Infrastructure;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

Cryptography.GenerateKeyPair();

builder.Services.AddDbContext<ApplicationDbContext>(
    options => options.UseSqlServer(builder.Configuration.GetConnectionString("CoreDb"))
);

builder.Services.AddControllers();

builder.Services.AddEndpointsApiExplorer();

builder.Services.AddSwaggerGen();

WebApplication app = builder.Build();

using (
    IServiceScope serviceScope = app.Services
        .GetRequiredService<IServiceScopeFactory>()
        .CreateScope()
)
{
    ILogger<Program> logger = serviceScope.ServiceProvider.GetRequiredService<ILogger<Program>>();
    DatabaseFacade db = serviceScope.ServiceProvider
        .GetRequiredService<ApplicationDbContext>()
        .Database;

    logger.LogInformation("Migrating database...");

    while (!db.CanConnect())
    {
        logger.LogInformation("Database not ready yet; waiting...");
        Thread.Sleep(1000);
    }

    try
    {
        serviceScope.ServiceProvider.GetRequiredService<ApplicationDbContext>().Database.Migrate();
        logger.LogInformation("Database migrated successfully.");
    }
    catch (Exception ex)
    {
        logger.LogError(ex, "An error occurred while migrating the database.");
    }
}

if (app.Environment.IsDevelopment())
{
    _ = app.UseSwagger();
    _ = app.UseSwaggerUI(c => c.SwaggerEndpoint("/swagger/v1/swagger.json", "Life Creator v1"));
}

app.UseHttpsRedirection();

app.UseAuthorization();

app.MapControllers();

app.Run();
