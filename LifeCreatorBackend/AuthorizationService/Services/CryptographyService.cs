using CryptoNet;
using Microsoft.Extensions.Options;
using System.Security.Cryptography;
using System.Text;

namespace AuthorizationService.Services;

public class CryptographyServiceSettings
{
    public required string PrivateKey { get; set; }
}

public interface ICryptographyService
{
    public string HashString(string input);
    public string EncryptString(string input);
    public string DecryptString(string input);
    public string GetPublicKey();
}

public class CryptographyService : ICryptographyService
{
    private ICryptoNet cryptoNet;
    private string publicKey;

    public CryptographyService(IOptions<CryptographyServiceSettings> cryptoData)
    {
        cryptoNet = new CryptoNetRsa(cryptoData.Value.PrivateKey);
        publicKey = cryptoNet.ExportKey(false);
    }

    public string DecryptString(string input)
    {
        byte[] bytes = Convert.FromBase64String(input);
        byte[] encrypted = cryptoNet.DecryptToBytes(bytes.ToArray());
        return Encoding.UTF8.GetString(encrypted);
    }

    public string EncryptString(string input)
    {
        byte[] encrypted = cryptoNet.EncryptFromString(input);
        return Convert.ToBase64String(encrypted);
    }

    public string GetPublicKey()
    {
        return publicKey;
    }

    public string HashString(string input)
    {
        byte[] hashedBytes = SHA256.HashData(Encoding.UTF8.GetBytes(input));
        StringBuilder sBuilder = new();
        for (int i = 0; i < hashedBytes.Length; i++)
        {
            _ = sBuilder.Append(hashedBytes[i].ToString("x2"));
        }
        return sBuilder.ToString();
    }
}
