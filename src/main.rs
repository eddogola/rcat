use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::io::BufReader;
use std::process;

const OPTIONS: [&str;4] = [
    "n", // number lines in each file independently
    "b", // number nonblank lines
    "e", // show ends - display `$` at end of each line
    "h", // help
    ];

fn handle_filename(filename: &str) {
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
                println!("{}", content.trim());
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn handle_option(option: &str) {
    if !OPTIONS.contains(&option) {
        eprintln!("rcat: illegal option -- {}", option);
        println!("rcat [-benh] [file ...]");
        process::exit(0);
    }
}

fn main() {
    // use env::args to panic when invalid unicode is entered
    // don't trust user input
    let arguments = env::args().collect::<Vec<String>>();

    if arguments.len() <= 1 {
        panic!("Please provide at least one filename")
    } else {
        // read from the second argument...
        // if argument doesn't begin with `-`
        // it's a filename
        // pass it and all that follow to handle_filename
        
        // is_filename flag 
        // when true, everything that follows should be treated as a filename
        let mut is_filename = false;
        for arg in &arguments[1..] {
            if !arg.starts_with('-') {
                is_filename = true;
            }

            if is_filename {
                handle_filename(arg);
            }
            // else, handle options
        }
    }
}
