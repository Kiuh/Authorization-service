using CryptoNet;
using System.Security.Cryptography;
using System.Text;

namespace AuthorizationService.Common;

public static class StringExtensions
{
    public static string GetHash(this string input)
    {
        using SHA256 sha256Hash = SHA256.Create();
        byte[] hashedBytes = sha256Hash.ComputeHash(Encoding.UTF8.GetBytes(input));
        StringBuilder sBuilder = new();
        for (int i = 0; i < hashedBytes.Length; i++)
        {
            _ = sBuilder.Append(hashedBytes[i].ToString("x2"));
        }
        return sBuilder.ToString();
    }

    public static string GetDecrypted(this string value, ICryptoNet rsa)
    {
        byte[] bytes = Convert.FromBase64String(value);
        byte[] encrypted = rsa.DecryptToBytes(bytes);
        return Encoding.UTF8.GetString(encrypted);
    }
}
