#!/usr/bin/env bash

cd /media/velvetremedy/Server-Backups/releases/repos

for file in `find -name '*.png'`
do
  optipng -o7 $file
done