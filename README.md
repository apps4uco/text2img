


Started on this the idea being

## Basic

text2img "myemail@domain.com" email.png

## Font size 

12 point font

text2img -p 12 "myemail@domain.com" email.png


text2img --points 12 --font DejaVu "myemail@domain.com" email.png

## Read stdin 

echo data.txt | text2img data.png

## Alignment 

echo data.txt | text2img --justify data.png
--center --left --right

# Alternatives

## text2image Tesseract

text2image

## ImageMagick

convert -size 360x360 xc:white -font "FreeMono" -pointsize 12 -fill black -draw @ascii.txt image.png


## ascii.txt:

text 15,15 "                 .88888888:. 

convert -size 360x360 xc:white -font "FreeMono" -pointsize 12 -fill black -annotate +15+15 "@ascii.txt" image.png

convert -list font | grep Font:


    #!/bin/sh

    $1 | convert -background black -fill white \
    -font Helvetica -pointsize 14 \
    -border 10 -bordercolor black \
    label:@- $2

# Font

Currently only Roboto is supported.

The Roboto font used was copied from

/usr/share/fonts/truetype/roboto/unhinted/RobotoTTF/Roboto-Regular.ttf