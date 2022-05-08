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

id=("1" "2" "3" "4" "5" "6")

rm -rf id.log

ponev=$(jq '.repos.addons[0].variants | length' assets.json)
ptwov=$(jq '.repos.addons[1].variants | length' assets.json)
pthreev=$(jq '.repos.addons[2].variants | length' assets.json)
pfourv=$(jq '.repos.addons[3].variants | length' assets.json)
pfivev=$(jq '.repos.addons[4].variants | length' assets.json)
psixv=$(jq '.repos.addons[5].variants | length' assets.json)
for ((pone = 0 ; pone <= ponev - 1 ; pone++)); do
    for ((ptwo = 0 ; ptwo <= ptwov - 1 ; ptwo++)); do
        for ((pthree = 0 ; pthree <= pthreev - 1 ; pthree++)); do
            for ((pfour = 0 ; pfour <= pfourv - 1 ; pfour++)); do
                for ((pfive = 0 ; pfive <= pfivev - 1 ; pfive++)); do
                    for ((psix = 0 ; psix <= psixv - 1 ; psix++)); do
                        id[0]=$(jq .repos.addons[0].variants[$pone].id assets.json)
                        id[1]=$(jq .repos.addons[0].variants[$ptwo].id assets.json)
                        id[2]=$(jq .repos.addons[0].variants[$pthree].id assets.json)
                        id[3]=$(jq .repos.addons[0].variants[$pfour].id assets.json)
                        id[4]=$(jq .repos.addons[0].variants[$pfive].id assets.json)
                        id[5]=$(jq .repos.addons[0].variants[$psix].id assets.json)
                        echo ${id[*]}
                    done
                done
            done
        done
    done
done