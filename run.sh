#!/bin/bash

cat ~/data/COVID-19/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv | xsv select 1,2,46,47 | cargo run -- $@
