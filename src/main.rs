use std::env;
use std::fs;
use std::io::ErrorKind;

fn main() {
    // use env::args to panic when invalid unicode is entered
    // don't trust user input
    let filenames = env::args().collect::<Vec<String>>();

    if filenames.len() <= 1 {
        panic!("Please provide at least one filename")
    } else {
        // read from the second argument...
        // if an error is associated with opening the file
        // show the error and end the process
        
        for filename in &filenames[1..] {
            match fs::read_to_string(filename) {
                Ok(content) => println!("{}", content),
                Err(e) => match e.kind() {
                    // file/directory not found
                    ErrorKind::NotFound => eprintln!("rcat: {filename}: No such file or directory", filename=filename),
                    // is a directory
                    // ErrorKind::IsADirectory => eprintln!("rcat: {filename}: Is a directory", filename=filename),
                    _ => println!("{}", e),
                }
            }
        }
    }
}
