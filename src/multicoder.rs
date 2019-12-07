use crate::intcoder::Intcode;

pub struct MultiCoder {
    computers : Vec<Intcode>,
    active: usize
}

impl MultiCoder {

    pub fn new(prog: &Vec<i64>, size: usize) -> MultiCoder {
        let mut computers = Vec::new();

        for _i in 0..size {
            computers.push(Intcode::new(&prog));
        }

        MultiCoder {
            computers: computers,
            active: 0
        }
    }

    pub fn manual(&mut self, input: i64) -> i64 {
        let answer = self.computers[self.active].run(input);
        self.active = (self.active + 1) % self.computers.len();

        answer
    }

    pub fn feedback(&mut self) -> i64 {
        let mut answer = 0;
        let mut isgo = 1;

        while isgo >= 0 {

            for comp in &mut self.computers {
                isgo = comp.run(answer);

                if isgo > 0 {
                    answer = isgo;
                } else {
                    break;
                }
            }
        }

        answer
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback1() {
        let prog = vec!(3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,
                        4,27,1001,28,-1,28,1005,28,6,99,0,0,5);
        let phase = vec!(9,8,7,6,5);
        let mut mcoder = MultiCoder::new(&prog, phase.len());

        for p in phase {
            mcoder.manual(p);
        }

        let answer = mcoder.feedback();
        assert_eq!(answer, 139629729);
    }

    #[test]
    fn test_feedback2() {
        let prog = vec!(3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10);
        let phase = vec!(9,7,8,5,6);
        let mut mcoder = MultiCoder::new(&prog, phase.len());

        for p in phase {
            mcoder.manual(p);
        }

        let answer = mcoder.feedback();
        assert_eq!(answer, 18216);
    }
}
