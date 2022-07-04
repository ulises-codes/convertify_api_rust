#[macro_use]
extern crate rocket;

mod lib {
    pub mod cors;
    pub mod helper;
    pub mod init_units;
}

use lib::init_units;
use rocket::{
    serde::json::{json, Value},
    Build, Rocket,
};
use std::collections::HashMap;

#[get("/categories")]
fn categories() -> Value {
    let config = init_units::CONFIG.get();

    let keys: Vec<&String> = config.units.keys().collect();

    json!({ "categories": keys })
}

#[get("/categories/<category>?<from>&<to>&<amount>")]
fn category(category: &str, from: Option<&str>, to: Option<&str>, amount: Option<f64>) -> Value {
    let config = init_units::CONFIG.get();

    let mut map: HashMap<&str, String> = HashMap::new();

    for (_k, unit_category) in config.units.get(category).unwrap().iter() {
        if map.contains_key(category) {
            continue;
        }

        for (l, to_unit) in unit_category.iter() {
            map.insert(l, lib::helper::remove_underscores(&to_unit.to()));
        }
    }

    let from = from.unwrap_or("");
    let to = to.unwrap_or("");
    let amount = amount.unwrap_or(0.0);

    if from.len() == 0 || to.len() == 0 || amount <= 0.0 {
        json!(map)
    } else {
        let unit = config.units.get(category).unwrap();

        if map.contains_key(from) && map.contains_key(to) {
            let x = unit.get(from).unwrap().get(to).unwrap().multiply_by();

            json!({ "result": lib::helper::multiply(amount, x) })
        } else {
            json!({"status": 400, "error": "Invalid parameters"})
        }
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    let _data = init_units::get_data();

    rocket::build()
        .mount("/", routes![categories, category])
        .attach(lib::cors::CORS)
}
