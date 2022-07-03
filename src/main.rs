use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    conversion_type: String,
    from: String,
    to: String,
    multiply_by: f64,
    from_symbol: String,
    to_symbol: String,
}

#[derive(Getters, Debug, Deserialize)]
struct Unit {
    from: String,
    to: String,
    multiply_by: f64,
    from_symbol: String,
    to_symbol: String,
}

impl Unit {
    fn new(
        from: String,
        to: String,
        multiply_by: f64,
        from_symbol: String,
        to_symbol: String,
    ) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            multiply_by,
            from_symbol: from_symbol.to_string(),
            to_symbol: to_symbol.to_string(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./src/resources/data.csv")?;

    let mut units: HashMap<String, HashMap<String, HashMap<String, Unit>>> = HashMap::new();

    let mut rdr = csv::Reader::from_reader(file);

    let mut record: Record;

    for result in rdr.deserialize() {
        record = result.unwrap();

        let unit = Unit::new(
            record.from.clone(),
            record.to.clone(),
            record.multiply_by,
            record.from_symbol.clone(),
            record.to_symbol.clone(),
        );

        if units.contains_key(&record.conversion_type) {
            if units
                .get(&record.conversion_type)
                .unwrap()
                .contains_key(&record.from_symbol)
            {
                units
                    .get_mut(&record.conversion_type)
                    .unwrap()
                    .get_mut(&record.from_symbol)
                    .unwrap()
                    .insert(record.to_symbol, unit);
            } else {
                units.get_mut(&record.conversion_type).unwrap().insert(
                    record.from_symbol.clone(),
                    HashMap::from([(record.to_symbol, unit)]),
                );
            }
        } else {
            let to_map = HashMap::from([(record.to_symbol.clone(), unit)]);

            let from_map: HashMap<String, HashMap<String, Unit>> =
                HashMap::from([(record.from_symbol, to_map)]);

            units.insert(record.conversion_type, from_map);
        }
    }

    print!("{:?}", units.get("Area").unwrap().get("ha").unwrap().keys());

    Ok(())
}
