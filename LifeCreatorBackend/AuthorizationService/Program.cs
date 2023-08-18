using AuthorizationService.Data;
using AuthorizationService.Models;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.EntityFrameworkCore;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

_ = builder.Services.AddDbContext<AuthorizationDbContext>(
    options =>
        options.UseNpgsql(builder.Configuration.GetConnectionString("AuthorizationDbContext"))
);

builder.Services.Configure<MailSettings>(builder.Configuration.GetSection("MailSettings"));
builder.Services.AddTransient<IMailService, MailService>();

_ = builder.Services.AddControllers();
_ = builder.Services.AddSwaggerGen();

_ = builder.Services.AddAuthorization();
_ = builder.Services
    .AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(JwtToken.ConfigurateJwtBearerOptions);

_ = builder.Services.AddSingleton<IDbInitializeService, DbInitializeService>();

WebApplication app = builder.Build();

if (app.Environment.EnvironmentName is "DockerDevelopment" or "DesktopDevelopment")
{
    _ = app.UseSwagger();
    _ = app.UseSwaggerUI();
}

_ = app.UseAuthentication();
_ = app.UseAuthorization();

_ = app.MapControllers();

IDbInitializeService initService = app.Services.GetRequiredService<IDbInitializeService>();
if (app.Environment.EnvironmentName is "DockerDevelopment" or "DesktopDevelopment")
{
    initService.InitializeDb();
}
else if (app.Environment.EnvironmentName is "Production")
{
    initService.MigrateDb();
}

app.Run();
