use crate::intcoder::{Intcode, IntResponse};
use crate::explorer::tile::{Tile};
use crate::explorer::dir::{Dir};

pub struct Robot<'a> {
    icoder: &'a mut Intcode,
}

impl<'a> Robot<'a> {
    pub fn new(icoder : &'a mut Intcode) -> Robot {
        Robot {
            icoder,
        }
    }

    pub fn go(&mut self, dir: Dir) -> Tile {
        self.icoder.input(dir.int());

        match self.icoder.run() {
            IntResponse::Output(i) => Tile::new(i),
            IntResponse::Input => panic!("need input!"),
            IntResponse::Halt => panic!("intcoder program halted!"),
        }
    }

    pub fn explore(&mut self) -> [Tile; 4] {
        let mut tiles = [Tile::Space; 4];

        for i in 1..5 {
            let dir = Dir::new(i);
            let tile = self.go(dir);

            match &tile {
                Tile::Wall => {},
                Tile::Oxy | Tile::Path  => {
                    self.go(dir.rev());
                },
                _ => panic!("invalid tile response!")
            };

            tiles[(i-1) as usize] = tile.clone();
        }

        tiles
    }
}
