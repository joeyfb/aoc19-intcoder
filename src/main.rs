use std::fs::File;
use std::io::{self, prelude::*};
use std::time::{Instant};
use std::collections::HashMap;

mod intcoder;

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Halt
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

fn getTile(response: (intcoder::IntResponse, intcoder::IntResponse, intcoder::IntResponse)) -> Tile {
    let x = match response.0 {
        intcoder::IntResponse::Output(i) => i,
        _ => return Tile::Halt,
    };
    let y = match response.1 {
        intcoder::IntResponse::Output(i) => i,
        _ => return Tile::Halt,
    };
    let t = match response.2 {
        intcoder::IntResponse::Output(i) => i,
        _ => return Tile::Halt,
    };

    match t {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Paddle,
        4 => Tile::Ball,
        _ => Tile::Halt,
    }
}

fn game(icoder : &mut intcoder::Intcode) -> i64 {
    let mut blocks = 0;

    loop {
        let x = icoder.run(0);
        let y = icoder.run(0);
        let t = icoder.run(0);

        match getTile((x, y, t)) {
            Tile::Halt => {
                break;
            },
            Tile::Block => {
                blocks += 1;
            },
            _ => {
            }
        }
    }

    blocks 
}

fn main() -> io::Result<()> {
    let now = Instant::now();
    let prog = read("program.txt")?; 
    let mut computer = intcoder::Intcode::new(&prog);

    let blocks = game(&mut computer);

    println!("blocks: {}", blocks);

    // TIMING
    let duration = (now.elapsed().subsec_millis() as u128) + 1000*(now.elapsed().as_secs() as u128);

    println!("it took {}ms", duration);

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
