#!/usr/bin/env bash

id=("1" "2" "3" "4" "5" "6")

addons=$(jq '.repos.addons | length' assets.json)
curr_cat=0

function addon_loop() {
    for ((a = $curr_cat ; a <= $addons - 1 ; a++)); do
        curr_subadd=0
        sub_addon_loop
    done
}

function sub_addon_loop() {
    sub_addons=$(jq '.repos.addons['$a'].variants | length' assets.json)
    for ((s = $curr_subadd ; s <= $sub_addons - 1 ; s++)); do
        curr_id=$(jq .repos.addons[$a].variants[$s].id assets.json)
        id[$a]=$curr_id
        echo ${id[*]}
        next_cat=$((a + 1))
        curr_cat=$((a + 1))
        # if there are more addon catagories, do this:
        if [[ $next_cat < $addons ]]; then
            curr_cat=$((a + 1))
            addon_loop
        # if there are no more addon catagories, quit:
        elif [[ $curr_cat = $addons ]]; then
            exit;
        fi
        a=$next_cat
        sub_addon_loop
    done
}

addon_loop
