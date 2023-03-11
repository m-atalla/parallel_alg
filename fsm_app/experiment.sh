#!/bin/bash

test_file_name=seed.txt


# make the test file if doesnt exist
if [[ ! -e ./$test_file_name ]]; then
    touch input.txt
    for _ in {1..1000000}
    do
        cat bird >> input.txt
    done 
fi 


if [[ ! -e ./inputs/1_input.txt ]]; then
    mkdir inputs
    for i in {1..16}
    do
        cp -- "./$test_file_name" "./inputs/$(echo $i)_input.txt"
    done
fi


# modifies time output format to elapsed real time (clock time)
export TIMEFORMAT="%R" 


search_term="needle"

seq_time=$({ time grep -rn $search_term inputs/* > /dev/null; } 2>&1)

echo "1,$seq_time" >> output.csv

for i in {2..16..2}
do
    timing=$({ time ls inputs | parallel -j $i 'grep needle inputs/{} > /dev/null'; } 2>&1 )

    echo "$i,$timing" >> output.csv
done
