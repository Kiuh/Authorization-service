using LifeCreatorBackend.Data;
using LifeCreatorBackend.Models;
using Microsoft.AspNetCore.Mvc;

namespace LifeCreatorBackend.Controllers;

[ApiController]
[Route("[controller]")]
public class UsersController : ControllerBase
{
    private readonly ApplicationDbContext appDbContext;

    public UsersController(ApplicationDbContext appDbContext)
    {
        this.appDbContext = appDbContext;
    }

    [HttpGet]
    public ActionResult Get()
    {
        _ = appDbContext.Users?.Add(
            new User()
            {
                Login = "LogIn",
                Email = $"jj{new Random().Next(0, 100)}",
                Password = $"{new Random().Next(0, 10000)}"
            }
        );

        _ = appDbContext.Users?.Add(new User() { Login = "LogIn", Email = $"jj" });

        _ = appDbContext.SaveChanges();

        string str = "";
        List<User>? users = appDbContext.Users?.ToList();
        if (users is null)
        {
            str = "No Users table";
        }
        else
        {
            str += "Список объектов:\n";
            foreach (User u in users.OrderBy(p => p.Id))
            {
                str += $"{u.Id}.{u.Login} - {u.Email} - {u.Password}\n";
            }
        }
        return Ok(str);
    }
}
