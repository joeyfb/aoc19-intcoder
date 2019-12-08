use crate::intcoder::{Intcode, IntResponse};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct MultiCoder {
    senders : Vec<Sender<IntResponse>>,
    receivers : Vec<Receiver<IntResponse>>
}

impl MultiCoder {

    pub fn new(prog: &Vec<i64>, size: usize) -> MultiCoder {
        let mut senders = Vec::new();
        let mut receivers = Vec::new();

        for _i in 0..size {
            let mut computer = Intcode::new(&prog);
            let (from_tx, from_rx) = channel();
            let (to_tx, to_rx) = channel();

            thread::spawn(move || {
                for received in to_rx {
                    match received {
                        IntResponse::Output(i) => {
                            let response = computer.run(i);
                            match &response {
                                IntResponse::Halt => break,
                                _ => {}
                            }
                            from_tx.send(response).unwrap();
                        },
                        _ => {}
                    }
                }
            });

            senders.push(to_tx);
            receivers.push(from_rx);
        }

        MultiCoder {
            senders: senders,
            receivers: receivers
        }
    }

    pub fn send(&mut self, comp: usize, input: i64) -> i64 {
        self.senders[comp].send(IntResponse::Output(input)).unwrap();

        self.output(comp)
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        for input in &self.receivers[from] {
            self.senders[to].send(input);
        }
    }

    pub fn output(&mut self, comp: usize) -> i64 {
        match self.receivers[comp].recv() {
            Ok(i) => match i {
                IntResponse::Output(i) => i,
                _ => -1
            },
            _ => -1
        }
    }

    pub fn feedback(&mut self, start: i64) -> i64 {
        let mut answer = self.send(0, start);
        let mut final_answer = 0;
        let mut index = 1;

        while answer > 0 {
            answer = self.send(index, answer);

            if answer > 0 {
                final_answer = answer;
            }

            index = (index + 1) % self.senders.len();
        }

        final_answer
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal1() {
        let prog = vec!(3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0);
        let phase = vec!(4,3,2,1,0);
        let size = phase.len();
        let mut mcoder = MultiCoder::new(&prog, size);

        for (i, p) in phase.iter().enumerate() {
            mcoder.send(i, *p);
        }

        let answer = mcoder.feedback(0);

        assert_eq!(answer, 43210);
    }

    #[test]
    fn test_normal2() {
        let prog = vec!(3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0);
        let phase = vec!(1,0,4,3,2);
        let size = phase.len();
        let mut mcoder = MultiCoder::new(&prog, size);

        for (i, p) in phase.iter().enumerate() {
            mcoder.send(i, *p);
        }

        let answer = mcoder.feedback(0);

        assert_eq!(answer, 65210);
    }


    #[test]
    fn test_feedback1() {
        let prog = vec!(3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,
                        4,27,1001,28,-1,28,1005,28,6,99,0,0,5);
        let phase = vec!(9,8,7,6,5);
        let mut mcoder = MultiCoder::new(&prog, phase.len());

        for (i, p) in phase.iter().enumerate() {
            mcoder.send(i, *p);
        }

        let answer = mcoder.feedback(0);
        assert_eq!(answer, 139629729);
    }

    #[test]
    fn test_feedback2() {
        let prog = vec!(3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10);
        let phase = vec!(9,7,8,5,6);
        let mut mcoder = MultiCoder::new(&prog, phase.len());

        for (i, p) in phase.iter().enumerate() {
            mcoder.send(i, *p);
        }

        let answer = mcoder.feedback(0);
        assert_eq!(answer, 18216);
    }

}
