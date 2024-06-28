use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use clap::Parser;

use options::Options;

mod options;

fn main() {
    let options = Options::parse();

    match get_strings(options.file, options.minimum) {
        Ok(strings) => {
            for string in strings {
                println!("{string}")
            }
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

fn get_strings<P: AsRef<Path>>(file: P, minimum: usize) -> io::Result<Vec<String>>{
    let mut strings = vec![];
    let mut run = vec![];
    let mut file = File::open(file)?;
    let mut bucket = [0_u8; 0x100];
    loop {
        let read = file.read(&mut bucket)?;
        for i in 0..read {
            let byte = bucket[i];
            if byte >= 0x20 && byte <= 0x7f {
                run.push(byte);
            } else {
                if run.len() >= minimum {
                    let string = String::from_utf8(run.clone()).unwrap_or_default();
                    strings.push(string);
                }
                run.clear();
            }
        }
        if read < 0x100 { break }
    }
    Ok(strings)
}
