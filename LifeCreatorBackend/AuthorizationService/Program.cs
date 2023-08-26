using AuthorizationService.Controllers;
using AuthorizationService.Data;
using AuthorizationService.Services;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.EntityFrameworkCore;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.Configuration.AddUserSecrets<Program>();

string connectionStringSection;
string mailBodyBuilderSettingsSection;
string redirectionSettingsSection;

if (builder.Environment.EnvironmentName is "DockerDevelopment" or "Production")
{
    connectionStringSection = "AuthorizationDbContextDocker";
    mailBodyBuilderSettingsSection = "MailBodyBuilderSettingsDocker";
    redirectionSettingsSection = "RedirectionSettingsDocker";
}
else
{
    if (builder.Environment.EnvironmentName is not "DesktopDevelopment")
    {
        throw new Exception("Unknown Environment");
    }
    else
    {
        connectionStringSection = "AuthorizationDbContextWindows";
        mailBodyBuilderSettingsSection = "MailBodyBuilderSettingsDesktop";
        redirectionSettingsSection = "RedirectionSettingsDesktop";
    }
}

builder.Services.AddDbContext<AuthorizationDbContext>(
    options => options.UseNpgsql(builder.Configuration.GetConnectionString(connectionStringSection))
);

builder.Services.Configure<MailSenderSettings>(builder.Configuration.GetSection("MailSettings"));
builder.Services.AddTransient<IMailSenderService, MailSender>();

builder.Services.AddSwaggerGen();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddControllers();

builder.Services.Configure<TokensLifeTimeSettings>(
    builder.Configuration.GetSection("TokensLifeTimeSettings")
);

IConfigurationSection jwtTokenServiceSettingsConfig = builder.Configuration.GetSection(
    "JwtTokenServiceSettings"
);

builder.Services.Configure<JwtTokenToolsSettings>(jwtTokenServiceSettingsConfig);

JwtTokenToolsSettings? jwtTokenServiceSettings =
    jwtTokenServiceSettingsConfig.Get<JwtTokenToolsSettings>()
    ?? throw new Exception("No JwtTokenServiceSettings");

builder.Services.AddTransient<IJwtTokenToolsService, JwtTokenTools>();

builder.Services.AddAuthorization();
builder.Services
    .AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(jwtTokenServiceSettings.ConfigurateJwtBearerOptions);

builder.Services.AddSingleton<IDbInitializeService, DbInitialize>();

builder.Services.Configure<CryptographyServiceSettings>(
    builder.Configuration.GetSection("CryptographySettings")
);

builder.Services.AddTransient<ICryptographyService, Cryptography>();

builder.Services.Configure<MailBodyBuilderSettings>(
    builder.Configuration.GetSection(mailBodyBuilderSettingsSection)
);
builder.Services.Configure<RedirectionSettings>(
    builder.Configuration.GetSection(redirectionSettingsSection)
);
builder.Services.AddTransient<IMailBodyBuilder, MailBodyBuilder>();

builder.Services.AddRazorPages();

builder.Services.AddHttpClient();

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
app.MapRazorPages();

app.Run();
