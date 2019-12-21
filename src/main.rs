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

#[derive(Debug)]
enum Tile {
    Scaffold,       // 35 - #
    Space,          // 46 - .
    NewLine,        // 10 - \n 
    Robot(u8),      // 118 | 60 | 61 | 94
    Message(u8),    // Anything else positive
    Halt,
    Input
}

impl Tile {
    pub fn new(response: intcoder::IntResponse) -> Tile {
        let t = match response {
            intcoder::IntResponse::Output(i) => i,
            intcoder::IntResponse::Halt => -2,
            intcoder::IntResponse::Input => return Tile::Input,
            _ => return Tile::Halt,
        };

        match t {
            10 => Tile::NewLine,
            46 => Tile::Space,
            35 => Tile::Scaffold,
            118 | 60 | 61 | 94 => Tile::Robot(t as u8),
            _ => Tile::Message(t as u8),
        }
    }

    pub fn toChar(&self) -> char {
        'a'
    }
}



fn game(icoder : &mut intcoder::Intcode) -> bool {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut message = Vec::new();

    loop {
        let r = icoder.run(0);

        let t = match Tile::new(r) {
            Tile::Scaffold => '#',
            Tile::Space => '.',
            Tile::Robot(i) => i as char,
            Tile::Input => {
                println!("waiting for input!");
                continue;
            },
            Tile::NewLine => {
                rows.push(row);
                row = Vec::new();
                continue;
            },
            Tile::Message(i) => {
                message.push(i as char);
                continue;
            },

            Tile::Halt => break
        };

        row.push(t);
    }

    // numbering
    for i in 0..45 {
        if i < 10 {
            print!(" ");
        } else {
            print!("{}", i/10);
        }
    }
    println!();
    for i in 0..5 {
        for j in 0..10 {
            print!("{}", j);

            if i == 4 && j == 5 {
                break;
            }
        }
    }
    println!("");

    // graph printing
    for (i, r) in rows.iter().enumerate() {
        for (j, t) in r.iter().enumerate() {
            print!("{}", t);
        }
        println!("{}", i);
    }

    // message printing
    for c in message {
        print!("{}", c);
    }
    println!("");

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
