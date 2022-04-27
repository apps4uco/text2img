# text2img

Command line utility and (soon to be) library.

## Summary

Creates a png image from text, with an easy to use command line tool and/or library.

The idea is to be able to show sensitive information and defeat Optical Character Recognition (OCR).

Useful to protect text from bots and scrapers,
such as email addresses and telephone numbers in web pages,
and even passwords or tokens sent via email.

The image size is calculated automatically so that the text fits,
with a small margin.
The background is transparent so that the image can be used on any color background.


## Options 

You can change the font size (-s)
also to make it harder for bots and scrapers to do OCR on the text 
there is an option (-w) to only fill in a percentage of the pixels at random.

If the text has spaces it must be put in quotes on the command line.

## Help

```
USAGE:
text2img [OPTIONS] --output <OUTPUT> <TEXT>

ARGS:
<TEXT>    Text to render

OPTIONS:
-h, --help               Print help information
-o, --output <OUTPUT>    Output filename. Currently only png format is supported
-s, --size <SIZE>        Point size of text [default: 20]
-V, --version            Print version information
-w, --weight <WEIGHT>    Weight 1 to 100 [default: 100]
```

## Basic Usage

text2img -o email.png "myemail@domain.com"

## Font size 

50 point font

text2img -o email.png -s 50 "myemail@domain.com" 

## Weight 

50 point font and 30% of pixels filled in

text2img -o email.png -s 50 -w 30 "myemail@domain.com"

## Font

Currently only Roboto is supported.

The Roboto font used was copied from

/usr/share/fonts/truetype/roboto/unhinted/RobotoTTF/Roboto-Regular.ttf

# Roadmap Ideas for next version

Submit an issue if you would like a feature, 
or send a PR if you would like to contribute.

## Poisson Disks

Render the text using Poisson Disks to make it even harder for OCR to work.

## Background color

Currently the background is always transparent.

## Change Font 

text2img --points 12 --font DejaVu "myemail@domain.com" email.png

## Read stdin 

echo data.txt | text2img data.png

## Alignment 

echo data.txt | text2img --justify data.png
--center --left --right

# Alternatives to this crate

The alternatives given require the output image size to be specified.

## text2image from the Tesseract project

text2image

## ImageMagick

```
convert -size 360x360 xc:white -font "FreeMono" -pointsize 12 -fill black -draw @ascii.txt image.png
```

## file ascii.txt:

```
text 15,15 "                 text to show
"
```

```
convert -size 360x360 xc:white -font "FreeMono" -pointsize 12 -fill black -annotate +15+15 "@ascii.txt" image.png
```

```
convert -list font | grep Font:
```

```
#!/bin/sh

$1 | convert -background black -fill white \
-font Helvetica -pointsize 14 \
-border 10 -bordercolor black \
label:@- $2
```
