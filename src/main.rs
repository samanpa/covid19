mod data;
mod ops;

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
    #[structopt(
        long,
        short,
        default_value = "2",
        help = "Number of columns (days) to show"
    )]
    num_cols: usize,
    #[structopt(
        long,
        short,
        default_value = "1",
        help = "1 for daily stats, 7 for weekly, 30 for monthly"
    )]
    skip: usize,
    #[structopt(
        long,
        default_value = "100",
        help = "maximum number of entries to show"
    )]
    num_rows: usize,
    #[structopt(long, help = "sort by name instead of number of confirmed cases")]
    sort_by_name: bool,
    #[structopt(long, help = "Group by states instead of the default by country")]
    states: bool,
    #[structopt(long, default_value = "0", help = "Minimum value we want to show")]
    min: u32,
    countries: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opts = Opts::from_args();
    let file = reqwest::blocking::get(&opts.url)?.text()?;
    let file = Box::new(std::io::Cursor::new(file.into_bytes()));
    let table = data::read(file)?;

    let sort = if opts.sort_by_name {
        SortBy::Name
    } else {
        SortBy::Max
    };
    let ops = vec![
        match opts.countries.as_slice() {
            [] => Op::NoOp,
            countries => Op::Filter(countries.to_vec()),
        },
        if opts.states {
            Op::NoOp
        } else {
            Op::GroupByCountry
        },
        Op::GreaterThan(opts.min),
        Op::Select {
            start: 0,
            size: opts.num_cols,
            step: opts.skip,
        },
        Op::SortBy(sort),
        Op::Limit(opts.num_rows),
    ];

    let table = ops::eval(ops, table);
    table.write(std::io::stdout())?;

    Ok(())
}
