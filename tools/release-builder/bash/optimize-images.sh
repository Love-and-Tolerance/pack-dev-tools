#!/usr/bin/env bash

cd /media/velvetremedy/Server-Backups/releases/repos

for file in `find -name '*.png'`
do
  oxipng -o 6 -i 1 --strip safe $file
done