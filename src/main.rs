use std::fs::File;
use std::io::{self, Read, prelude::*};
use std::time::{Instant};
use std::time;

mod intcoder;
mod board;


pub fn game(icoder : &mut intcoder::Intcode) -> bool {
    let mut last = -1;
    let mut last_coor = (0, 0);
    let size = 60;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut coor = (size/2,size/2);

    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            row.push(' ');
        }
        map.push(row);
    }

    loop {
        match icoder.run() {
            intcoder::IntResponse::Output(i) => {
                last = i;
                println!("output: {}", i);
                print_map(&mut map, &mut coor, i, &last_coor);
            },
            intcoder::IntResponse::Input => {
                let input: i64 = std::io::stdin()
                    .bytes() 
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as i64).unwrap();

                if input == 10 { continue; }

                let val = input - 48;
                last_coor = (coor.0, coor.1);

                match val {
                    1 => { coor = (coor.0, coor.1 + 1) },
                    2 => { coor = (coor.0, coor.1 - 1) },
                    3 => { coor = (coor.0 - 1, coor.1) },
                    4 => { coor = (coor.0 + 1, coor.1) },
                    _ => continue
                };

                icoder.input(val);

                continue;
            },
            intcoder::IntResponse::Halt => break,
        };
    }

    println!("{}", last);

    true
}

fn print_map(map: &mut Vec<Vec<char>>, coor: &mut (usize, usize), i: i64, last_coor: &(usize, usize)) {
    let size = map.len();

    match i {
        0 => { 
            map[coor.1][coor.0] = '#';
            coor.0 = last_coor.0;
            coor.1 = last_coor.1;
        },
        1 => { map[coor.1][coor.0] = '.' },
        2 => { map[coor.1][coor.0] = 'W' },
        _ => panic!("weird output value! {}", i)
    }

    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if x == coor.0 && y == coor.1 {
                print!("D");
            } else if x == size/2 && y == size/2 {
                print!("S");
            } else {
                print!("{}", c);
            }
        }
        println!("");
    }
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
