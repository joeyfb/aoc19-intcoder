use std::fs::File;
use std::io::{self, prelude::*};
use std::time::{Instant};
use std::collections::HashMap;

mod intcoder;

#[derive(Debug)]
enum Color {
    Not,
    Black,
    White
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

fn robotOut(icoder : &mut intcoder::Intcode, color: &Color) -> (i64, i64) {
    let input = match color {
            Color::White => 1,
            _ => 0,
    };
    let response = icoder.run(input);

    let color = match response {
        intcoder::IntResponse::Output(i) => i,
        intcoder::IntResponse::Input => {
            println!("asking for input!");
            -1
        },
        intcoder::IntResponse::Halt => {
            println!("halting!");
            -1
        }
    };

    if color == -1 {
        return (-1, -1);
    }
    
    let response = icoder.run(input);
    let turn = match response {
        intcoder::IntResponse::Output(i) => i,
        intcoder::IntResponse::Input => {
            println!("asking for input!");
            -1
        },
        intcoder::IntResponse::Halt => {
            println!("halting!");
            -1
        }
    };

    (color, turn)
}

fn main() -> io::Result<()> {
    let now = Instant::now();
    let prog = read("program.txt")?; 

    enum Dir {
        Up, Right, Down, Left
    }

    // part 1
    let mut icoder = intcoder::Intcode::new(&prog);
    let mut map : HashMap<(i64, i64), Color> = HashMap::new();
    let mut coor : (i64, i64) = (0, 0);
    let mut direction = Dir::Up;

    let color = &Color::White;
    let answer = robotOut(&mut icoder, &color); 
    let color = match answer.0 {
        0 => Color::Black,
        1 => Color::White,
        _ => panic!("not a color!")
    };

    map.insert(coor, color);

    match answer.1 {
        0 => {
            direction = match direction {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Up,
            };
        },
        1 => {
            direction = match direction {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        },
        _ => panic!("not a direction!")
    };

    coor = match direction {
        Dir::Up => (coor.0, coor.1 + 1),
        Dir::Left => (coor.0 - 1, coor.1),
        Dir::Down => (coor.0, coor.1 - 1),
        Dir::Right => (coor.0 + 1, coor.1),
    };

    loop {
        let mut color = &Color::Not;
        if map.contains_key(&coor) {
            color = &map[&coor];
        }

        let answer = robotOut(&mut icoder, &color); 

        if answer.0 == -1 {
            break;
        }

        let color = match answer.0 {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("not a color!")
        };

        map.insert(coor, color);

        match answer.1 {
            0 => {
                direction = match direction {
                    Dir::Up => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Down => Dir::Right,
                    Dir::Right => Dir::Up,
                };
            },
            1 => {
                direction = match direction {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                };
            },
            _ => panic!("not a direction!")
        };

        coor = match direction {
            Dir::Up => (coor.0, coor.1 + 1),
            Dir::Left => (coor.0 - 1, coor.1),
            Dir::Down => (coor.0, coor.1 - 1),
            Dir::Right => (coor.0 + 1, coor.1),
        };
    }

    println!("{:?}", map.len());


    let mut image : Vec<Vec<char>> = Vec::new();

    for _i in 0..6 {
        let mut row = Vec::new();

        for _j in 0..50 {
            row.push(' ');
        }

        image.push(row);
    }

    for (key, color) in &map {
        let coor = (key.0, key.1 + 5);

        image[coor.1 as usize][coor.0 as usize] = match color {
            Color::White => 'o',
            _ => ' ',
        };
    }

    image.reverse();
    for line in image {
        for color in line {
            print!("{}", color);
        }
        println!();
    }

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
