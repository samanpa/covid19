Utility to get summary of historical covid data.
- Original data can be found [here](https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv)

- The data is only updated at night to see current numbers [follow this link](https://www.arcgis.com/apps/opsdashboard/index.html)

Will depend on a nightly/beta rustc till 2020-03-12

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

## Examples
List the top 10 affected countries
```sh
./target/release/covid19 --limit 10
City     State   Country         3/7/20   3/8/20
                 Japan           461      502
                 US              416      538
                 Spain           500      673
                 Others          696      696
                 Germany         799      1,040
                 France          949      1,126
                 Iran            5,823    6,566
                 South Korea     7,041    7,314
                 Italy           5,883    7,375
                 Mainland China  80,652   80,699
Summary  ------  -------         103,220  106,529
```

Get a summary of the cases in Italy, UK, South Korea, Iran France and Germany for the last month (weekly intervals)
```sh
./target/release/covid19 --num-cols 4 --skip 7 Italy UK "South Korea" Iran France Germany
City     State   Country      2/16/20  2/23/20  3/1/20  3/8/20
                 UK           9        9        36      273
                 Germany      16       16       130     1,040
                 France       12       12       130     1,126
                 Iran         0        43       978     6,566
                 South Korea  29       602      3,736   7,314
                 Italy        3        155      1,694   7,375
Summary  ------  -------      69       837      6,704   23,694
```


Get the top 3 affected states in the US
```sh
./target/release/covid19 --limit 3 --num-cols 4 --states US
City     State   Country  3/4/20  3/5/20  3/6/20  3/7/20
         NY      US       11      23      36      76
         CA      US       35      51      59      82
         WA      US       39      70      83      107
Summary  ------  -------  85      144     178     265
```
