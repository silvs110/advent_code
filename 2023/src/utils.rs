use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::error::Error;

pub fn read_file(filename:&str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let path = Path::new(filename);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, <dyn Error>::to_string(&why)),
        Ok(file) => file,
    };
    Ok(BufReader::new(file).lines())
}


