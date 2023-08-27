using AuthorizationService.Services;
using Microsoft.Extensions.Options;
using Moq;

namespace AuthorizationService.Tests.ServiceTests;
public class CryptographyServiceTests
{
    private Mock<IOptions<CryptographyServiceSettings>> cryptographyServiceSettingsMock;

    public CryptographyServiceTests()
    {
        CryptographyServiceSettings cryptographyServiceSettings = new() { PrivateKey = "" };
        cryptographyServiceSettingsMock = new Mock<IOptions<CryptographyServiceSettings>>();
        _ = cryptographyServiceSettingsMock.Setup(x => x.Value).Returns(cryptographyServiceSettings);
    }

    public void
}
