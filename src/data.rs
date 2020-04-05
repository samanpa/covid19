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
    pub country: String,
    pub province: String,
}

impl Name {
    pub fn new(province: &str, country: &str) -> Self {
        Name {
            province: province.trim().to_string(),
            country: country.to_string(),
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

pub fn read_us(csv: Box<dyn Read>) -> Result<Table, Box<dyn Error>> {
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
        if let [_, _, _, _, _, county, state, _, _, _, data @ ..] = row.as_slice() {
            let name = Name::new(county, state);
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
    pub fn write<W: std::io::Write>(&self, diffs: bool, w: W) -> Result<(), std::io::Error> {
        use num_format::{Locale, ToFormattedString};
        use std::io::Write;
        let locale = &Locale::en;
        let mut writer = tabwriter::TabWriter::new(w);

        write!(writer, "County\tState/Country\t")?;
        for header in self.header.iter().rev() {
            write!(writer, "{}\t", header)?;
        }
        writeln!(writer)?;

        for row in &self.rows {
            let nm = &row.name;
            let mut prev: i32 = 0;
            write!(writer, "{}\t{}\t", nm.province, nm.country)?;
            for val in row.data.iter().rev() {
                let val = *val as i32;
                let fval = if diffs {
                    (val - prev).to_formatted_string(locale)
                } else {
                    val.to_formatted_string(locale)
                };
                write!(writer, "{}\t", fval)?;
                prev = val;
            }
            writeln!(writer)?;
        }

        let mut summary = Vec::new();
        for row in &self.rows {
            let data = row.data.iter().rev();
            if summary.is_empty() {
                summary = data.copied().collect();
            } else {
                summary.iter_mut().zip(data).for_each(|(v1, v2)| *v1 += v2);
            }
        }
        write!(writer, "Summary\t-------\t")?;
        for val in summary {
            write!(writer, "{}\t", val.to_formatted_string(&Locale::en))?;
        }
        writeln!(writer, "\n")?;
        writer.flush()
    }
}
