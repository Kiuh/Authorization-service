using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.IdentityModel.Tokens;
using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using System.Text;

namespace AuthorizationService.Models;

public static class JwtToken
{
    private const string issuer = "LifeCreatorBackend";
    private const string audience = "LifeCreatorUnity";
    private static string key;

    private static SymmetricSecurityKey SymmetricSecurityKey => new(Encoding.UTF8.GetBytes(key));

    static JwtToken()
    {
        long number = DateTimeOffset.Now.ToUnixTimeMilliseconds();
        key = Convert.ToString(number * number);
    }

    public static void ConfigurateJwtBearerOptions(JwtBearerOptions jwtBearerOptions)
    {
        jwtBearerOptions.TokenValidationParameters = new TokenValidationParameters
        {
            ValidateIssuer = true,
            ValidIssuer = issuer,
            ValidateAudience = true,
            ValidAudience = audience,
            ValidateLifetime = true,
            IssuerSigningKey = SymmetricSecurityKey,
            ValidateIssuerSigningKey = true,
        };
    }

    public static JwtSecurityToken GetJwtSecurityToken(string login)
    {
        List<Claim> claims = new() { new Claim(ClaimTypes.Actor, login) };
        return new JwtSecurityToken(
            issuer: issuer,
            audience: audience,
            claims: claims,
            expires: DateTime.UtcNow.Add(TimeSpan.FromDays(1)),
            signingCredentials: new SigningCredentials(
                SymmetricSecurityKey,
                SecurityAlgorithms.HmacSha256
            )
        );
    }
}
