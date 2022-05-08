#!/usr/bin/env bash

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
                        id[1]=$(jq .repos.addons[1].variants[$ptwo].id assets.json)
                        id[2]=$(jq .repos.addons[2].variants[$pthree].id assets.json)
                        id[3]=$(jq .repos.addons[3].variants[$pfour].id assets.json)
                        id[4]=$(jq .repos.addons[4].variants[$pfive].id assets.json)
                        id[5]=$(jq .repos.addons[5].variants[$psix].id assets.json)
                        ./generate-pack.sh ${id[*]}
                    done
                done
            done
        done
    done
done