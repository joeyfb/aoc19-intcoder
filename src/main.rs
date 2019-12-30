use std::fs::File;
use std::io::{self, Read, prelude::*};
use std::time::{Instant};
use std::time;

mod intcoder;
mod explorer;

fn main() -> io::Result<()> {
    let now = Instant::now();
    let prog = read("program.txt")?; 

    let mut computer = intcoder::Intcode::new(&prog);
    let mut exp = explorer::Explorer::new(&mut computer);
    println!("path from start to oxy: {}", exp.run(false));
    
    exp.print();

    let mut computer = intcoder::Intcode::new(&prog);
    let mut exp = explorer::Explorer::new(&mut computer);
    println!("longest path from oxy: {}", exp.run(true));

    exp.print();

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
