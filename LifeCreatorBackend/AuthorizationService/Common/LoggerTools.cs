using System.Net;

namespace AuthorizationService.Common;

public static class LoggerTools
{
    public static string GetIPAddresses()
    {
        return "[ "
            + Dns.GetHostEntry(Dns.GetHostName())
                .AddressList.Where((x, i) => i % 2 != 0)
                .Select(x => x.ToString())
                .Aggregate((x, y) => x + " , " + y)
            + " ]";
    }

    public static void LogDefaultInfo(this ILogger logger, HttpRequest httpRequest)
    {
        logger.LogInformation(
            "Processing request {RP} at {DT} from {IPs}",
            httpRequest.Path,
            DateTime.UtcNow.ToLongTimeString(),
            GetIPAddresses()
        );
    }
}
