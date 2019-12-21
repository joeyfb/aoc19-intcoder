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
    Scaffold,   // 35 - #
    Space,      // 46 - .
    NewLine,    // 10 - \n 
    Robot(i64),    // 10 - \n 
    Halt
}

fn getTile(response: intcoder::IntResponse) -> Tile {
    let t = match response {
        intcoder::IntResponse::Output(i) => i,
        intcoder::IntResponse::Halt => -2,
        _ => return Tile::Halt,
    };

    match t {
        10 => Tile::NewLine,
        46 => Tile::Space,
        35 => Tile::Scaffold,
        _ => Tile::Robot(t),
    }
}

fn game(icoder : &mut intcoder::Intcode) -> i64 {
    let mut rows = Vec::new();
    let mut row = Vec::new();

    loop {
        let r = icoder.run(0);

        let t = match getTile(r) {
            Tile::Scaffold => "#",
            Tile::Space => ".",
            Tile::Robot(i) => ">",
            Tile::NewLine => "\n",
            Tile::Halt => break
        };

        if t == "\n" {
            rows.push(row);
            row = Vec::new();
        } else {
            row.push(t);
        }
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
    let mut ints = 0;
    let mut hits = 0;
    let mut matches = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        if i > 34 {
            break;
        }
        for (j, t) in r.iter().enumerate() {
            print!("{}", t);

            if i < 1 || j < 1  || i >= (rows.len()-1) || j >= (rows[0].len()-1) {
                continue;
            }

            if *t == "#" && r[j - 1] == "#" && r[j + 1] == "#" && rows[i-1][j] == "#" && rows[i+1][j] == "#" {
                ints += i * j;
                hits += 1;
                matches.push((j, i));
            }
        }
        println!("{}", i);
    }

    println!("hits: {}", hits);
    println!("matches: {:?}", matches);

    ints as i64
}

fn main() -> io::Result<()> {
    let now = Instant::now();
    let prog = read("program.txt")?; 
    let mut computer = intcoder::Intcode::new(&prog);

    let blocks = game(&mut computer);

    println!("score: {}", blocks);

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
