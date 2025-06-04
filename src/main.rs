use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

/// Program to build an html table from a list of lines with optional separator
/// and conversely to convert html table to CSV (using comma or any separator character)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of input file, relative to current path
    #[arg(short, long)]
    input: String,

    /// Path of output file, relative to current path
    #[arg(short, long)]
    output: String,

    /// Character to be used as separator inside lines of input file
    #[arg(short, long, default_value_t = ' ')]
    separator: char,

    /// Reverse mode (html to plain text with separator), default=false
    #[arg(short, long, default_value_t = false)]
    reverse: bool,
}

fn main() {
    let args = Args::parse();
    println!("Input file: {}", args.input);
    println!("Output file: {}", args.output);
    println!("Separator: '{}'", args.separator);
    println!("Reverse mode: '{}'", args.reverse);

    if !Path::new(&args.input).exists() {
        println!("Error: input file {} does not exist", args.input);
        return;
    }

    if !args.reverse {
        let readresult: Vec<String> = read_lines(&args.input);
        create_table(readresult, &args.output, args.separator);
    } else {
        parse_html(&args.input, &args.output, args.separator);
    }
}

/// Read input file as a vec of Strings
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Error while reading input file")
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

/// Create html table from csv (or equivalent with other separators)
fn create_table(input: Vec<String>, outfile: &str, sep: char) {
    let mut ofil = File::create(outfile).expect("Cannot create output file");
    writeln!(ofil, "<table class='rustgen'>").expect("Cannot write to file");

    for line in input {
        writeln!(ofil, "<tr>").expect("Cannot write to file");
        let cells = line.split(sep);
        for cell in cells {
            write!(ofil, "<td>{}</td>", cell).expect("Cannot write cell to file");
        }
        writeln!(ofil, "").expect("Cannot write to file");
        writeln!(ofil, "</tr>").expect("Cannot write to file");
    }

    writeln!(ofil, "</table>").expect("Cannot write to file");
}

/// Parse html table and get csv (or equivalent with other separators)
fn parse_html(infile: &str, outfile: &str, sep: char) {
    // 1. read infile as a String:
    let mut content = read_to_string(infile).expect("Failed to read input file");

    // 2. eliminate anything before <table..
    if let Some(i) = content.find("<table") {
        content = content.get(i..).unwrap().to_string();
    } else {
        println!("No <table> found in input file");
        return;
    }

    // 3. create output file
    let mut ofil = File::create(outfile).expect("Cannot create output file");

    // 4. loop on lines
    while let Some(_i) = content.find("<tr") {
        let mut line = String::new();

        // 5. loop on cells within line:
        while let Some(i) = content.find("<td") {
            let iend = content.find("</tr>").unwrap();
            println!("{} - {}", i, iend);
            if i > iend {
                content = content.get((iend + 5)..).unwrap().to_string();
                break;
            }
            let re = Regex::new(r"<td[^>]*>([^<]*)</td>").unwrap();
            let caps = re.captures(&content).unwrap();
            line.push_str(&caps[1]);
            line.push(sep);
            println!("{}", line);
            let i = content.find("</td>").unwrap();
            content = content.get((i + 5)..).unwrap().to_string();
        }

        // remove last added separator:
        line.pop();

        // write line to file:
        writeln!(ofil, "{}", &line).expect("Failed writing to output file");
        println!("{}", &line);
    }
}
