using AuthorizationService.Data;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.EntityFrameworkCore;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.Configuration.AddUserSecrets<Program>();

string connectionStringName;

if (builder.Environment.EnvironmentName is "DockerDevelopment" or "Production")
{
    connectionStringName = "AuthorizationDbContextDocker";
}
else
{
    connectionStringName = builder.Environment.EnvironmentName is "DesktopDevelopment"
        ? "AuthorizationDbContextWindows"
        : throw new Exception("Unknown Environment");
}

_ = builder.Services.AddDbContext<AuthorizationDbContext>(
    options => options.UseNpgsql(builder.Configuration.GetConnectionString(connectionStringName))
);

builder.Services.Configure<MailServiceSettings>(builder.Configuration.GetSection("MailSettings"));
builder.Services.AddTransient<IMailService, MailService>();

_ = builder.Services.AddControllers();
_ = builder.Services.AddSwaggerGen();

builder.Services.Configure<TokensLifeTimeSettings>(
    builder.Configuration.GetSection("TokensLifeTimeSettings")
);

IConfigurationSection jwtTokenServiceSettingsConfig = builder.Configuration.GetSection(
    "JwtTokenServiceSettings"
);

builder.Services.Configure<JwtTokenServiceSettings>(jwtTokenServiceSettingsConfig);

JwtTokenServiceSettings? jwtTokenServiceSettings =
    jwtTokenServiceSettingsConfig.Get<JwtTokenServiceSettings>()
    ?? throw new Exception("No JwtTokenServiceSettings");

_ = builder.Services.AddTransient<IJwtTokenService, JwtTokenService>();

_ = builder.Services.AddAuthorization();
_ = builder.Services
    .AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(jwtTokenServiceSettings.ConfigurateJwtBearerOptions);

_ = builder.Services.AddSingleton<IDbInitializeService, DbInitializeService>();

builder.Services.Configure<CryptographyServiceSettings>(
    builder.Configuration.GetSection("CryptographySettings")
);
_ = builder.Services.AddTransient<ICryptographyService, CryptographyService>();

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
