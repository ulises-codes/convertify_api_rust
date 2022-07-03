#[macro_use]
extern crate rocket;

mod lib {
    pub mod helper;
    pub mod init_units;
}

use std::collections::HashMap;

use lib::{helper::remove_underscores, init_units};
use rocket::{
    serde::json::{json, Value},
    Build, Rocket,
};

#[get("/categories")]
fn categories() -> Value {
    let config = init_units::CONFIG.get();

    let keys: Vec<&String> = config.units.keys().collect();

    json!({ "categories": keys })
}

#[get("/categories/<category>")]
fn category(category: &str) -> Value {
    let config = init_units::CONFIG.get();

    let mut map: HashMap<String, &str> = HashMap::new();

    for (_k, unit_category) in config.units.get(category).unwrap().iter() {
        if map.contains_key(category) {
            continue;
        }

        for (l, to_unit) in unit_category.iter() {
            map.insert(lib::helper::remove_underscores(&to_unit.from()), l);
        }
    }

    json!({ "data": map })
}

#[launch]
fn rocket() -> Rocket<Build> {
    init_units::get_data();

    rocket::build().mount("/", routes![categories, category])
}
