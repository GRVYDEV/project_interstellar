
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;

#[cfg(test)] mod tests;

use std::collections::HashMap;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}


#[get("/")]
fn index() -> Template {
    let ctx: HashMap<&str, u32> = HashMap::new();
    Template::render("index", &ctx)
}

#[get("/hello/<name>")]
fn get(name: String)  {
    
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get])
        .mount("/public", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}