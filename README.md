# rust command line program to convert csv files to html and reverse

## Directions for use
Program to build an html table from a list of lines with optional separator and conversely to convert html table to CSV (using comma or any separator character)

Usage: htmltable [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>          Path of input file, relative to current path
  -o, --output <OUTPUT>        Path of output file, relative to current path
  -s, --separator <SEPARATOR>  Character to be used as separator in lines of input file [default: " "]
  -r, --reverse                Reverse mode (html to plain text with separator), default=false
  -h, --help                   Print help
  -V, --version                Print version

## Example:
One sample file romaji.txt is provided

To test the program, try

cargo run -- -i romaji.txt -o romaji.html

Then try to convert the resulting file to CSV:

cargo run -- -i romaji.html -o romaji.csv -s , -r

I used romaji.html, together with some CSS, in my [hkana program](https://eludev.fr/hkana/)
