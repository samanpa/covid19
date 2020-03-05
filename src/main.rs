use serde::Deserialize;
use structopt::StructOpt;

#[derive(Deserialize)]
struct Row(String, String, u32, u32);

#[derive(Debug)]
struct Record {
    province: String,
    country: String,
    yesterday: u32,
    today: u32,
}


impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:43} {:5} {:5}\t{:8.1}%",
               self.province,
               self.yesterday,
               self.today,
               (self.today - self.yesterday) as f64 / self.yesterday as f64 * 100.
        )
    }
}

#[derive(StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Select {
        country: String
    }
}


fn read() -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let mut records = Vec::new();
    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    for result in rdr.deserialize() {
        let row: Row = result?;
        let record = Record {
            province: row.0,
            country: row.1,
            yesterday: row.2,
            today: row.3,
        };
        records.push(record);
    }
    Ok(records)
}

fn main() {
    let records = match read() {
        Ok(records) => records,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let opts = Opts::from_args();
    let results = match opts.cmd {
        Command::Select{ country } => {
            let mut results: Vec<_> = records.into_iter()
                .filter(|record| record.country == country)
                .collect();
            results.sort_by_key(|result| result.yesterday as i32 - result.today as i32);
            for result in &results {
                println!("{}", result);
            }

            results
        }
    };

    let summary = Record {
        province: String::new(),
        country: String::new(),
        yesterday: 0,
        today: 0
    };
    let summary = results.iter().fold(summary, |prev, next| Record {
        yesterday: prev.yesterday + next.yesterday,
        today: prev.today + next.today,
        province: String::new(),
        country: String::new(),
    });
    println!("-------------------\n{}", summary);
}
