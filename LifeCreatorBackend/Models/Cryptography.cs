using CryptoNet;

namespace LifeCreatorBackend.Models;

public static class Cryptography
{
    public static string PublicKey = "";
    private static string privateKey = "";

    public static ICryptoNet CryptoService => new CryptoNetRsa(privateKey);

    static Cryptography()
    {
        ICryptoNet cryptoNet = new CryptoNetRsa();
        privateKey = cryptoNet.ExportKey(true);
        PublicKey = cryptoNet.ExportKey(false);
    }
}
