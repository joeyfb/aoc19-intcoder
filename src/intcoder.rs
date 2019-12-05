pub struct Intcode {
    prog: Vec<i64>,
    pos: usize,
    halt: bool
}

impl Intcode {

    pub fn new(to_copy: &Vec<i64>) -> Intcode {
        let mut prog = to_copy.clone();

        Intcode {
            prog: prog,
            pos: 0,
            halt: false,
        }
    }


    pub fn run(&mut self) -> i64 {
        let stop = self.prog.len();

        while self.pos < stop {
            let code = self.prog[self.pos];
            self.pos += 1;

            println!("{}: {}", self.pos, code);

            match code {
                1|2 => self.tertOp(code),
                3|4 => self.unOp(code),
                _  => self.immediate(code),
                99 => break,
            }

            if self.halt {
                break;
            }
        }

        self.prog[0]
    }

    fn immediate(&mut self, code: i64) {
        let mut rem = code;
        let op = rem % 100;
        rem = rem / 100;
        let c = rem % 10;
        rem = rem / 10;
        let b = rem % 10;
        rem = rem / 10;
        let a = rem % 10;

        match op {
            1 => {
                let left = self.prog[self.pos];
                self.pos += 1;
                let right = self.prog[self.pos];
                self.pos += 1;
                let store = self.prog[self.pos];
                self.pos += 1;
                let mut sum : i64 = 0;

                if c != 1 {
                    sum += self.prog[left as usize];
                } else {
                    sum += left as i64;
                }

                if b != 1 {
                    sum += self.prog[right as usize];
                } else {
                    sum += right as i64;
                }

                println!("storing at {}", store);
                self.prog[store as usize] = sum;
            },
            2 => {
                let left = self.prog[self.pos];
                self.pos += 1;
                let right = self.prog[self.pos];
                self.pos += 1;
                let store = self.prog[self.pos];
                self.pos += 1;
                let mut prod : i64 = 1;

                if c != 1 {
                    prod *= self.prog[left as usize];
                } else {
                    prod *= left as i64;
                }

                if b != 1 {
                    prod *= self.prog[right as usize];
                } else {
                    prod *= right as i64;
                }

                self.prog[store as usize] = prod;
            },
            4 => {
                let mut first = self.prog[self.pos];
                self.pos += 1;
                
                if c != 1 {
                    first = self.prog[first as usize];
                }

                println!("> {} from {}", first, self.pos);
            }
            99 => {
                self.halt = true;
            },
            _ => {
                println!("missed something? {}, {}", code, op);
            }
        };
    }

    fn unOp(&mut self, code: i64) {
        let first = self.prog[self.pos];
        self.pos += 1;
        
         match code {
            3 => {
                self.prog[first as usize] = 1;
            }
            4 => {
                println!("> {} from {}", self.prog[first as usize], self.pos);
            },
            _ => {}
        };
    }


    fn tertOp(&mut self, code: i64) {
        let left = self.prog[self.pos];
        self.pos += 1;
        let right = self.prog[self.pos];
        self.pos += 1;
        let store = self.prog[self.pos];
        self.pos += 1;

        let val1 = self.prog[left as usize];
        let val2 = self.prog[right as usize];

        self.prog[store as usize] = match code {
            1 => val1 + val2,
            _ => val1 * val2,
        };
    }
}
