# Split Sprite-sheet

This shell script splits sprite-sheets into individual images.

### Dependencies:

`awk`  
`echo`  
`imagemagick`

### How to use.

The command to run the script is as follows:
```
./slit_spritesheet.sh ascii.png 16 16
```
The first part being the script itself, second being the file you want to split, and the third and fourth being how many columns and rows the image has respectively.

### Explanation of the code.

To start all shell scripts we use a she-bang followed by what shell to use:

```sh
#!/usr/bin/sh
```

After this we are going to set some variable, sprite size x/y are calculated by taking the image height and wight and dividing by the columns/rows the user entered to run the script. Output sets the output filename to be the same as input. Tile x/y are set to 0.

```sh
SPRITE_SIZE_X=$(identify "$1" | awk '{print $3}' | awk -F x '{print $1 / '"$2"'}')
SPRITE_SIZE_Y=$(identify "$1" | awk '{print $3}' | awk -F x '{print $1 / '"$3"'}')

OUTPUT=$(echo "$1" | awk -F '.' '{print $1}')

TILE_X=0
TILE_Y=0
```



```sh
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
```
