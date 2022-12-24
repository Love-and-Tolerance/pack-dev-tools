#!/usr/bin/env bash

if [ -f ./bedrock.json ]; then
    rm bedrock.json
fi

wget 'https://raw.githubusercontent.com/Love-and-Tolerance/pack-builder-assets/mane/assets/bedrock.json'

lt_version=$(jq .repos.base.version ./bedrock.json | tr -d '"')

mkdir ./{repos,zips}

cd ./zips
rm -rf *
cd ../repos
rm -rf *

function make_pack() {
    if [ $1 == "base" ]; then
        echo $1
        filename=$(jq .repos.$1.filename ../bedrock.json | tr -d '"' | sed s/{version}/$lt_version/g)
        url=$(jq .repos.$1.url ../bedrock.json | tr -d '"')
        folder=$(echo $url | awk -F '/' '{print  $NF}')
        git clone $url
        cd ./$folder
        for file in `find -name '*.png'`
        do
            oxipng -o 6 -i 1 --strip safe $file --fix
        done
        zip -rq9 ../../zips/"$filename" *
        cd ..
    elif [ $1 == addons ]; then
        echo $1 $2
        filename=$(jq .repos.$1[$2].filename ../bedrock.json | tr -d '"' | sed s/{version}/$lt_version/g)
        url=$(jq .repos.$1[$2].url ../bedrock.json | tr -d '"')
        folder=$(echo $url | awk -F '/' '{print  $NF}')
        git clone $url
        cd ./$folder
        for file in `find -name '*.png'`
        do
            oxipng -o 6 -i 1 --strip safe $file --fix
        done
        zip -rq9 ../../zips/"$filename" *
        cd ..
    fi
}

make_pack "base"

addon_count=$(jq '.repos.addons | length' ../bedrock.json)

for ((addon = 0 ; addon <= addon_count - 1 ; addon++)); do
    make_pack "addons" $addon
done



