#!/usr/bin/env bash

lt_version=$(jq .repos.base.version assets.json)
filename=$(echo L-T_$lt_version-$1$2$3$4$5$6.zip | tr -d  '"')

cd /media/velvetremedy/Server-Backups/releases/tmp
rm -rf *
DATE=$(date +%Y-%m-%d-%k-%M-%S | tr -d ' ')
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Love-and-Tolerance/{assets,LICENSE,pack.png,pack.mcmeta,README.md}
echo "$DATE: pack generated" > config.log
echo "Love and Tolerance base" >> config.log

pone=$(echo $1 | tr -d '"')
ptwo=$(echo $2 | tr -d '"')
pthree=$(echo $3 | tr -d '"')
pfour=$(echo $4 | tr -d '"')
pfive=$(echo $5 | tr -d '"')
psix=$(echo $6 | tr -d '"')

if [[ "$psix" == "b" ]]; then
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Bronydog-Textures-Addon/assets
    echo "Bronydog textures addon" >> config.log
fi

if [[ "$pfive" == "c" ]]; then
    grep minecraft ../repos/Classic-Textures-Addon/assets/minecraft/lang/eq_eq-full.json | awk -F ':' '{print $1}' > mc-names.txt
    grep minecraft ../repos/Classic-Textures-Addon/assets/minecraft/lang/eq_eq-full.json | awk -F ':' '{print $2}' > lt-names.txt
    while IFS= read -r mc_name && IFS= read -r lt_name <&3; do
        if [[ $lt_name == *","* ]]; then
            sed -i "/$mc_name/c\\$mc_name:$lt_name" assets/minecraft/lang/eq_eq-full.json
        else
            sed -i "/$mc_name/c\\$mc_name:$lt_name," assets/minecraft/lang/eq_eq-full.json
        fi
    done < mc-names.txt 3< lt-names.txt
    rm ./{mc-names.txt,lt-names.txt}
    grep minecraft ../repos/Classic-Textures-Addon/assets/minecraft/lang/eq_eq-min.json | awk -F ':' '{print $1}' > mc-names.txt
    grep minecraft ../repos/Classic-Textures-Addon/assets/minecraft/lang/eq_eq-min.json | awk -F ':' '{print $2}' > lt-names.txt
    while IFS= read -r mc_name && IFS= read -r lt_name <&3; do
        if [[ $lt_name == *","* ]]; then
            sed -i "/$mc_name/c\\$mc_name:$lt_name" assets/minecraft/lang/eq_eq-min.json
        else
            sed -i "/$mc_name/c\\$mc_name:$lt_name," assets/minecraft/lang/eq_eq-min.json
        fi
    done < mc-names.txt 3< lt-names.txt
    rm ./{mc-names.txt,lt-names.txt}
    rclone copy ../repos/Classic-Textures-Addon/assets/ ./assets --exclude=/minecraft/lang/** &>/dev/null
    echo "Classic textures addon" >> config.log
fi

if [[ "$pthree" == "s" ]]; then
    cd ../repos/3d-Models-Addon/
    git switch simple &>/dev/null
    cd ../../tmp/
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/3d-Models-Addon/assets
    echo "Simple 3d Models addon" >> config.log
fi

if [[ "$pthree" == "c" ]]; then
    cd ../repos/3d-Models-Addon/
    git switch complex &>/dev/null
    cd ../../tmp/
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/3d-Models-Addon/assets
    echo "Complex 3d Models addon" >> config.log
fi

if [[ "$pfour" == "h" ]]; then
    if [[ "$pthree" == "s" ]]; then
        cd ../repos/Hearts-and-Hooves-Addon/
        git switch simple-models &>/dev/null
        cd ../../tmp/
        cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Hearts-and-Hooves-Addon/assets
        echo "Hearts and Hooves addon with simple 3d models" >> config.log
    else
        cd ../repos/Hearts-and-Hooves-Addon/
        git switch mane &>/dev/null
        cd ../../tmp/
        cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Hearts-and-Hooves-Addon/assets
        echo "Hearts and Hooves addon" >> config.log
    fi
fi

if [[ "$ptwo" == "x" ]]; then
    grep minecraft ../repos/Music-side-B/assets/minecraft/lang/eq_eq-full.json | awk -F ':' '{print $1}' > mc-names.txt
    while IFS= read -r mc_name; do
        sed -i "/$mc_name/d" assets/minecraft/lang/eq_eq-full.json
    done < mc-names.txt
    rm ./mc-names.txt
    grep minecraft ../repos/Music-side-B/assets/minecraft/lang/eq_eq-min.json | awk -F ':' '{print $1}' > mc-names.txt
    while IFS= read -r mc_name; do
        sed -i "/$mc_name/d" assets/minecraft/lang/eq_eq-full.json
    done < mc-names.txt
    rm -r ./mc-names.txt ./assets/minecraft/sounds/records
fi

if [[ "$ptwo" == "b" ]]; then
    grep minecraft ../repos/Music-side-B/assets/minecraft/lang/eq_eq-full.json | awk -F ':' '{print $1}' > mc-names.txt
    grep minecraft ../repos/Music-side-B/assets/minecraft/lang/eq_eq-full.json | awk -F ':' '{print $2}' > lt-names.txt
    while IFS= read -r mc_name && IFS= read -r lt_name <&3; do
        if [[ $lt_name == *","* ]]; then
            sed -i "/$mc_name/c\\$mc_name:$lt_name" assets/minecraft/lang/eq_eq-full.json
        else
            sed -i "/$mc_name/c\\$mc_name:$lt_name," assets/minecraft/lang/eq_eq-full.json
        fi
    done < mc-names.txt 3< lt-names.txt
    rm ./{mc-names.txt,lt-names.txt}
    grep minecraft ../repos/Music-side-B/assets/minecraft/lang/eq_eq-min.json | awk -F ':' '{print $1}' > mc-names.txt
    grep minecraft ../repos/Music-side-B/assets/minecraft/lang/eq_eq-min.json | awk -F ':' '{print $2}' > lt-names.txt
    while IFS= read -r mc_name && IFS= read -r lt_name <&3; do
        if [[ $lt_name == *","* ]]; then
            sed -i "/$mc_name/c\\$mc_name:$lt_name" assets/minecraft/lang/eq_eq-min.json
        else
            sed -i "/$mc_name/c\\$mc_name:$lt_name," assets/minecraft/lang/eq_eq-min.json
        fi
    done < mc-names.txt 3< lt-names.txt
    rm ./{mc-names.txt,lt-names.txt}
    rclone copy ../repos/Music-side-B/assets/ ./assets --exclude=/minecraft/lang/** --quiet
    rclone copy ../repos/Music-side-B/LICENSE . --quiet
    echo "Music side B" >> config.log
fi

if [[ "$pone" == "a" ]]; then
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Autumn-Addon/assets
    echo "Autumn addon" >> config.log
fi

if [[ "$pone" == "s" ]]; then
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Spring-Addon/assets
    echo "Spring addon" >> config.log
fi

if [[ "$pone" == "w" ]]; then
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Winter-Addon/assets
    echo "Winter addon" >> config.log
fi

zip -rq9 ../zip-dir/$filename *

echo $filename
