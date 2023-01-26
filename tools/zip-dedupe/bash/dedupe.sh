#!/usr/bin/env sh

find "$1" -name *.zip > input.txt
mkdir ./files
echo "" > output.txt

LINES=$(wc -l < "input.txt")
CURR_LINE=1

until [ "$CURR_LINE" -eq "$LINES" ]; do
    FILE=$(sed -n "$CURR_LINE"p ./input.txt)
    ORIGINAL_HASH=$(sha256sum "$FILE")
    NAME=$(echo "$FILE" | awk -F '/' '{print $NF}')
    unzip "$FILE" -d ./files
    zip -0 -r files.zip files
    LVL_HASH=$(sha256sum "files.zip" | awk '{print $1}')
    chmod -R 777 ./files
    rm -r ./files files.zip
    echo "$LVL_HASH" "$ORIGINAL_HASH" >> output.txt
    CURR_LINE=$((CURR_LINE + 1))
done

sort output.txt -o dupes.txt
