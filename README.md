Utility to get summary of covid data because why not
- Original data can be found [here](https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv)


```sh
USAGE:
    covid19 [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --num-days <num-days>     [default: 2]
    -u, --url <url>               [default: https://raw.githubusercontent.com/CSSEGISandData/COVID-
                                 19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-
                                 Confirmed.csv]

SUBCOMMANDS:
    country       Summary of cases by country
    help          Prints this message or the help of the given subcommand(s)
    summary       Worldwide summary
    us-summary    Summary of cases in the US by state
```

```sh
adamsk@ruth:~/src/self/covid19$ ./target/release/covid19 --num-days 2 us-summary
City     State                                        Country  3/5/20  3/6/20  
         CA (From Diamond Princess)                   US       0       0       
         NE (From Diamond Princess)                   US       0       0       
         TX (From Diamond Princess)                   US       0       0       
         IN                                           US       0       1       
         KY                                           US       0       1       
         MN                                           US       0       1       
         NE                                           US       0       1       
         TN                                           US       1       1       
         WI                                           US       1       1       
         NC                                           US       1       2       
         NH                                           US       2       2       
         NJ                                           US       2       2       
         NV                                           US       1       2       
         PA                                           US       0       2       
         RI                                           US       2       2       
         AZ                                           US       2       3       
         GA                                           US       2       3       
         MD                                           US       0       3       
         OR                                           US       3       3       
         CO                                           US       0       4       
         FL                                           US       4       4       
         IL                                           US       5       5       
         TX                                           US       4       5       
         MA                                           US       2       7       
         NY                                           US       23      36      
         Unassigned Location (From Diamond Princess)  US       45      45      
         CA                                           US       51      59      
         WA                                           US       70      83      
Summary  ------                                       -------  221     278     
```