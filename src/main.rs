use std::fs::File;
use std::io::{self, prelude::*};

mod intcoder;

fn read(filename: &str) -> Result<Vec<i64>,std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents.split(',').map( |x| {
        match x.parse() {
            Ok(x) => x,
            _ => -1
        }
    }).collect())
}

fn main() -> io::Result<()> {
    let prog = read("program.txt")?;

    let mut icoder = intcoder::Intcode::new(&prog);
    let answer = icoder.run();

    Ok(())
}
