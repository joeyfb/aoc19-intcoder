use std::fs::File;
use std::io::{self, prelude::*};
use std::time::{Instant};
use std::time;

mod intcoder;
mod board;

pub fn game(icoder : &mut intcoder::Intcode) -> bool {
    let comma = ',' as u8;
    let mut isComma = false;

    let mut input = vec!(
        // main
        'A', 'A', 'C', 'B', 'B', 'A', 'B', 'C', 'B', 'C', '\n',

        // A                     <||
        'L', '4', 'L', '4', 'L', '6', 'R', '1', '0', 'L', '6', '\n', 

        // B            ||>
        'R', '8', 'R', '1', '0', 'L', '6', '\n', 

        // C
        'L', '1', '2', 'L', '6', 'R', '1', '0', 'L', '6', '\n',

        // Videofeed
        'N', '\n',
        // Padding
        '1', '\n',
    );
    let mut last = 1;

    loop {
        if input[0] == '\n' {
            isComma = false;
        }

        match icoder.run() {
            intcoder::IntResponse::Output(i) => {
                last = i;
                let c = i as u8;
                print!("{}", c as char);
            },

            intcoder::IntResponse::Input => {
                let i = match isComma {
                    true => comma,
                    false => input.remove(0) as u8
                };

                print!("{}", i as char);
                icoder.input(i as i64);

                if i as char != '\n' {
                    isComma =  ! isComma;
                }

                if (i as char).is_digit(10) && input[0].is_digit(10) {
                    isComma = false;
                }

                continue;
            },
            intcoder::IntResponse::Halt => break,
        };
    }

    println!("{}", last);

    true
}


fn main() -> io::Result<()> {
    let now = Instant::now();
    let prog = read("program.txt")?; 
    let mut computer = intcoder::Intcode::new(&prog);

    game(&mut computer);

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
