namespace AuthorizationService.Common;

public class ErrorBody
{
    public required string Error { get; set; }
}

public static class ErrorBodyTools
{
    public static ErrorBody ToErrorBody(this string input)
    {
        return new ErrorBody() { Error = input };
    }
}
