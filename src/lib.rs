pub mod knot;

extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::io::{self, Read};

pub struct ProgramInput {
    data: Option<String>,
    print: bool,
}

impl ProgramInput {
    pub fn new(name: &str, default: &str) -> Self {
        let matches = App::new(name)
            .author("Ben Morgan <neembi@gmail.com")
            .arg(Arg::with_name("INPUT")
                .help("Input file, use - for stdin")
                .index(1))
            .arg(Arg::with_name("verbose")
                 .short("v")
                 .long("verbose")
                 .multiple(true)
                 .help("Print the default input used"))
            .get_matches();

        if let Some(input) = matches.value_of("INPUT") {
            if input == "-" {
                // We will read stdin later.
                ProgramInput {
                    data: None,
                    print: matches.occurrences_of("verbose") > 2,
                }
            } else {
                // Try to read input as a file.
                let mut f = File::open(input).expect("file not found");
                let mut contents = String::new();
                f.read_to_string(&mut contents).expect("error reading the file");
                ProgramInput {
                    data: Some(contents),
                    print: matches.occurrences_of("verbose") > 1,
                }
            }
        } else {
            ProgramInput {
                data: Some(String::from(default.trim())),
                print: matches.occurrences_of("verbose") > 0,
            }
        }
    }

    pub fn to_str(&mut self) -> &str {
        if self.data.is_none() {
            println!(":: Reading from stdin...");
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            self.data = Some(buffer);
        }
        if self.print {
            println!(":: Program input is:\n{}\n", self.data.as_ref().unwrap());
            self.print = false;
        }
        self.data.as_ref().unwrap().as_str()
    }
}
