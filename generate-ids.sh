#!/usr/bin/env bash

addons=$(jq '.repos.addons | length' assets.json)
for ((a = 0 ; a <= $addons - 1 ; a++)); do
    sub_addons=$(jq '.repos.addons['$a'].variants | length' assets.json)
    for ((s = 0 ; s <= $sub_addons - 1 ; s++)); do
        sub_addons=$(jq '.repos.addons['$a'].variants | length' assets.json)
        echo $addons $sub_addons
    done
done
