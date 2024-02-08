#[macro_use] extern crate rocket;
use rocket::http::{Status, ContentType};
use rocket::response::{content, status};
use rocket_dyn_templates::{Template, context};
use yew::{Html, html};

#[derive(Responder)]
#[response( status = 418, content_type = "json")]
struct RawTeapotJson(&'static str);

//#[derive(Responder)]
//#[response( status = 200, content_type = "HTML")]
//struct WebpageContent(HTML);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/response")]
//fn headlines() -> status::Custom<content::RawJson<&'static str>> {
fn teapot() -> RawTeapotJson {
    RawTeapotJson("{\"hi\": \"world\"}")
    //html!("<h1> Hello World</h1>");
}

#[get("/template")]
fn example() -> Template {
    let users = vec!["mark", "ruffalo"];
    Template::render("test_template_base", context! { users: users } )
}
#[get("/template_override")]
    fn template_2() -> Template {
    let users = vec!["mark", "ruffalo"];
    Template::render("first_extend_template", context! { users: users } )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![teapot])
        .mount("/", routes![example])
        .mount("/", routes![template_2])
        .attach(Template::fairing())
}
