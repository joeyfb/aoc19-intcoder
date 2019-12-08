pub struct Intcode {
    prog: Vec<i64>,
    pos: usize
}

#[derive(Debug)]
pub enum IntResponse {
    Halt,
    Input,
    Output(i64)
}


impl Intcode {

    pub fn new(to_copy: &Vec<i64>) -> Intcode {
        let prog = to_copy.clone();

        Intcode {
            prog: prog,
            pos: 0
        }
    }
    
    pub fn run(&mut self, input: i64) -> IntResponse {
        let mut input_spent = false;
        let mut halt = false;
        let mut result = IntResponse::Halt;

        while ! halt {
            let instruction = self.fetch(false);
            let (code, first_mode, second_mode, third_mode) = self.decode(instruction);

            //println!("{}: {}", self.pos, code);
            //println!("{:?}", self.prog);

            match code {

                // ARITHMETIC
                1|2|7|8 => {
                    let first = self.fetch(first_mode);
                    let second = self.fetch(second_mode);
                    let mut store = self.pos;
                    self.pos += 1;

                    if third_mode {
                        store = self.prog[store] as usize;
                    }

                    self.prog[store] = self.arithmetic(code, first, second);
                },

                // I/O
                3 => {
                    if input_spent { 
                        self.pos -= 1;
                        halt = true;
                        result = IntResponse::Input;
                    } else {
                        let first = self.fetch(false);
                        self.prog[first as usize] = input;
                        input_spent = true;
                    }
                },
                4 => {
                    halt = true;
                    result = IntResponse::Output(self.fetch(first_mode));
                },

                // JUMP
                5|6 => {
                    let first = self.fetch(first_mode);
                    let second = self.fetch(second_mode);

                    self.pos = self.jmp(code, first, second);
                },

                99 => {
                    halt = true;
                    result = IntResponse::Halt;
                },

                _   => {
                    println!("error! {}: {}", self.pos, code);
                    break;
                },
            };
        }

        result
    }

    fn decode(&self, instruction: i64) ->  (i64, bool, bool, bool) {
        let code = instruction % 100;
        let mut mode = instruction / 100;
        let mut third_mode = true;
        let mut second_mode = true;
        let mut first_mode = true;

        if mode > 0 {
            first_mode = (mode % 10) != 1;
            mode = mode / 10;
            second_mode = (mode % 10) != 1;
            mode = mode / 10;
            third_mode = (mode % 10) != 1;
        }

        (code, first_mode, second_mode, third_mode)
    }

    fn fetch(&mut self, is_pos: bool) -> i64 {
        let arg = self.prog[self.pos];
        self.pos += 1;

        match is_pos {
            true => self.prog[arg as usize],
            false => arg
        }
    }

    fn jmp(&self, code: i64, left : i64, right: i64) -> usize {
        let mut dest = self.pos;
        let jump = match code {
            5 => left != 0,
            6 => left == 0,
            _ => false
        };

        if jump {
            dest = right as usize;
        }

        dest
    }

    fn arithmetic(&self, code: i64, val1 : i64, val2 : i64) -> i64 {
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
        let response = icoder.run(8);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&eq_8);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let lt_8 = vec!(3,3,1107,-1,8,3,4,3,99);
        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.run(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let lt_8 = vec!(3,4,1007,0,-1,4,4,4,99);
        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.run(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pos_eq() {
        let eq_8 = vec!(3,9,8,9,10,9,4,9,99,-1,8);
        let mut icoder = Intcode::new(&eq_8);
        let response = icoder.run(8);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&eq_8);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let lt_8 = vec!(3,9,7,9,10,9,4,9,99,-1,8);
        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.run(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_pos_jmp() {
                //     0 1  2 3  4  5   6  7  8   9 10 11  12
        let jmp = vec!(3,11,5,11,12,104,1,99,104,0,99, 0, 8);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let jmp = vec!(3,11,6,11,12,104,1,99,104,0,99, 0, 8);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let jmp = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_imm_jmp() {
                //     0 1  2 3  4  5   6  7  8   9 10 11  12
        let jmp = vec!(3,3,1105,0,8,104,1,99,104,0,99);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let jmp = vec!(3,3,1106,11,8,104,1,99,104,0,99);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let jmp = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.run(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_long() {
        let long = vec!(3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99);

        let mut icoder = Intcode::new(&long);
        let response = icoder.run(7);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 999);

        let mut icoder = Intcode::new(&long);
        let response = icoder.run(8);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1000);

        let mut icoder = Intcode::new(&long);
        let response = icoder.run(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1001);
    }
}
