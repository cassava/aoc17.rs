use std::fs::File;
use std::io::{self, Read};

pub struct ProgramInput {
    data: Option<String>,
}

impl ProgramInput {
    pub fn new(name: &str, default: &str) -> Self {
        let matches = App::new(name)
            .author("Ben Morgan <neembi@gmail.com")
            .arg(Arg::with_name("INPUT")
                .help("Input file, use - for stdin")
                .index(1))
            .get_matches();

        if let Some(input) = matches.value_of("INPUT") {
            if input == "-" {
                ProgramInput {
                    data: None,
                }
            } else {
                // Try to read input as a file.
                let mut f = File::open(input).expect("file not found");
                let mut contents = String::new();
                f.read_to_string(&mut contents).expect("error reading the file");
                ProgramInput {
                    data: Some(contents),
                }
            }
        } else {
            ProgramInput {
                data: Some(String::new(default)),
            }
        }
    }

    pub fn to_str(&mut self) -> &str {
        if let Some(s) = self.data {
            s.as_str()
        } else {
            // Try to read input from stdin.
            println!(":: Reading from stdin...");
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            self.data = Some(buffer)
            self.data.unwrap().as_str()
        }
    }
}
