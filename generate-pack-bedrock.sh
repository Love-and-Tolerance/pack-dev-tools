#!/usr/bin/env bash

lt_version=$(jq .repos.base.version bedrock.json | tr -d '"')

cd /media/velvetremedy/Server-Backups/releases/tmp
rm -rf *
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Love-and-Tolerance-Bedrock/*
zip -rq9 ../zip-dir-bedrock/"L-T_$lt_version.mcpack" *
echo "bedrock base pack"

rm -rf *
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Seasonal-Textures-Addon-Bedrock/*
zip -rq9 ../zip-dir-bedrock/"L-T_$lt_version-Seasons.mcpack" *
echo "bedrock seasonal pack"

rm -rf *
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Holiday-Textures-Addon-Bedrock/*
zip -rq9 ../zip-dir-bedrock/"L-T_$lt_version-Holiday.mcpack" *
echo "bedrock holiday pack"

rm -rf *
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Music-Addon-Bedrock/*
zip -rq9 ../zip-dir-bedrock/"L-T_$lt_version-Music.mcpack" *
echo "bedrock music pack"

rm -rf *
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Classic-Textures-Addon-Bedrock/*
zip -rq9 ../zip-dir-bedrock/"L-T_$lt_version-Classic.mcpack" *
echo "bedrock Classic pack"

rm -rf *
cp -rt . /media/velvetremedy/Server-Backups/releases/repos/Bronydog-Textures-Addon-Bedrock/*
zip -rq9 ../zip-dir-bedrock/"L-T_$lt_version-Bronydog.mcpack" *
echo "bedrock bronydog pack"
