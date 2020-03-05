use serde::Deserialize;

#[derive(Deserialize)]
struct Row(String, String, u32, u32);

#[derive(Debug)]
pub struct Record {
    pub province: String,
    pub country: String,
    pub summary: Summary,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Summary {
    pub yesterday: u32,
    pub today: u32,
}

impl Summary {
    pub fn change(&self) -> u32 {
        self.today - self.yesterday
    }
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:5} {:5}\t{:8.1}%",
            self.yesterday,
            self.today,
            (self.today - self.yesterday) as f64 / self.yesterday as f64 * 100.
        )
    }
}

pub fn read(csv_data: Box<dyn std::io::Read>) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let mut records = Vec::new();
    let mut rdr = csv::Reader::from_reader(csv_data);
    for result in rdr.deserialize() {
        let row: Vec<String> = result?;
        if let [province, country, .., yesterday, today] = row.as_slice() {
            let summary = Summary {
                yesterday: yesterday.parse()?,
                today: today.parse()?,
            };
            let record = Record {
                province: province.trim().to_string(),
                country: country.trim().to_string(),
                summary,
            };
            if record.summary.today != 0 {
                records.push(record);
            }
        }
    }
    Ok(records)
}
