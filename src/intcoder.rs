pub struct Intcode {
    prog: Vec<i64>,
    pos: usize,
    halt: bool,
    input: i64,
    result: i64,
}

impl Intcode {

    pub fn new(to_copy: &Vec<i64>) -> Intcode {
        let prog = to_copy.clone();

        Intcode {
            prog: prog,
            pos: 0,
            halt: false,
            input: 0,
            result: 0
        }
    }

    pub fn run(&mut self, input: i64) -> i64 {
        let stop = self.prog.len();
        self.input = input;

        while self.pos < stop {
            if self.halt {
                break;
            }

            let mut code = self.prog[self.pos];
            let mut a : i64 = 0;
            let mut b : i64 = 0;
            let mut c : i64 = 0;
            self.pos += 1;

            if code > 99 {
                let mut rem = code;
                code = code % 100;
                rem = rem / 100;

                c = rem % 10;
                rem = rem / 10;
                b = rem % 10;
                rem = rem / 10;
                a = rem % 10;
            }

            println!("{}: {}", self.pos, code);
            //println!("{:?}", self.prog);

            match code {
                // tertiary
                1|2|7|8 => {
                    let left = self.prog[self.pos];
                    self.pos += 1;
                    let right = self.prog[self.pos];
                    self.pos += 1;
                    let mut store = self.pos as i64;
                    self.pos += 1;
                    let val1 : i64;
                    let val2 : i64;

                    if c != 1 {
                        val1 = self.prog[left as usize];
                    } else {
                        val1 = left;
                    }

                    if b != 1 {
                        val2 = self.prog[right as usize];
                    } else {
                        val2 = right;
                    }

                    if a != 1 {
                        store = self.prog[store as usize];
                    }

                    self.prog[store as usize] = self.tertOp(code, val1, val2);
                },
                // unary
                3 => {
                    let mut first = self.prog[self.pos];
                    self.pos += 1;

                    self.unOp(code, first);
                },
                4 => {
                    let mut first = self.prog[self.pos];
                    self.pos += 1;

                    if c != 1 {
                        first = self.prog[first as usize];
                    }

                    self.unOp(code, first);
                },
                //binary
                5|6 => {
                    let mut left = self.prog[self.pos];
                    self.pos += 1;
                    let mut right = self.prog[self.pos];
                    self.pos += 1;

                    if c != 1 {
                        left = self.prog[left as usize];
                    }

                    if b != 1 {
                        right = self.prog[right as usize];
                    }

                    self.binOp(code, left, right);
                },
                99 => break,
                _   => {
                    println!("error! {}: {}", self.pos, code);
                    break;
                },
            };
        }

        self.result
    }

    fn binOp(&mut self, code: i64, left : i64, right: i64) {
        match code {
            5 => {
                if left != 0 {
                    self.pos = right as usize
                }
            }
            6 => {
                if left == 0 {
                    self.pos = right as usize
                }
            },
            _ => {}
        };
    }

    fn unOp(&mut self, code: i64, first: i64) {
        match code {
            3 => {
                println!("!!! SETTING {} at {} !!!!", self.input, first);
                self.prog[first as usize] = self.input;
            }
            4 => {
                self.result = first;
                println!("> {} from {}", first, self.pos);
            },
            _ => {}
        };
    }


    fn tertOp(&mut self, code: i64, val1 : i64, val2 : i64) -> i64 {
        match code {
            1 => val1 + val2,
            2 => val1 * val2,
            7 => (val1 < val2) as i64,
            8 => (val1 == val2) as i64,
            _ => 00,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_eq() {
        let eq_8 = vec!(3,3,1108,-1,8,3,4,3,99);
        let mut icoder = Intcode::new(&eq_8);
        let answer = icoder.run(8);
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&eq_8);
        let answer = icoder.run(0);
        assert_eq!(answer, 0);
        
        let lt_8 = vec!(3,3,1107,-1,8,3,4,3,99);
        let mut icoder = Intcode::new(&lt_8);
        let answer = icoder.run(9);
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&lt_8);
        let answer = icoder.run(0);
        assert_eq!(answer, 1);
        
        let lt_8 = vec!(3,4,1007,0,-1,4,4,4,99);
        let mut icoder = Intcode::new(&lt_8);
        let answer = icoder.run(9);
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&lt_8);
        let answer = icoder.run(0);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pos_eq() {
        let eq_8 = vec!(3,9,8,9,10,9,4,9,99,-1,8);
        let mut icoder = Intcode::new(&eq_8);
        let answer = icoder.run(8);
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&eq_8);
        let answer = icoder.run(0);
        assert_eq!(answer, 0);
        
        let lt_8 = vec!(3,9,7,9,10,9,4,9,99,-1,8);
        let mut icoder = Intcode::new(&lt_8);
        let answer = icoder.run(9);
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&lt_8);
        let answer = icoder.run(0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_pos_jmp() {
                //     0 1  2 3  4  5   6  7  8   9 10 11  12
        let jmp = vec!(3,11,5,11,12,104,1,99,104,0,99, 0, 8);
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(1);
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(12);
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(0);
        assert_eq!(answer, 1);
        
        let jmp = vec!(3,11,6,11,12,104,1,99,104,0,99, 0, 8);
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(1);
        assert_eq!(answer, 1);
        
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(12);
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(0);
        assert_eq!(answer, 0);
        
        let jmp = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(0);
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(12);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_imm_jmp() {
                //     0 1  2 3  4  5   6  7  8   9 10 11  12
        let jmp = vec!(3,3,1105,0,8,104,1,99,104,0,99);
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(1);
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(12);
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(0);
        assert_eq!(answer, 1);
        
        let jmp = vec!(3,3,1106,11,8,104,1,99,104,0,99);
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(1);
        assert_eq!(answer, 1);
        
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(12);
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(0);
        assert_eq!(answer, 0);
        
        let jmp = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(0);
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let answer = icoder.run(12);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_long() {
        let long = vec!(3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99);

        let mut icoder = Intcode::new(&long);
        let answer = icoder.run(7);
        assert_eq!(answer, 999);

        let mut icoder = Intcode::new(&long);
        let answer = icoder.run(8);
        assert_eq!(answer, 1000);

        let mut icoder = Intcode::new(&long);
        let answer = icoder.run(9);
        assert_eq!(answer, 1001);
    }
}
