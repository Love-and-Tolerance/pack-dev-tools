#!/usr/bin/env bash

id=("1" "2" "3" "4" "5" "6")

addons=$(jq '.repos.addons | length' assets.json)
curr_cat=0

function addon_loop() {
    for ((a = 0 ; a <= $addons - 1 ; a++)); do
    sub_addon_loop
    done
}

function sub_addon_loop() {
    sub_addons=$(jq '.repos.addons['$a'].variants | length' assets.json)
    for ((s = 0 ; s <= $sub_addons - 1 ; s++)); do
    curr_id=$(jq .repos.addons[$a].variants[$s].id assets.json)
    id[$a]=$curr_id
    echo $curr_id
    next_cat=$((a + 2))
    if [[ $next_cat > $addons ]]; then
        curr_cat=$((a + 1))
        addon_loop
    fi
    done
}

addon_loop

echo $curr_cat