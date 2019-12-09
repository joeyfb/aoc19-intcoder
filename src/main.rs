use std::fs::File;
use std::io::{self, prelude::*};
use permutohedron::LexicalPermutation;

mod intcoder;
mod multicoder;

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

fn main() -> io::Result<()> {
    let prog = read("program.txt")?; 

    // part 1
    let mut icoder = intcoder::Intcode::new(&prog);
    let response = icoder.run(1);
    let mut answer = 1;

    answer = match response {
        intcoder::IntResponse::Output(i) => i,
        _ => -1
    };
    println!("{}", answer);

    // part 1
    let mut icoder = intcoder::Intcode::new(&prog);
    let response = icoder.run(2);
    let mut answer = 1;

    answer = match response {
        intcoder::IntResponse::Output(i) => i,
        _ => -1
    };
    println!("{}", answer);

    Ok(())
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
