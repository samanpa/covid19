mod data;
mod ops;

use data::Place;
use ops::{Op, SortBy};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(
        long,
        short,
        default_value = "https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv"
    )]
    url: String,
    #[structopt(long, short, default_value="2")]
    num_days: usize,
    #[structopt(long)]
    by_name: bool,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(about = "Summary of cases by country" )]
    Country { country: String },
    #[structopt(about = "Summary of cases in the US by state" )]
    UsSummary,
    #[structopt(about = "Worldwide summary" )]    
    Summary,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opts = Opts::from_args();
    let file = reqwest::blocking::get(&opts.url)?.text()?;
    let file = Box::new(std::io::Cursor::new(file.into_bytes()));
    let table = data::read(file)?;
    let op = match opts.cmd {
        Command::Country { country } => Op::Filter {
            name: country,
            place: Place::Country,
        },
        Command::UsSummary => {
            let name = "US".to_string();
            let place = Place::Country;
            let filter = Op::Filter { name, place };
            let group_by = Op::GroupBy(Place::State);
            Op::Combine(Box::new(filter), Box::new(group_by))
        },
        Command::Summary => Op::GroupBy(Place::Country),
    };

    let select = Op::Select {
        start: 0,
        size: opts.num_days,
        step: 1,
    };
    let ops = Op::Combine(Box::new(op), Box::new(select));
    let table = ops::eval(&ops, &table);
    let table = if opts.by_name {
        ops::eval(&Op::SortBy(SortBy::Name), &table)
    } else {
        ops::eval(&Op::SortBy(SortBy::Max), &table)
    };

    table.write(std::io::stdout())?;

    Ok(())
}
