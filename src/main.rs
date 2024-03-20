#[macro_use] extern crate rocket;

use rocket::http::{Status, ContentType, CookieJar};
use rocket::response::{content, status, Redirect, Response};
use rocket::fs::{NamedFile, FileServer};
use rocket::form::Form;
use rocket::Request;
use rocket_dyn_templates::{Template, context};
use std::path::{Path,PathBuf};

#[derive(Responder)]
#[response( status = 418, content_type = "json")]
struct RawTeapotJson(&'static str);
#[derive(Responder)]
#[response( status = 404, content_type = "json")]
struct NotFound(&'static str);
#[derive(Responder)]
#[response( status = 400, content_type = "json")]
struct BadRequest(&'static str);
#[derive(Responder)]
#[response( status = 201, content_type = "json")]
struct Created(&'static str);
#[catch(400)]
fn bad_request(req: &Request) -> String{
    format!("Invalid input")
}
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Page or Resource could not be found: {}", req.uri())
}

#[derive(FromForm,Debug)]
struct NewNote {
    content: String,
}
#[derive(FromForm, Debug)]
struct User {
    username: String,
    password: Password,
}
#[derive(FromForm, Debug)]
struct Password(String);
#[derive(FromForm, Debug)]
struct CalcVars {
    num1: f32,
    num2: f32,
}


#[post("/note", data = "<new_note>")]
async fn create_new_note(new_note: Form<NewNote>) -> Template {
    let title = String::from("Note created!");
    println!("{:?}", new_note.content);
    Template::render("note_created",
                     context! { title: title, text: &new_note.content })
}

#[get("/")]
async fn index() -> Redirect {
    Redirect::to(uri!(home))
}
#[get("/home")]
async fn home() -> Template {
    let title = String::from("Home");
    Template::render("test_template_base",
                     context! { title: title } )
}
#[get("/note/new")]
async fn write_note() -> Template {
    let title = String::from("New Note");
    Template::render("create_note",
                     context! { title: title } )
}
#[get("/user/login")]
async fn login_page() -> Template {
    let title = String::from("Login");
    Template::render("login_form",
                     context! { title: title } )
}
#[post("/login", data = "<user_auth>")]
async fn login(jar: &CookieJar<'_>, user_auth: Form<User>) -> Status {
    jar.add(("user_name", user_auth.username.to_owned()));
    Status::Ok
}
#[get("/calculator")]
async fn calculator() -> Template {
    let title = String::from("Calculator");
    Template::render("calculator",
                     context! { title: title } )
}
#[post("/add", data = "<calc_vars>")]
async fn relay_add(calc_vars: Form<CalcVars>) -> Redirect {
    Redirect::to(uri!(add(calc_vars.num1, calc_vars.num2)))
}

#[get("/add/<num1>/<num2>")]
async fn add(num1: f32, num2: f32) -> Template {
    let title = String::from("Calculator - addition");
    let operator = String::from("+");
    let result = num1 + num2;
    Template::render("calc_result",
                     context! { title, num1, num2, operator, result } )
}
#[get("/multiply?<num1>&<num2>")]
async fn multi(num1: f32, num2: f32) -> Template {
    let title = String::from("Calculator - multiplication");
    let operator = String::from("*");
    let result = num1 * num2;
    Template::render("calc_result",
                     context! { title, num1, num2, operator, result } )
}

#[get("/css/<css_file>")]
async fn serve_css(css_file: PathBuf) -> Option<NamedFile> {
    //let path: PathBuf = [PathBuf::from("./style"), css_file].iter().collect();
    NamedFile::open(Path::new("style").join(css_file)).await.ok()
}

#[get("/response")]
//fn headlines() -> status::Custom<content::RawJson<&'static str>> {
fn teapot() -> RawTeapotJson {
    RawTeapotJson("{\"hi\": \"world\"}")
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![home])
        .mount("/", routes![login])
        .mount("/", routes![login_page])
        .mount("/", routes![write_note])
        .mount("/", routes![create_new_note])
        .mount("/", routes![teapot])
        .mount("/", routes![example])
        .mount("/", routes![calculator])
        .mount("/", routes![relay_add])
        .mount("/", routes![add])
        .mount("/", routes![multi])
        .mount("/assets/", routes![serve_css])
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
