using CryptoNet;

namespace LifeCreatorBackend.Models;

public static class Cryptography
{
    public static string PublicKey = "";
    private static string privateKey = "";

    public static ICryptoNet CryptoService => new CryptoNetRsa(privateKey);

    public static void GenerateKeyPair()
    {
        ICryptoNet cryptoNet = new CryptoNetRsa();
        privateKey = cryptoNet.ExportKey(true);
        PublicKey = cryptoNet.ExportKey(false);
    }
}
