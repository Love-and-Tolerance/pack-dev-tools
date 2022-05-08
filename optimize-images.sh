#!/usr/bin/env bash

cd /media/velvetremedy/Server-Backups/releases/repos

for file in `find -name '*.png'`
do
  pngcrush -reduce -brute $file /media/velvetremedy/Server-Backups/releases/tmp/crushed.png
  mv /media/velvetremedy/Server-Backups/releases/tmp/crushed.png $file
done