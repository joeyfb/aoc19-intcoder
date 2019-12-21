pub struct Intcode {
    prog: Vec<i64>,
    ip: usize,
    rel: i64,
    input: Option<i64>
}


#[derive(Debug)]
pub enum IntResponse {
    Halt,
    Input,
    Output(i64)
}


#[derive(Debug)]
pub enum IntMode {
    Pos,
    Imm,
    Rel, 
}


impl Intcode {

    pub fn new(to_copy: &Vec<i64>) -> Intcode {
        let mut prog = to_copy.clone();

        // TODO
        // Shouldn't just niavely add space to end, ideally
        // use checks in self.store/self.fetch to see if we
        // exceed end of array then pad zeroes to fit it
        prog.resize(prog.len() + 3000, 0);

        Intcode {
            prog: prog,
            ip: 0,
            rel: 0,
            input: None
        }
    }

    /*
     * Provide input to program, returns true if input was set
     * and prior input didn't exit
     */
    pub fn input(&mut self, num: i64) -> bool {
        match self.input {
            Some(i) => return false,
            None => {
                self.input = Some(num);
            }
        };

        true
    }

    /*
     * Run program with provdided input if input doesn't already exist.
     * Exists to support old code/tests where run utilized argument.
     */
    pub fn start(&mut self, input: i64) -> IntResponse {
        self.input(input);

        self.run()
    }
   
