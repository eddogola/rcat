use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::io::BufReader;
use std::process;

const OPTIONS: [char;3] = [
    'n', // number lines in each file independently
    'b', // number nonblank lines
    'e', // show ends - display `$` at end of each line
    ];

struct Config {
    number: bool, // number lines in each file independently
    number_blanks: bool, // number nonblank lines in each file independently
    display_line_ends: bool, // show ends - display `$` at end of each line
}

impl Config {

    fn new(number: bool, number_blanks: bool, display_line_ends: bool) -> Config {
        Config{
            number,
            number_blanks,
            display_line_ends,
        }
    }

    fn number_lines(&self) {
        if self.number {
            // do something
        }
    }

    fn number_blank_lines(&self) {
        if self.number_blanks {
            // do something
        }
    }

    fn display_line_ends(&self) {
        if self.display_line_ends {
            // do something
        }
    }
}

fn handle_filename(filename: &str, config: &Config) {
    let path = Path::new(filename);
    if !path.exists() {
        eprintln!("rcat: {}: No such file or directory", filename);
    } else if path.is_dir() {
        eprintln!("rcat: {}: Is a directory", filename);
    } else {
        match fs::File::open(path) {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut content = String::new();
                if let Err(e) = buf_reader.read_to_string(&mut content) {
                    eprintln!("{}", e);
                }
                
                println!("{}", content.trim()); // normal print
                
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn handle_option(option: char, config: &mut Config) {
    if !OPTIONS.contains(&option) {
        eprintln!("rcat: illegal option -- {}", option);
        println!("rcat [-ben] [file ...]");
        process::exit(0);
    } else {
        // if `n`, number all lines in each file
        if option == 'n' {
            config.number = true;
        }
        // if `b`, only number non-blank lines
        else if option == 'b' {
            config.number_blanks = true;
        }
        // if `e`, show $ at end of each line
        else if option == 'e' {
            config.display_line_ends = true;
        }
    }
}

fn main() {
    // use env::args to panic when invalid unicode is entered
    // don't trust user input
    let arguments = env::args().collect::<Vec<String>>();

    if arguments.len() <= 1 {
        panic!("Please provide at least one filename");
    } else {
        // read from the second argument...
        // if argument doesn't begin with `-`
        // it's a filename
        // pass it and all that follow to handle_filename
        
        // declare is_filename flag 
        // when true, everything that follows should be treated as a filename
        let mut is_filename = false;

        // initiate config with every argument set to false
        let mut config = Config::new(false, false, false);

        for arg in &arguments[1..] {
            if !arg.starts_with('-') {
                is_filename = true;
            }

            if is_filename {
                handle_filename(arg, &config);
            } else {
                let option = arg.as_bytes()[1] as char;
                handle_option(option, &mut config);
            }
        }
    }
}
