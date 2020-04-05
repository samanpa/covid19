use covid19::data::{self, Table};
use covid19::ops::{self, Op, SortBy};
use structopt::StructOpt;

#[derive(StructOpt)]
struct CommonOpts {
    #[structopt(
        long,
        short,
        default_value = "3",
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
    #[structopt(long, default_value = "50", help = "maximum number of entries to show")]
    num_rows: usize,
    #[structopt(long, help = "sort by name instead of number of confirmed cases")]
    sort_by_name: bool,
    #[structopt(long, default_value = "0", help = "Minimum value we want to show")]
    min: u32,
    #[structopt(long, help = "Break down by county/state")]
    break_down: bool,
    #[structopt(long, help = "Show diffs")]
    show_diffs: bool,
    countries: Vec<String>,
}

#[derive(StructOpt)]
struct ConfirmedOpts {
    #[structopt(
        long,
        short,
        default_value = "https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_global.csv"
    )]
    url: String,
    #[structopt(flatten)]
    common_opts: CommonOpts,
}

#[derive(StructOpt)]
struct DeathsOpts {
    #[structopt(
        long,
        short,
        default_value = "https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_deaths_global.csv"
    )]
    url: String,
    #[structopt(flatten)]
    common_opts: CommonOpts,
}

#[derive(StructOpt)]
struct ConfirmedUSOpts {
    #[structopt(
        long,
        short,
        default_value = "https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_US.csv"
    )]
    url: String,
    #[structopt(flatten)]
    common_opts: CommonOpts,
}

#[derive(StructOpt)]
struct DeathsUSOpts {
    #[structopt(
        long,
        short,
        default_value = "https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_deaths_US.csv"
    )]
    url: String,
    #[structopt(flatten)]
    common_opts: CommonOpts,
}

#[derive(StructOpt)]
enum Opts {
    Confirmed(ConfirmedOpts),
    Deaths(DeathsOpts),
    USConfirmed(ConfirmedUSOpts),
    USDeaths(DeathsUSOpts),
}

impl Opts {
    fn get_table(self) -> Result<(Table, CommonOpts), Box<dyn std::error::Error>> {
        let (table, opts) = match self {
            Self::Confirmed(opts) => {
                let file = reqwest::blocking::get(&opts.url)?.text()?;
                let file = Box::new(std::io::Cursor::new(file.into_bytes()));
                let table = data::read(file)?;
                (table, opts.common_opts)
            }
            Self::Deaths(opts) => {
                let file = reqwest::blocking::get(&opts.url)?.text()?;
                let file = Box::new(std::io::Cursor::new(file.into_bytes()));
                let table = data::read(file)?;
                (table, opts.common_opts)
            }
            Self::USConfirmed(opts) => {
                let file = reqwest::blocking::get(&opts.url)?.text()?;
                let file = Box::new(std::io::Cursor::new(file.into_bytes()));
                let table = data::read_us(file)?;
                (table, opts.common_opts)
            }
            Self::USDeaths(opts) => {
                let file = reqwest::blocking::get(&opts.url)?.text()?;
                let file = Box::new(std::io::Cursor::new(file.into_bytes()));
                let table = data::read_us(file)?;
                (table, opts.common_opts)
            }
        };

        Ok((table, opts))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opts = Opts::from_args();
    let (table, opts) = opts.get_table()?;

    run(opts, table)
}

fn run(opts: CommonOpts, table: Table) -> Result<(), Box<dyn std::error::Error>> {
    let sort = if opts.sort_by_name {
        SortBy::Name
    } else {
        SortBy::Max
    };
    let ops = vec![
        Op::Select {
            start: 0,
            size: opts.num_cols,
            step: opts.skip,
        },
        match opts.countries.as_slice() {
            [] => Op::NoOp,
            countries => Op::Filter(countries.to_vec()),
        },
        if opts.break_down {
            Op::NoOp
        } else {
            Op::GroupByCountry
        },
        Op::GreaterThan(opts.min),
        Op::SortBy(sort),
        Op::Limit(opts.num_rows),
    ];
    let table = ops::eval(ops, table);
    table.write(opts.show_diffs, std::io::stdout())?;

    Ok(())
}
