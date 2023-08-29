namespace AuthorizationService.Common;

public class ApiException : Exception
{
    public int StatusCode { get; set; }

    public ApiException(int statusCode, string message) : base(message)
    {
        StatusCode = statusCode;
    }
}
