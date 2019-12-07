use std::fs::File;
use std::io::{self, prelude::*};
use permutohedron::LexicalPermutation;

mod intcoder;
mod multicoder;

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
    let mut phase_settings = vec!(5,6,7,8,9);
    let mut answers = Vec::new();
    let num_computers = phase_settings.len();

    loop {
        // permutohedron has to_vec generate next permutation
        let phase = phase_settings.to_vec();
        
        let mut mcoder = multicoder::MultiCoder::new(&prog, num_computers);

        for p in phase {
            mcoder.manual(p);
        }

        let answer = mcoder.feedback();

        answers.push(answer);

        // ask for next permutation
        if !phase_settings.next_permutation() {
            break;
        }
    }

    answers.sort();

    println!("MAXIMUM THRUST: {:?}", answers[answers.len() - 1]);

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
