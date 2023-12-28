mod sha912;

use sha912::sha912;
use std::fs::File;
use std::io::{ Read, Write };
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Provide a file and mode.\nUsage: {} filename [true|false]", args[0]);
        return;
    }

    let filename = &args[1];
    let mode = args[2].parse::<bool>().unwrap_or(true);

    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let converted = sha912(&content, mode);

            match File::create(filename) {
                Err(e) => panic!("Couldn't create {}: {}", filename, e),
                Ok(mut file) => {
                    match file.write_all(converted.as_bytes()) {
                        Err(e) => panic!("Couldn't write to {}: {}", filename, e),
                        Ok(_) => println!("Processed {}!", filename)
                    };
                }
            };
        },
        Err(e) => println!("Error opening file {}: {}", filename, e)
    };
}