using CryptoNet;
using System.Security.Cryptography;
using System.Text;

namespace LifeCreatorBackend.Common;

public static class StringExtensions
{
    public static string GetHash(this string input)
    {
        SHA256Managed sha256 = new();
        byte[] hashedBytes = sha256.ComputeHash(Encoding.UTF8.GetBytes(input));
        return Encoding.UTF8.GetString(hashedBytes);
    }

    public static string GetDecrypted(this string value, ICryptoNet rsa)
    {
        byte[] bytes = Convert.FromBase64String(value);
        byte[] encrypted = rsa.DecryptToBytes(bytes);
        return Encoding.UTF8.GetString(encrypted);
    }
}
