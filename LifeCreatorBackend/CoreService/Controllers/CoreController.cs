using Microsoft.AspNetCore.Mvc;

namespace CoreService.Controllers;

[ApiController]
public class CoreController : Controller
{
    private readonly ILogger<CoreController> logger;

    public CoreController(ILogger<CoreController> logger)
    {
        this.logger = logger;
    }

    [HttpGet("/Core")]
    public IActionResult Index()
    {
        logger.LogInformation("Handle request");
        return Ok("In Development");
    }
}
