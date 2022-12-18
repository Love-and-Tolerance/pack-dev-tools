#!/usr/bin/env bash

if [ -d "./assets" ]; then
    rm -rf ./assets
fi

find . -name "*.txt" -type f -delete
find . -name "*.md" -type f -delete
find . -name "*.json" -type f -delete
rename 'y/ /_/' * && rename 'y/A-Z/a-z/' *
rename 'y/ /_/' zompony/* && rename 'y/A-Z/a-z/' zompony/*

basedirs=$(ls -d */ | sort | awk '!/zompony/')
zomponydirs=$(cd ./zompony/ ; ls -d */)

IFS="/"
myarray=(`echo $basedirs | tr '/' '\n'`)
echo $myarray > base.txt
myarray=(`echo $zomponydirs | tr '/' '\n'`)
echo $myarray > zom.txt

mkdir -p assets/minelittlepony/textures/entity/{pony,zompony}

while IFS= read -r base && IFS= read -r zom <&3; do
    pony=$(echo $base | xargs)
    zompony=$(echo $zom | xargs)
    mv $pony assets/minelittlepony/textures/entity/pony/$pony
    mv zompony/$zompony assets/minelittlepony/textures/entity/zompony/$zompony
done < base.txt 3< zom.txt

rm -rf base.txt zom.txt ./zompony
