#!/usr/bin/env bash

filename=$(echo L_T-$1$2$3$4$5$6.zip | tr -d  '"')
id=($1 $2 $3 $4 $5 $6)
cd /media/velvetremedy/Server-Backups/releases/tmp
rm -rf *
DATE=$(date +%Y-%m-%d-%k-%M-%S | tr -d ' ')
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Love-and-Tolerance/{assets,LICENSE,pack.png,pack.mcmeta,README.md}
echo "$DATE: Love & Tolerance base" > config.log
echo ${id[0]}
if [[ "${id[0]}" == *"a"* ]]; then
    DATE=$(date +%Y-%m-%d-%k-%M-%S | tr -d ' ')
    cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Autumn-Addon/assets
    echo "$DATE: Autumn addon" >> config.log
    echo "test"
fi