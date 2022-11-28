#!/usr/bin/env bash

id=("1" "2" "3" "4" "5" "6")

ponev=$(jq '.repos.addons[0].variants | length' ./assets/java.json)
ptwov=$(jq '.repos.addons[1].variants | length' ./assets/java.json)
pthreev=$(jq '.repos.addons[2].variants | length' ./assets/java.json)
pfourv=$(jq '.repos.addons[3].variants | length' ./assets/java.json)
pfivev=$(jq '.repos.addons[4].variants | length' ./assets/java.json)
psixv=$(jq '.repos.addons[5].variants | length' ./assets/java.json)
for ((pone = 0 ; pone <= ponev - 1 ; pone++)); do
    for ((ptwo = 0 ; ptwo <= ptwov - 1 ; ptwo++)); do
        for ((pthree = 0 ; pthree <= pthreev - 1 ; pthree++)); do
            for ((pfour = 0 ; pfour <= pfourv - 1 ; pfour++)); do
                for ((pfive = 0 ; pfive <= pfivev - 1 ; pfive++)); do
                    for ((psix = 0 ; psix <= psixv - 1 ; psix++)); do
                        id[0]=$(jq .repos.addons[0].variants[$pone].id ./assets/java.json)
                        id[1]=$(jq .repos.addons[1].variants[$ptwo].id ./assets/java.json)
                        id[2]=$(jq .repos.addons[2].variants[$pthree].id ./assets/java.json)
                        id[3]=$(jq .repos.addons[3].variants[$pfour].id ./assets/java.json)
                        id[4]=$(jq .repos.addons[4].variants[$pfive].id ./assets/java.json)
                        id[5]=$(jq .repos.addons[5].variants[$psix].id ./assets/java.json)
                        ./generate-pack.sh ${id[*]}
                    done
                done
            done
        done
    done
done
