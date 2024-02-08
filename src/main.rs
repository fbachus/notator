#[macro_use] extern crate rocket;
use rocket::http::{Status, ContentType};
use rocket::response::{content, status};
use rocket_dyn_templates::{Template, context};
use rocket::fs::NamedFile;
use yew::{Html, html};
use std::path::PathBuf;

#[derive(Responder)]
#[response( status = 418, content_type = "json")]
struct RawTeapotJson(&'static str);
#[derive(Responder)]
#[response( status = 404, content_type = "json")]
struct NotFound(&'static str);

//#[derive(Responder)]
//#[response( status = 200, content_type = "HTML")]
//struct WebpageContent(HTML);

#[get("/")]
fn index() -> Template {
    let title = String::from("Home");
    Template::render("test_template_base",
                     context! { title: title } )
}

#[get("/<css_file>", rank = 2)]
    async fn serve_css(css_file: PathBuf) -> content::RawCss<NamedFile> {
        let path: PathBuf = [PathBuf::from("./style"), css_file].iter().collect();
        content::RawCss(NamedFile::open(path).await.expect("no file found"))
}

#[get("/response")]
//fn headlines() -> status::Custom<content::RawJson<&'static str>> {
fn teapot() -> RawTeapotJson {
    RawTeapotJson("{\"hi\": \"world\"}")
    //html!("<h1> Hello World</h1>");
}

#[get("/template")]
fn example() -> Template {
    let users = vec!["john", "lennon"];
    let title = String::from("other Template");
    Template::render("first_extend_template",
                     context! { users: users, title: title } )
}
//#[get("/template_override_test_empty")]
//fn template() -> Template {
//    let users = vec!["john", "lennon"];
//    let title = String::from("Notator");
//    Template::render("first_extend_template",
//                     context! { users: users, title: title } )
//}
#[get("/<_..>", rank = 12)]
fn not_found() -> NotFound {
    NotFound("could not find path")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![teapot])
        .mount("/", routes![example])
        .mount("/style", routes![serve_css])
        .mount("/", routes![not_found])
        .attach(Template::fairing())
}
