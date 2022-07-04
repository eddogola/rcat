use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::io::BufReader;

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
    }
}
