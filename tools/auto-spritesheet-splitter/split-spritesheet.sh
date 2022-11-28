#!/usr/bin/env bash

# start script by typing ./split_spritesheet.sh file.png vertical_tiles horizontal_tiles
# ex: ./split_spritesheet.sh ascii.png 16 16

SPRITE_SIZE_X=$(identify "$1" | awk '{print $3}' | awk -F x '{print $1 / '"$2"'}')
SPRITE_SIZE_Y=$(identify "$1" | awk '{print $3}' | awk -F x '{print $1 / '"$3"'}')

OUTPUT=$(echo "$1" | awk -F '.' '{print $1}')

TILE_X=0
TILE_Y=0

#convert ascii.png -crop 32x32+32+128 test.png
#                 x size, y size, x pos, y pos

until [ $TILE_Y -eq "$3" ]; do
    TILE_X=0
    until [ $TILE_X -eq "$2" ]; do
        X_POS=$((TILE_X * SPRITE_SIZE_X))
        Y_POS=$((TILE_Y * SPRITE_SIZE_Y))
        convert "$1" -crop "$SPRITE_SIZE_X"x"$SPRITE_SIZE_Y"+"$X_POS"+"$Y_POS" -trim -quiet ./output/"$OUTPUT"_"$TILE_Y"-"$TILE_X".png
        TILE_X=$((TILE_X + 1))
    done
    TILE_Y=$((TILE_Y + 1))
done
