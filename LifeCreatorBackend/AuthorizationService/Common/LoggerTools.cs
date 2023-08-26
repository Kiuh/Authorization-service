using System.Net;

namespace AuthorizationService.Common;

public static class LoggerTools
{
    public static string GetIPAddresses()
    {
        IPAddress[] list = Dns.GetHostEntry(Dns.GetHostName()).AddressList;
        if (list == null)
        {
            return "None ips";
        }
        IEnumerable<IPAddress> ips = list.Where((x, i) => i % 2 != 0);
        IEnumerable<string> stringIps = ips.Select(x => x.ToString());
        return "[ " + stringIps.Aggregate("-->", (x, y) => x + " , " + y) + " ]";
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
