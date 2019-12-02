use std::fs::File;
use std::io::{self, prelude::*};

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

fn command(mut prog : &Vec<i64>, left: i64, right: i64, code: i64) -> i64 {
    let val1 = prog[left as usize];
    let val2 = prog[right as usize];

    match code {
        1 => val1 + val2,
        _ => val1 * val2,
    }
}

fn test(orig: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut prog = orig.clone();
    let stop = prog.len();
    let mut index = 0;
    prog[1] = noun;
    prog[2] = verb;

    while index < stop {
        let code = prog[index];
        index += 1;
        let left = prog[index];
        index += 1;
        let right = prog[index];
        index += 1;
        let store = prog[index];
        index += 1;

        match code {
            1|2 => {
                let answer = command(&prog, left, right, code);
                prog[store as usize] = answer;
            },
            99 => {
                break;
            },
            _  => {
                print!("error!");
                break;
            }
        }
    }

    prog[0]
}

fn main() -> io::Result<()> {
    let mut prog = read("program.txt")?;

    let mut i = 0;
    while i < 100 {
        let mut j = 0;

        while j < 100 {
            let answer = test(&prog, i, j);

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
