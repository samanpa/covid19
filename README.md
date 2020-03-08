Utility to get summary of covid data because why not
- Original data can be found [here](https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv)


```sh
USAGE:
    covid19 [FLAGS] [OPTIONS] [countries]...

FLAGS:
        --by-name    sort by name instead of number of confirmed cases
    -h, --help       Prints help information
        --states     Group by states
    -V, --version    Prints version information

OPTIONS:
        --limit <limit>          maximum number of entries to show [default: 50000]
    -n, --num-cols <num-cols>    Number of columns (days) to show [default: 2]
    -s, --skip <skip>            1 for daily stats, 7 for weekly, 30 for monthly [default: 1]
    -u, --url <url>               [default: https://raw.githubusercontent.com/CSSEGISandData/COVID-
                                 19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-
                                 Confirmed.csv]

ARGS:
   <countries>...    
```

```sh
adamsk@ruth:~/src/self/covid19$ ./target/release/covid19 --limit 3 --num-cols 4 --states US
City     State   Country  3/4/20  3/5/20  3/6/20  3/7/20  
         NY      US       11      23      36      76      
         CA      US       35      51      59      82      
         WA      US       39      70      83      107     
Summary  ------  -------  85      144     178     265
```