use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

/// Program to build an html table from a list of lines with optional separator
/// and conversely to convert html table to CSV (using comma or any separator character)
#[derive(Parser, Debug)]
#[command(version)]
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
        .expect("Failed to open input file") // print message, then panic
        .lines() // split the string into an iterator of string slices Lines<'_>
        .map(String::from) // make each slice into a string
        .collect() // transform Lines<'_> into a Vec<String>
}

/// Create html table from csv (or equivalent with other separators)
fn create_table(input: Vec<String>, outfile: &str, sep: char) {
    let mut ofil = File::create(outfile).expect("Cannot create output file");
    writeln!(ofil, "<table class='rustgen'>").expect("Cannot write to file");

    // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    for line in input {
        writeln!(ofil, "<tr>").expect("Cannot write to file");
        let cells = line.split(sep);
        for cell in cells {
            write!(ofil, "<td>{}</td>", cell).expect("Cannot write cell to file");
        }
        writeln!(ofil, "").expect("Cannot write to file"); // add line feed (writeln)
        writeln!(ofil, "</tr>").expect("Cannot write to file");
    }

    writeln!(ofil, "</table>").expect("Cannot write to file");
}

/// Parse html table and get plain text file with separator between elements
/// and one line for each <tr...>...</tr> in html table
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

    // 3. eliminate anything after </table>
    if let Some(i) = content.find("</table>") {
        content = content.get(..i).unwrap().to_string();
    } else {
        println!("No </table> found in input file");
        return;
    }

    // 4. create output file
    let mut ofil = File::create(outfile).expect("Cannot create output file");

    // 5. split content as an iterator on &str lines
    content = content.replace("\n", ""); // remove any line feed chars
    content = content.replace("</tr>", "\n");
    let tablelines = content.lines();

    // 6. prepare regex for cell parsing:
    // The (.*) group is the content of the html table cell
    let re = Regex::new(r"<td[^>]*>(.*)$").unwrap();

    // 7. loop on tablelines
    for tableline in tablelines {
        // skip if tableline contains no cell element:
        if let None = tableline.find("<td") {
            continue;
        }

        let mut line = String::new(); // line will be a line of output file

        // 8. split tableline by </td>:
        let cells: Vec<&str> = tableline.split("</td>").collect();

        // 9. loop on cells within tableline:
        for cell in cells {
            // 10. capture cell content and push it to line
            if let Some(caps) = re.captures(cell) {
                if caps.len() == 2 {
                    line.push_str(&caps[1]); // push cell text to line
                } else {
                    println!("Could not capture cell text, line={}", tableline);
                    return;
                }
            } else {
                // the element does not conform with html td syntax, ignore it
                // this may be due to whitespace between </td> and </tr> in the html file
                continue;
            }
            line.push(sep); //  add separator
        }

        // remove last added separator:
        line.pop();

        // write line to file:
        writeln!(ofil, "{}", &line).expect("Failed writing to output file");
        println!("{}", &line);
    }
    // No need to close files: they get closed when going out of scope
    // or in case of panic
}
