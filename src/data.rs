use std::error::Error;
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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
pub struct Name {
    country: String,
    province: String,
    city: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Place {
    Country,
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

    pub fn group_name(&self, place: Place) -> Self {
        let country = self.country.clone();
        let province = self.province.clone();
        match place {
            Place::Country => Self {
                country,
                province: String::new(),
                city: String::new(),
            },
            Place::State => Self {
                country,
                province,
                city: String::new(),
            },
        }
    }

    pub fn get(&self, place: Place) -> &str {
        match place {
            Place::Country => &self.country,
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
            data.reverse();
            let row = Row { name, data };
            rows.push(row);
        }
    }
    rows.sort_by_key(|row| row.name.clone());
    let table = Table { header, rows };
    Ok(table)
}


impl Table {
    pub fn write<W: std::io::Write>(&self, w: W) -> Result<(), std::io::Error> {
        use std::io::Write;
        let mut writer = tabwriter::TabWriter::new(w);
        
        write!(writer, "City\tState\tCountry\t")?;
        for header in self.header.iter().rev() {
            write!(writer, "{}\t", header)?;
        }
        writeln!(writer)?;
        
        for row in &self.rows {
            let nm = &row.name;
            write!(writer, "{}\t{}\t{}\t", nm.city, nm.province, nm.country)?;
            for val in row.data.iter().rev() {
                write!(writer, "{}\t", val)?;
            }
            writeln!(writer)?;
        }

        //writeln!(writer, "-------------------------------\n")?;
        let mut summary = Vec::new();
        for row in &self.rows {
            let data = row.data.iter().rev();
            if summary.is_empty() {
                summary = data.copied().collect();
            } else {
                summary.iter_mut()
                    .zip(data)
                    .for_each(|(v1, v2)| *v1 += v2);
            }
        }
        write!(writer, "Summary\t------\t-------\t")?;
        for val in summary {
            write!(writer, "{}\t", val)?;
        }
        writeln!(writer, "\n")?;
        writer.flush()
    }
}
