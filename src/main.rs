use std::env;
use std::fs;
use std::io::BufRead;
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
    number_nonblanks: bool, // number nonblank lines in each file independently
    display_line_ends: bool, // show ends - display `$` at end of each line
}

impl Config {

    fn new(number: bool, number_nonblanks: bool, display_line_ends: bool) -> Config {
        Config{
            number,
            number_nonblanks,
            display_line_ends,
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
                let lines = BufReader::new(file).lines();

                let mut line_count = 1;
                for line in lines {
                    match line {
                        Ok(content) => {
                            let mut pre_line = String::new();
                            let mut post_line = String::new();

                            if (config.number_nonblanks // if number nonblanks option is true
                                ||                      // or
                                (config.number_nonblanks && config.number)) // if both number lines and number non blanks option are true
                                &&                                          // and
                                !content.is_empty() {                       // the line is nonblank
                                // show line number
                                pre_line = line_count.to_string();
                                // increment line count
                                line_count += 1;
                            } else if config.number && !config.number_nonblanks {
                                pre_line = line_count.to_string();
                                // increment line count
                                line_count += 1;
                            }

                            // show line ends
                            if config.display_line_ends {
                                post_line = String::from("$");
                            }

                            let new_line = format!("{} {}{}", 
                                                                pre_line,
                                                                content,
                                                                post_line);

                            println!("{}", new_line)
                        },
                        Err(e) => eprintln!("Error reading line: {}", e),
                    }
                }
                
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
            config.number_nonblanks = true;
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
