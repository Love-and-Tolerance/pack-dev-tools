#!/usr/bin/env bash

# id=("1" "2" "3" "4" "5" "6")

# addons=$(jq '.repos.addons | length' assets.json)
# for ((a = 0 ; a <= $addons - 1 ; a++)); do
#     sub_addons=$(jq '.repos.addons['$a'].variants | length' assets.json)
#     for ((s = 0 ; s <= $sub_addons - 1 ; s++)); do
#         curr_id=$(jq .repos.addons[$a].variants[$s].id assets.json)
#         id[$a]=$curr_id
#         for ((n = $a ; n <= $addons - 1 ; n++)); do
#             curr_id=$(jq .repos.addons[$a].variants[$s].id assets.json)
#             id[$a]=$curr_id
#             echo ${id[*]}
#         done
#     done
# done

echo "" > id.log

ponev=$(jq '.repos.addons[0].variants | length' assets.json)
ptwov=$(jq '.repos.addons[1].variants | length' assets.json)
pthreev=$(jq '.repos.addons[2].variants | length' assets.json)
pfourv=$(jq '.repos.addons[3].variants | length' assets.json)
pfivev=$(jq '.repos.addons[4].variants | length' assets.json)
psixv=$(jq '.repos.addons[5].variants | length' assets.json)
for ((pone = 1 ; pone <= ponev ; pone++)); do
    for ((ptwo = 1 ; ptwo <= ptwov ; ptwo++)); do
        for ((pthree = 1 ; pthree <= pthreev ; pthree++)); do
            for ((pfour = 1 ; pfour <= pfourv ; pfour++)); do
                for ((pfive = 1 ; pfive <= pfivev ; pfive++)); do
                    for ((psix = 1 ; psix <= psixv ; psix++)); do
                        echo $pone $ptwo $pthree $pfour $pfive $psix >> id.log
                    done
                done
            done
        done
    done
done