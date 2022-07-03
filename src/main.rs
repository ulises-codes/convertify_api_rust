#[macro_use]
extern crate rocket;

mod lib {
    pub mod helper;
    pub mod init_units;
}

use lib::init_units;
use rocket::{
    serde::json::{json, Value},
    Build, Rocket,
};

#[get("/")]
fn index() -> Value {
    let config = init_units::CONFIG.get();

    let keys: Vec<&String> = config.units.keys().collect();

    json!({ "categories": keys })
}

#[launch]
fn rocket() -> Rocket<Build> {
    init_units::get_data();

    rocket::build().mount("/", routes![index])
}
