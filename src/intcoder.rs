pub struct Intcode {
    prog: Vec<i64>,
    pos: usize,
    halt: bool,
    result: i64,
    input: Vec<i64>,
    input_index: i64,
}


impl Intcode {

    pub fn new(to_copy: &Vec<i64>) -> Intcode {
        let prog = to_copy.clone();

        Intcode {
            prog: prog,
            pos: 0,
            halt: false,
            input: vec!(),
            result: 0,
            input_index: 0
        }
    }

    pub fn run(&mut self, input: &Vec<i64>) -> i64 {
        self.input = input.to_vec();
        self.input_index = 0;

        loop {
            if self.halt {
                break;
            }

            let mut code = self.prog[self.pos];
            let mut third_imm : bool = true;
            let mut second_imm : bool = true;
            let mut first_imm : bool = true;
            self.pos += 1;

            if code > 99 {
                let mut rem = code;
                code = code % 100;
                rem = rem / 100;

                first_imm = (rem % 10) != 1;
                rem = rem / 10;
                second_imm = (rem % 10) != 1;
                rem = rem / 10;
                third_imm = (rem % 10) != 1;
            }

            //println!("{}: {}", self.pos, code);
            //println!("{:?}", self.prog);

            match code {
                // tertiary
                1|2|7|8 => {
                    let first = self.get_arg(first_imm);
                    let second = self.get_arg(second_imm);
                    let mut store = self.pos;
                    self.pos += 1;

                    if third_imm {
                        store = self.prog[store] as usize;
                    }

                    self.prog[store] = self.tert_op(code, first, second);
                },
                // unary
                3 => {
                    if self.input.len() <= self.input_index as usize { 
                        self.pos -= 1;
                        return -1;
                    };
                    let first = self.get_arg(false);

                    self.un_op(code, first);
                },
                4 => {
                    let first = self.get_arg(first_imm);

                    self.un_op(code, first);

                    return self.result;
                },
                //binary
                5|6 => {
                    let first = self.get_arg(first_imm);
                    let second = self.get_arg(second_imm);

                    self.bin_op(code, first, second);
                },
                99 => break,
                _   => {
                    println!("error! {}: {}", self.pos, code);
                    break;
                },
            };
        }

        -2
    }

    fn get_arg(&mut self, is_pos: bool) -> i64 {
        let mut arg = self.prog[self.pos];
        self.pos += 1;

        if is_pos {
            arg = self.prog[arg as usize];
        }
        
        arg
    }

    fn bin_op(&mut self, code: i64, left : i64, right: i64) {
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

    fn un_op(&mut self, code: i64, first: i64) {
        match code {
            3 => {
                //println!("!!! SETTING {} at {} !!!!", self.input[self.input_index as usize], first);
                self.prog[first as usize] = self.input[self.input_index as usize];
                self.input_index += 1;
            }
            4 => {
                self.result = first;
                //println!("> {} from {}", first, self.pos);
            },
            _ => {}
        };
    }


    fn tert_op(&mut self, code: i64, val1 : i64, val2 : i64) -> i64 {
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
