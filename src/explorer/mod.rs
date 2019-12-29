use crate::intcoder::{Intcode};
mod tile;
mod dir;
mod map;
mod robot;

pub struct Explorer<'a> {
    robot:  robot::Robot<'a>,
    map:    map::Map,
}

impl<'a> Explorer<'a> {
    pub fn new(icoder : &mut Intcode) -> Explorer {
        Explorer {
            robot: robot::Robot::new(icoder),
            map: map::Map::new(60),
        }
    }
    
    pub fn print(&self) {
        self.map.print();
    }

    pub fn run(&mut self) -> bool {
        let mut path = Vec::new();

        loop {
            for (i, tile) in self.robot.explore().iter().enumerate() {
                self.map.set(dir::Dir::new((i+1) as i64), tile);
            }

            match self.decide() {
                Some(g) => {
                    let dir = g.0;
                    self.robot.go(dir);
                    self.map.go(dir);
                    path.push(g);
                },
                None => {
                    if let Some(g) = path.pop() {
                        let dir = g.0;

                        self.robot.go(dir.rev());
                        self.map.go(dir.rev());
                    } else {
                        break;
                    }
                }
            };
        }

        true
    }

    fn decide(&self) -> Option<(dir::Dir, usize)> {
        let mut possible = 0;
        let mut dir = None;

        for (i, s) in self.map.explorable().iter().enumerate() {
            if *s {
                dir = Some(dir::Dir::new( (i+1) as i64));
                possible += 1;
            }
        }

        match dir {
            Some(d) => Some( (d, possible) ),
            None => None
        }
    }

}
