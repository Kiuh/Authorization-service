using AuthorizationService.Common;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.Extensions.Options;
using Microsoft.IdentityModel.Tokens;
using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using System.Text;

namespace AuthorizationService.Services;

public class TokensLifeTimeSettings
{
    public required TimeSpan LoginTokenDuration { get; set; }
    public required TimeSpan EmailValidationTokenDuration { get; set; }
    public required TimeSpan AccessCodeDuration { get; set; }
}

public class JwtTokenToolsSettings
{
    public required string Issuer { get; set; }
    public required string Audience { get; set; }
    public required string Key { get; set; }

    public void ConfigurateJwtBearerOptions(JwtBearerOptions jwtBearerOptions)
    {
        jwtBearerOptions.TokenValidationParameters = new TokenValidationParameters
        {
            ValidateIssuer = true,
            ValidIssuer = Issuer,
            ValidateAudience = true,
            ValidAudience = Audience,
            ValidateLifetime = true,
            IssuerSigningKey = new SymmetricSecurityKey(Encoding.UTF8.GetBytes(Key)),
            ValidateIssuerSigningKey = true,
        };
    }
}

public interface IJwtTokenToolsService
{
    public string GenerateToken(string actor, TimeSpan duration);
    public Result ValidateToken(string token);
}

public class JwtTokenTools : IJwtTokenToolsService
{
    private JwtTokenToolsSettings tokenSettings;

    public JwtTokenTools(IOptions<JwtTokenToolsSettings> tokenSettings)
    {
        this.tokenSettings = tokenSettings.Value;
    }

    public string GenerateToken(string actor, TimeSpan duration)
    {
        List<Claim> claims = new() { new Claim(ClaimTypes.Actor, actor) };
        JwtSecurityToken token =
            new(
                issuer: tokenSettings.Issuer,
                audience: tokenSettings.Audience,
                claims: claims,
                expires: DateTime.UtcNow.Add(duration),
                signingCredentials: new SigningCredentials(
                    new SymmetricSecurityKey(Encoding.UTF8.GetBytes(tokenSettings.Key)),
                    SecurityAlgorithms.HmacSha256
                )
            );
        return new JwtSecurityTokenHandler().WriteToken(token);
    }

    public Result ValidateToken(string token)
    {
        if (token == null)
        {
            return new FailResult("Null token");
        }

        JwtSecurityTokenHandler tokenHandler = new();
        try
        {
            _ = tokenHandler.ValidateToken(
                token,
                new TokenValidationParameters
                {
                    ValidateIssuerSigningKey = true,
                    IssuerSigningKey = new SymmetricSecurityKey(
                        Encoding.UTF8.GetBytes(tokenSettings.Key)
                    ),
                    ValidateIssuer = false,
                    ValidateAudience = false,
                    ClockSkew = TimeSpan.Zero
                },
                out _
            );
            return new SuccessResult();
        }
        catch (Exception ex)
        {
            return new FailResult(ex.Message);
        }
    }
}
