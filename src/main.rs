mod data;

use data::{Record, Summary};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(
        long,
        short,
        default_value = "https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv"
    )]
    url: String,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Select { country: String },
    Countries,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opts = Opts::from_args();
    let file = reqwest::blocking::get(&opts.url)?.text()?;
    let file = Box::new(std::io::Cursor::new(file.into_bytes()));

    let records = data::read(file)?;
    let mut results: Vec<Record> = match opts.cmd {
        Command::Select { country } => records
            .into_iter()
            .filter(|record| record.country == country)
            .collect(),
        Command::Countries => {
            let mut countries = std::collections::HashMap::new();
            for record in &records {
                let summary: &mut Summary = countries.entry(&record.country).or_default();
                summary.today += record.summary.today;
                summary.yesterday += record.summary.yesterday;
            }
            countries
                .into_iter()
                .map(|(key, summary)| Record {
                    country: key.to_string(),
                    province: key.to_string(),
                    summary,
                })
                .collect()
        }
    };

    results.sort_by_key(|result| result.summary.change());
    for result in &results {
        println!("{:43}{}", result.province, result.summary);
    }
    let summary = Summary::default();
    let summary = results.iter().fold(summary, |prev, next| Summary {
        yesterday: prev.yesterday + next.summary.yesterday,
        today: prev.today + next.summary.today,
    });
    println!("-------------------\n{:43}{}", "", summary);

    Ok(())
}
