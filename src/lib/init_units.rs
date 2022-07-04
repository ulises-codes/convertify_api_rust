use std::collections::HashMap;

use crate::lib::helper;
use derive_getters::Getters;
use serde::Deserialize;
use state::LocalStorage;
use std::error::Error;
use std::fs::File;

pub struct ConvertifyState {
    pub units: HashMap<String, HashMap<String, HashMap<String, Unit>>>,
}

pub static CONFIG: LocalStorage<ConvertifyState> = LocalStorage::new();

#[derive(Clone, Getters, Debug, Deserialize)]
pub struct Record {
    conversion_type: String,
    from: String,
    to: String,
    multiply_by: f64,
    from_symbol: String,
    to_symbol: String,
}

impl Record {
    pub fn new(
        conversion_type: String,
        from: String,
        to: String,
        multiply_by: f64,
        from_symbol: String,
        to_symbol: String,
    ) -> Self {
        Self {
            conversion_type: helper::format_key(&conversion_type),
            from: helper::format_key(&from),
            to: helper::format_key(&to),
            multiply_by,
            from_symbol: helper::format_key(&from_symbol),
            to_symbol: helper::format_key(&to_symbol),
        }
    }
}

#[allow(unused)]
#[derive(Clone, Getters, Debug, Deserialize)]
pub struct Unit {
    from: String,
    to: String,
    multiply_by: f64,
    from_symbol: String,
    to_symbol: String,
}

impl Unit {
    pub fn new(
        from: String,
        to: String,
        multiply_by: f64,
        from_symbol: String,
        to_symbol: String,
    ) -> Self {
        Self {
            from: helper::format_key(&from),
            to: helper::format_key(&to),
            multiply_by,
            from_symbol: helper::format_key(&from_symbol),
            to_symbol: helper::format_key(&to_symbol),
        }
    }
}

pub fn get_data() -> Result<(), Box<dyn Error>> {
    let file = File::open("./src/resources/data.csv")?;

    let mut units: HashMap<String, HashMap<String, HashMap<String, Unit>>> = HashMap::new();

    let mut rdr = csv::Reader::from_reader(file);

    let mut record: Record;

    for result in rdr.deserialize() {
        let result: Record = result?;

        record = Record::new(
            result.conversion_type,
            result.from,
            result.to,
            result.multiply_by,
            result.from_symbol,
            result.to_symbol,
        );

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

    CONFIG.set(move || ConvertifyState {
        units: units.clone(),
    });

    Ok(())
}
