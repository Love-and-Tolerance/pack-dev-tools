#!/usr/bin/env bash

declare -a trigger_pixels=("0-0" "1-0" "1-1" "2-0" "2-1" "3-0")

if [ -d "./zompony" ]; then
    rm -rf ./zompony
fi

if [ -d "./Zompony" ]; then
    rm -rf ./Zompony
fi

mkdir ./zompony

find . -path ./zompony -prune -o -name '*.png' -exec cp --parents \{\} ./zompony \;
cd ./zompony
rm ./zompony_overlay.png

rename 'y/ /_/' *
rename 'y/A-Z/a-z/' *

for file in `find -name '*.png'`
do
    output=$(echo "$file" | sed s/.png/_zompony.png/g)
    cp $file $output
    palette=$(convert $file -unique-colors txt: | awk -F ' ' '{print  $2}' | tr -d [:alpha:] | tr -d '()' | awk -F ',' '{print $1" "$2" "$3}')
    echo "$palette" | while read color
    do
        if [ -z "$color" ]; then
            continue
        fi
        r=$(echo $color | awk -F ' ' '{print $1}')
        g=$(echo $color | awk -F ' ' '{print $2}')
        b=$(echo $color | awk -F ' ' '{print $3}')
        average=$(awk 'BEGIN{print int(('$r' + '$g' + '$g') / 3)}')
        adjusted_r=$(awk 'BEGIN{print '$average' + (('$r' - '$average') / 2)}')
        adjusted_g=$(awk 'BEGIN{print '$average' + (('$g' - '$average') / 2)}')
        adjusted_b=$(awk 'BEGIN{print '$average' + (('$b' - '$average') / 2)}')
        convert $output -fill 'rgb('$adjusted_r','$adjusted_g','$adjusted_b')' -opaque 'rgb('$r','$g','$b')' $output
    done
    for pixel in ${trigger_pixels[@]}
    do
        x=$(cut -d'-' -f1 <<<"$pixel")
        y=$(cut -d'-' -f2 <<<"$pixel")
        hex=$(convert $file -format "%[hex:u.p{$x,$y}]" info:)
        convert $output -fill "#$hex" -draw "color $x,$y point" $output
    done
    rm $file
    convert $output ../zompony_overlay.png -gravity center -compose over -composite $output
    mv $output $file
    echo $file
done

cd ..
mv zompony Zompony

basedirs=$(ls -d */ | sort | awk '!/Zompony/')
zomponydirs=$(cd ./Zompony/ ; ls -d */)

cd ./Zompony

IFS="/"
myarray=(`echo $basedirs | tr '/' '\n'`)
echo $myarray > base.txt
myarray=(`echo $zomponydirs | tr '/' '\n'`)
echo $myarray > zom.txt

while IFS= read -r base && IFS= read -r zom <&3; do
    old=$(echo $zom | xargs)
    new=$(echo $base | xargs)
    mv $old $new
done < base.txt 3< zom.txt

rm -rf base.txt zom.txt

