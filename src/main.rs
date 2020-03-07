#![feature(is_sorted)]

mod data;
mod ops;

use data::Place;
use ops::Op;
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
    FilterCountry { country: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opts = Opts::from_args();
    let file = reqwest::blocking::get(&opts.url)?.text()?;
    let file = Box::new(std::io::Cursor::new(file.into_bytes()));
    let table = data::read(file)?;
    let op = match opts.cmd {
        Command::FilterCountry { country } => ops::Op::Filter {
            name: country,
            place: Place::Country,
        },
    };

    let select = Op::Select {
        start: 0,
        size: 2,
        step: 1,
    };
    let ops = Op::Combine(Box::new(op), Box::new(select));
    let table = ops::eval(&ops, &table);
    println!("{:#?}", table);

    Ok(())
}
