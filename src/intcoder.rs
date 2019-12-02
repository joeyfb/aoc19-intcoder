pub struct Intcode {
    prog: Vec<i64>,
    pos: usize
}

impl Intcode {

    pub fn new(to_copy: &Vec<i64>, noun: i64, verb: i64) -> Intcode {
        let mut prog = to_copy.clone();

        prog[1] = noun;
        prog[2] = verb;

        Intcode {
            prog: prog,
            pos: 0
        }
    }


    pub fn run(&mut self) -> i64 {
        let stop = self.prog.len();

        while self.pos < stop {
            let code = self.prog[self.pos];
            self.pos += 1;
            let left = self.prog[self.pos];
            self.pos += 1;
            let right = self.prog[self.pos];
            self.pos += 1;
            let store = self.prog[self.pos];
            self.pos += 1;

            match code {
                1|2 => {
                    let answer = self.command(left, right, code);
                    self.prog[store as usize] = answer;
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

        self.prog[0]
    }


    fn command(&self, left: i64, right: i64, code: i64) -> i64 {
        let val1 = self.prog[left as usize];
        let val2 = self.prog[right as usize];

        match code {
            1 => val1 + val2,
            _ => val1 * val2,
        }
    }
}
