pub struct Intcode {
    prog: Vec<i64>,
    pos: usize
}


impl Intcode {

    pub fn new(to_copy: &Vec<i64>) -> Intcode {
        let prog = to_copy.clone();

        Intcode {
            prog: prog,
            pos: 0
        }
    }
    
    pub fn run(&mut self, input: i64) -> i64 {
        let mut input_spent = false;
        let mut halt = false;
        let mut result = 0;

        while ! halt {
            let code = self.prog[self.pos] % 100;
            let mut mode = self.prog[self.pos] / 100;
            let mut third_imm : bool = true;
            let mut second_imm : bool = true;
            let mut first_imm : bool = true;
            self.pos += 1;

            if mode > 0 {
                first_imm = (mode % 10) != 1;
                mode = mode / 10;
                second_imm = (mode % 10) != 1;
                mode = mode / 10;
                third_imm = (mode % 10) != 1;
            }

            //println!("{}: {}", self.pos, code);
            //println!("{:?}", self.prog);

            match code {
                // ARITHMETIC
                1|2|7|8 => {
                    let first = self.get_arg(first_imm);
                    let second = self.get_arg(second_imm);
                    let mut store = self.pos;
                    self.pos += 1;

                    if third_imm {
                        store = self.prog[store] as usize;
                    }

                    self.prog[store] = self.arithmetic(code, first, second);
                },

                // I/O
                3 => {
                    if input_spent { 
                        self.pos -= 1;
                        halt = true;
                        result = 0;
                    } else {
                        let first = self.get_arg(false);
                        self.prog[first as usize] = input;
                        input_spent = true;
                    }
                },
                4 => {
                    halt = true;
                    result = self.get_arg(first_imm);
                },

                // JUMP
                5|6 => {
                    let first = self.get_arg(first_imm);
                    let second = self.get_arg(second_imm);

                    self.jmp(code, first, second);
                },

                99 => {
                    halt = true;
                    result = -1;
                },
                _   => {
                    println!("error! {}: {}", self.pos, code);
                    break;
                },
            };
        }

        result
    }

    fn get_arg(&mut self, is_pos: bool) -> i64 {
        let mut arg = self.prog[self.pos];
        self.pos += 1;

        if is_pos {
            arg = self.prog[arg as usize];
        }
        
        arg
    }

    fn jmp(&mut self, code: i64, left : i64, right: i64) {
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

    fn arithmetic(&mut self, code: i64, val1 : i64, val2 : i64) -> i64 {
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
