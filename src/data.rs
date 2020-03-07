use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::io::Read;
use std::rc::Rc;

#[derive(Debug)]
pub struct Table {
    pub header: Rc<Vec<String>>,
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub name: Name,
    pub data: Vec<u32>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Name {
    country: String,
    province: String,
    city: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Place {
    Country,
    City,
    State,
}

impl Name {
    pub fn new(city_or_province: &str, country: &str) -> Self {
        let (city, province) = match city_or_province.find(',') {
            Some(idx) => (&city_or_province[..idx], &city_or_province[idx + 1..]),
            None => ("", city_or_province),
        };
        Name {
            city: city.trim().to_string(),
            province: province.trim().to_string(),
            country: country.to_string(),
        }
    }

    pub fn get(&self, place: Place) -> &str {
        match place {
            Place::Country => &self.country,
            Place::City => &self.city,
            Place::State => &self.province,
        }
    }
}

pub fn read(csv: Box<dyn Read>) -> Result<Table, Box<dyn Error>> {
    let mut rows = Vec::new();
    let mut rdr = csv::Reader::from_reader(csv);
    let header: Vec<String> = rdr
        .headers()?
        .iter()
        .rev()
        .map(ToString::to_string)
        .collect();
    let header = Rc::new(header);
    for result in rdr.deserialize() {
        let row: Vec<String> = result?;
        if let [province, country, _long, _lat, data @ ..] = row.as_slice() {
            let name = Name::new(province, country);
            let mut data: Vec<u32> = data
                .iter()
                .map(|val| val.parse().unwrap_or_default())
                .collect();
            if data.is_sorted() {
                data.reverse();
                if let Some(idx) = data.iter().position(|&v| v == 0) {
                    data.drain(idx..);
                };
                let row = Row { name, data };
                rows.push(row);
            }
        }
    }
    rows.sort_by_key(|row| row.name.clone());
    let table = Table { header, rows };
    Ok(table)
}
