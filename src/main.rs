#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};




#[post("/<id>")]
fn new(id: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: '{}'", id)))
}


#[get("/")]
fn json() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}

#[derive(Responder)]
#[response(status = 418, content_type = "json")]
struct RawTeapotJson(&'static str);

#[get("/home")]
fn json2() -> RawTeapotJson {
    RawTeapotJson("{ \"hi\": \"world\" }")
}

//-----------------------Manejo de Errores --------------------------------


#[get("/<id>")]
fn just_fail(id: i8) -> Status {
    if id == 0 {return Status::Accepted}
    else if id == 1 {return Status::BadRequest}
    else if id == 2 {return Status::Conflict}
    else if id == 3 {return Status::Created}
    else if id == 4 {return Status::Forbidden}
    else if id == 5 {return Status::NoContent}
    else if id == 6 {return Status::NotFound}
    else if id == 7 {return Status::NotImplemented}
    else if id == 8 {return Status::NotModified}
    else if id == 9 {return Status::Ok}
    else if id == 10 {return Status::PartialContent}
    else if id == 11 {return Status::PaymentRequired}
    else if id == 12 {return Status::PreconditionFailed}
    else {
        return Status::InternalServerError
    }
    }




// Returning a String
#[get("/hello")]
fn hello() -> String {
    "Hello, World!".to_string()
}



// Returning an Option
#[get("/optional")]
fn optional() -> Option<String> {
    Some("This is an optional response".to_string())
}

// Returning a Result
#[get("/result")]
fn result() -> Result<String, status::NotFound<String>> {
    Err(status::NotFound("Resource not found".to_string()))
}

//_______ AsyncStremas ___________

use rocket::tokio::time::{Duration, interval};
use rocket::response::stream::TextStream;

/// Produce an infinite series of `"hello"`s, one per second.
#[get("/infinite")]
fn infinite() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            yield "hello";
            interval.tick().await;
        }
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
       .mount("/", routes![new, json,json2,just_fail,infinite ])
       .mount("/new", routes![ hello, result,optional])
}
