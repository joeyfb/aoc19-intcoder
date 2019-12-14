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
    Empty(usize, usize),
    Wall(usize, usize),
    Block(usize, usize),
    Paddle(usize, usize),
    Ball(usize, usize),
    Score(i64),
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
        intcoder::IntResponse::Halt => -2,
        _ => return Tile::Halt,
    };
    let y = match response.1 {
        intcoder::IntResponse::Output(i) => i,
        intcoder::IntResponse::Halt => -2,
        _ => return Tile::Halt,
    };
    let t = match response.2 {
        intcoder::IntResponse::Output(i) => i,
        intcoder::IntResponse::Halt => -2,
        _ => return Tile::Halt,
    };

    if x == -1 && y == 0 {
        return Tile::Score(t);
    }

    if x == -2 || y == -2 || t == -2 {
        return Tile::Halt;
    }

    match t {
        0 => Tile::Empty(x as usize, y as usize),
        1 => Tile::Wall(x as usize, y as usize),
        2 => Tile::Block(x as usize, y as usize),
        3 => Tile::Paddle(x as usize, y as usize),
        4 => Tile::Ball(x as usize, y as usize),
        _ => Tile::Halt,
    }
}

fn direction() -> i64 {
    let reader = io::stdin();

    0
}

fn game(icoder : &mut intcoder::Intcode) -> i64 {
    let mut blocks = 0;
    let height = 24;
    let width = 42;
    let mut board = Vec::new();
    let mut score = 0;
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    for i in 0..height {
        board.push(vec![' '; width]);
    }

    let mut dir = 0;
    let mut counter = 0;
    loop {
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('q') => break,
                termion::event::Key::Char('a') => {
                    dir = -1;
                },
                termion::event::Key::Char('d') => {
                    dir = 1;
                },
                termion::event::Key::Char('s') => {
                    dir = 0;
                },
                _ => {
                    dir = dir;
                }
            }
        }

        let x = icoder.run(dir);
        match x { 
            intcoder::IntResponse::Halt => {
                return -1;
            },
            _ => {}
        }
        let y = icoder.run(dir);
        match y { 
            intcoder::IntResponse::Halt => {
                return -1;
            },
            _ => {}
        }
        let t = icoder.run(dir);
        match t { 
            intcoder::IntResponse::Halt => {
                return -1;
            },
            _ => {}
        }

        match getTile((x, y, t)) {
            Tile::Halt => {
                break;
            },
            Tile::Block(x,y) => {
                board[y][x] = '#';
            },
            Tile::Empty(x,y) => {
                board[y][x] = ' ';
            },
            Tile::Wall(x,y) => {
                board[y][x] = 'W';
            },
            Tile::Paddle(x,y) => {
                board[y][x] = '=';
            },
            Tile::Ball(x,y) => {
                board[y][x] = 'o';
            },
            Tile::Score(s) => {
                score = s;
            },
        }
        counter += 1;

        if counter > (width * height) {
            let mut disp = format!("- {}\n\r", score);
            for y in 0..height {
                for x in 0..width {
                    disp.push_str(&format!("{}", board[y][x]));
                }
                    disp.push_str(&format!("\n\r"));
            }
            stdout.lock().flush().unwrap();
            write!(stdout, "{}{}", clear::All, disp).unwrap();
            thread::sleep(time::Duration::from_millis(1));
        }
    }

    score 
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
