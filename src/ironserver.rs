use iron::prelude::*;
use iron::status;

pub fn ironserver() {
    let handler = |_: &mut Request| {
        Ok(Response::with((status::Ok, "Imma be a webserver!")))
    };

    Iron::new(handler)
        .http("localhost:3000")
        .expect("Failed to open port 3000");
}
