use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

fn parse_markdown_file(_filename: &str) {

    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);

    // Create a path variable from the filename
    let input_filename = Path::new(_filename);

    // Try open file
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");

    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    // Create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();

    // Read the file line-by-line
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                } 
        
                output_line.push_str(&line_contents);
            }
        }


        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");      
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }

        // let line_contents = line.unwrap();
        // let mut first_char: Vec<char> = line_contents
    }

    for t in &tokens {
        println!("{}", t);
    }

    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile = File::create(output_filename).expect("[ ERROR ] Could not create output file!");
    for line in &tokens {
        outfile.write_all(line.as_bytes()).expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] Parsing complete!");
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n", 
        env!("CARGO_PKG_AUTHORS"), 
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation");
            usage();
        },
    }
}
