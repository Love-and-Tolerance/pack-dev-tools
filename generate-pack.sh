#!/usr/bin/env bash

filename=$(echo L_T-$1$2$3$4$5$6.zip | tr -d  '"')
echo $filename
cd /media/velvetremedy/Server-Backups/releases/tmp
rm -rf *
DATE=$(date +%Y-%m-%d-%k-%M-%S | tr -d ' ')
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Love-and-Tolerance/{assets,LICENSE,pack.png,pack.mcmeta,README.md}
echo "$DATE: Love & Tolerance base" > config.log

pone=$(echo $1 | tr -d '"')
ptwo=$(echo $2 | tr -d '"')
pthree=$(echo $3 | tr -d '"')
pfour=$(echo $4 | tr -d '"')
pfive=$(echo $5 | tr -d '"')
psix=$(echo $6 | tr -d '"')

if [[ "$psix" == "b" ]]; then
    DATE=$(date +%Y-%m-%d-%k-%M-%S | tr -d ' ')
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Bronydog-Textures-Addon/assets
    echo "$DATE: Bronydog textures addon" >> config.log
fi

if [[ "$pone" == "a" ]]; then
    DATE=$(date +%Y-%m-%d-%k-%M-%S | tr -d ' ')
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Autumn-Addon/assets
    echo "$DATE: Autumn addon" >> config.log
fi


zip -rq9 ../zip-dir/$filename *