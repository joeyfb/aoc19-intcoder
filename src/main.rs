use std::fs::File;
use std::io::{self, prelude::*};
use permutohedron::LexicalPermutation;

mod intcoder;

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
    let mut start_vals = vec!(5,6,7,8,9);
    let mut answers = Vec::new();

    loop {
        let vals = start_vals.to_vec();
        let mut computers = Vec::new();

        for _i in 0..5 {
            computers.push(intcoder::Intcode::new(&prog, false));
        }

        for i in 0..5 {
            let inputs = vec!(vals[i]);
            computers[i].run(&inputs);
        }

        let mut answer = 0;
        let mut isgo = 1;
        while isgo > 0 {
            for comp in &mut computers {
                let inputs = vec!(answer);
                isgo = comp.run(&inputs);

                if isgo > 0 {
                    answer = isgo;
                } else {
                    break;
                }
            }
        }

        answers.push(answer);

        println!("{}", answer);

        if !start_vals.next_permutation() {
            break;
        }
    }

    answers.sort();

    println!("{:?}", answers);
    println!("{:?}", answers.len());

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
