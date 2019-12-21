use std::fs::File;
use std::io::{self, prelude::*};
use std::time::{Instant};
use std::collections::HashMap;
use std::thread;
use std::time;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear};

mod intcoder;
mod asciicoder;


fn main() -> io::Result<()> {
    let now = Instant::now();
    let prog = read("program.txt")?; 
    let mut computer = intcoder::Intcode::new(&prog);

    asciicoder::game(&mut computer);

    // TIMING
    let duration = (now.elapsed().subsec_millis() as u128) + 1000*(now.elapsed().as_secs() as u128);

    println!("it took {}ms", duration);

    Ok(())
}


fn read(filename: &str) -> Result<Vec<i64>,std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents = contents.trim().to_string();
    
    Ok(contents.split(',').map( |x| {
        match x.parse() {
            Ok(x) => x,
            _ => -1
        }
    }).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() -> io::Result<()> {
        let prog = read("123.txt")?;
        assert_eq!(prog, vec!(1,2,3));

        Ok(())
    }
}