    /*
     * Run intcode computer until given condition below is hit and returned:
     *
     * IntResponse::Output(i) -> program has stopped to hand off calculation
     * IntResponse::Input     -> program needs input to continue, any provided input used
     * IntResponse::Halt      -> progam has completed
     */
    pub fn run(&mut self) -> IntResponse {
        let mut halt = false;
        let mut result = IntResponse::Halt;

        while ! halt {
            let instruction = self.fetch(IntMode::Imm);
            let (code, first_mode, second_mode, third_mode) = self.decode(instruction);


            match code {

                // ARITHMETIC
                1|2|7|8 => {
                    let first = self.fetch(first_mode);
                    let second = self.fetch(second_mode);
                    let value = self.arithmetic(code, first, second);

                    self.store(third_mode, value);
                },

                // I/O
                3 => {
                    match first_mode {
                        IntMode::Imm => panic!("Immediate mode for input doesn't make sense!"),
                        _ => {}
                    };

                    match self.input {
                        Some(i) => {
                            self.store(first_mode, i);
                            self.input = None;
                        },
                        None => {
                            self.ip -= 1;
                            result = IntResponse::Input;
                            halt = true;
                        }
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

                    self.ip = self.jmp(code, first, second);
                },

                // Relative set
                9 => {
                    self.rel += self.fetch(first_mode);
                },

                99 => {
                    halt = true;
                },

                _ => panic!("bad opcode! {} at {}", code, self.ip),
            };
        }

        result
    }

    /*
     * Decodes and incode instruction, extracting the code and any parameter modes.
     * Default mode is positional mode.
     */
    fn decode(&self, instruction: i64) ->  (i64, IntMode, IntMode, IntMode) {
        let code = instruction % 100;
        let mut mode = instruction / 100;

        let first = mode % 10;
        mode = mode / 10;
        let second = mode % 10;
        mode = mode / 10;
        let third = mode % 10;

        (code, self.mode(first), self.mode(second), self.mode(third))
    }

    /*
     * Store given value at current position in program with given mode. For
     * example, if mode is positional current position in program will be 
     * treated as address to store value in.
     */
    fn store(&mut self, mode: IntMode, val: i64) {
        let mut store = self.ip;
        self.ip += 1;

        store = match mode {
            IntMode::Pos => self.prog[store] as usize,
            IntMode::Imm => store,
            IntMode::Rel => (self.prog[store] + self.rel) as usize
        };

        self.prog[store] = val;
    }

    /*
     * Return mode of given input
     */
    fn mode(&self, bit: i64) -> IntMode {
        match bit {
            0 => IntMode::Pos,
            1 => IntMode::Imm,
            2 => IntMode::Rel,
            _ => panic!("bad bit mode!")
        }
    }

    /*
     * Retrieves next intcode off the program and increments instruction pointer.
     * Modes are:
     *
     * 0 -> positional mode, argument is treated as index into program
     * 1 -> immediate mode, immediately used value at given address
     * 2 -> relative mode, same as poitional mode but increment index by global offset
     */
    fn fetch(&mut self, mode: IntMode) -> i64 {
        let val = self.prog[self.ip];
        self.ip += 1;
        
        match mode {
            IntMode::Pos => self.prog[val as usize],
            IntMode::Imm => val,
            IntMode::Rel => self.prog[(val + self.rel) as usize]
        }
    }

    /*
     * All jump code intructions. Returns new IP location based on
     * whether given test condition passes
     */
    fn jmp(&self, code: i64, left : i64, right: i64) -> usize {
        let mut dest = self.ip;
        let jump = match code {
            5 => left != 0,
            6 => left == 0,
            _ => panic!("bad opcode in jmp!"),
        };

        if jump {
            dest = right as usize;
        }

        dest
    }

    /*
     * Computes given intcode arithmetic and returns result.
     */
    fn arithmetic(&self, code: i64, val1 : i64, val2 : i64) -> i64 {
        match code {
            1 => val1 + val2,
            2 => val1 * val2,
            7 => (val1 < val2) as i64,
            8 => (val1 == val2) as i64,
            _ => panic!("bad opcode in arithmetic!"),
        }
    }


    fn dump(&self) {
        println!("{:?}", self.prog);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_eq() {
        let eq_8 = vec!(3,3,1108,-1,8,3,4,3,99);
        let mut icoder = Intcode::new(&eq_8);
        icoder.input(8);
        let response = icoder.run();
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&eq_8);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let lt_8 = vec!(3,3,1107,-1,8,3,4,3,99);
        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.start(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let lt_8 = vec!(3,4,1007,0,-1,4,4,4,99);
        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.start(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.start(0);
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
        let response = icoder.start(8);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&eq_8);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let lt_8 = vec!(3,9,7,9,10,9,4,9,99,-1,8);
        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.start(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&lt_8);
        let response = icoder.start(0);
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
        let response = icoder.start(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let jmp = vec!(3,11,6,11,12,104,1,99,104,0,99, 0, 8);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let jmp = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(2);
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
        let response = icoder.start(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let jmp = vec!(3,3,1106,11,8,104,1,99,104,0,99);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(1);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(2);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1);

        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let jmp = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 0);
        
        let mut icoder = Intcode::new(&jmp);
        let response = icoder.start(2);
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
        let response = icoder.start(7);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 999);

        let mut icoder = Intcode::new(&long);
        let response = icoder.start(8);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1000);

        let mut icoder = Intcode::new(&long);
        let response = icoder.start(9);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1001);
    }

    #[test]
    fn test_relative() {
        let rel = vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99);
        let mut icoder = Intcode::new(&rel);

        for val in rel {
            let response = icoder.start(0);
            let answer = match response {
                IntResponse::Output(i) => i,
                _ => -1
            };
            assert_eq!(answer, val);
        }
    }

    #[test]
    fn test_bigint() {
        let rel = vec!(1102,34915192,34915192,7,4,7,99,0);
        let mut icoder = Intcode::new(&rel);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1219070632396864);
    }

    #[test]
    fn test_bigint2() {
        let rel = vec!(104,1125899906842624,99);
        let mut icoder = Intcode::new(&rel);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };
        assert_eq!(answer, 1125899906842624);
    }

    #[test]
    fn test_arith_rel() {
        let rel = vec!(109,10,21102,3,3,0,4,10,99);
        let mut icoder = Intcode::new(&rel);
        let response = icoder.start(0);
        let answer = match response {
            IntResponse::Output(i) => i,
            _ => -1
        };

        icoder.dump();

        assert_eq!(answer, 9);
    }

}
