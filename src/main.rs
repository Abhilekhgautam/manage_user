#[macro_use] extern crate rocket;
use rocket::http::{ContentType,Status};
use rocket::{fs::TempFile};
use rocket_dyn_templates::Template;
use rocket::form::{Form, Contextual};

#[derive(Debug, FromForm)]
struct Password<'a>{
    #[field(validate = len(8..))]
    #[field(validate = eq(self.confirm_entry))]
    first_entry  : &'a str,
    #[field(validate = eq(self.first_entry))]
    confirm_entry: &'a str,
}

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct User<'a>{
    first_name : &'a str,
    last_name  : &'a str,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    email      : &'a str,
    password   : Password<'a>,
    #[field(validate = ext(ContentType::JPEG))]
    profile_picture: TempFile<'a>,
}

#[get("/")]
fn hello() -> &'static str{
  "Hello, World"
}

#[post("/addUser", data = "<form>")]
fn add_user<'a>(form: Form<Contextual<'a, User>>) -> (Status, Template) {
    let template = match form.value{
        Some(ref val) => Template::render("profile", &form.context),
        None            => Template::render("signup", &form.context),
    };

    (form.context.status(), template)
}

#[launch]
fn rocket() -> _{
    rocket::build().mount("/", routes![hello])
}
