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
        for (i, tile) in self.robot.explore().iter().enumerate() {
            self.map.set(dir::Dir::new((i+1) as i64), tile);
        }

        let mut path = Vec::new();

        match self.decide() {
            Some(dir) => {
                println!("dir: {:?}", dir);
                path.push(dir);
            },
            None => {}
        };

        while let Some(dir) = path.pop() {
            self.robot.go(dir);
            self.map.go(dir);
            for (i, tile) in self.robot.explore().iter().enumerate() {
                self.map.set(dir::Dir::new((i+1) as i64), tile);
            }

            match self.decide() {
                Some(dir) => path.push(dir),
                None => {}
            };
        }

        true
    }

    fn decide(&self) -> Option<dir::Dir> {
        for (i, s) in self.map.explorable().iter().enumerate() {
            if *s {
                return Some(dir::Dir::new( (i+1) as i64));
            }
        }

        None
    }

}
