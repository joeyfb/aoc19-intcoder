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

    let mut i = 0;
    while i < 100 {
        let mut j = 0;

        while j < 100 {
            let mut icoder = intcoder::Intcode::new(&prog, i, j);
            let answer = icoder.run();

            if answer == 19690720 {
                println!("got: {}", answer);
                println!("nums: {}, {}", i, j);
                println!("answer: {}", (100 * i) + j);
            }

            j += 1;
        }

        i += 1;
    }

    Ok(())
}
