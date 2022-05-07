#!/usr/bin/env bash

id=("1" "2" "3" "4" "5" "6")

function looper() {
        for ((s = 0 ; s <= $sub_addons - 1 ; s++)); do
        curr_id=$(jq .repos.addons[$a].variants[$s].id assets.json)
        id[$a]=$curr_id
        echo ${id[*]}
    done
}

addons=$(jq '.repos.addons | length' assets.json)
for ((a = 0 ; a <= $addons - 1 ; a++)); do
    sub_addons=$(jq '.repos.addons['$a'].variants | length' assets.json)
    looper
done
