#[macro_use] extern crate rocket;


// cokies 
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};


use rocket::{Rocket, Build};
use rocket_dyn_templates::{Template, context};
use rocket::tokio::time::{Duration, sleep};

use rocket::Request;

use rocket::http::Status;


/// Retrieve the user's ID, if any.
#[get("/user_id")]
fn user_id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get_private("user_id")
        .map(|crumb| format!("User ID: {}", crumb.value()))
}

/// Remove the `user_id` cookie.
#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {foo: "hello world"})
}


#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

// #[get("/<_..>")]
// fn everything() -> &'static str {
//     "Hey, you're here."
// }

#[get("/")]
fn home() -> &'static str {
    "Hello, home, como estamos!"
}
#[get("/lucas")]
fn lucas() -> &'static str {
    "Lucas en la tarde!"
}
#[post("/")]
fn create_post() -> &'static str {
    "Estos es la primera creacion de post !"
}
#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/?<numbers>")]
fn form(numbers: Vec<usize>) -> String {
    let sum: usize = numbers.iter().sum();
    // Verifica la lógica aquí, asegurándote de que sum sea correcto
    println!("Suma de números: {}", sum);
    // Asegúrate de que el resultado se almacene correctamente en la cadena de formato
    format!("La suma de los números es: {}", sum)
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Delaying for {} seconds", seconds)
}



#[get("/test")]
fn test()-> &'static str{
    "esto es una prueba"
}



#[catch(default)]
fn default_catcher(status: Status, request: &Request) -> String {
    format!("sorry, '{}' is not a valid path: erro {}.", request.uri(),status)
}

#[catch(404)]
fn foo_not_found() -> &'static str {
    "Foo 404"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index,lucas, form, test, delay, foo_bar, hello, user_id, logout])
    .mount("/home", routes![home,  create_post, test])
    .register("/", catchers![default_catcher])
    .register("/foo", catchers![foo_not_found])
    .attach(Template::fairing())
}



 