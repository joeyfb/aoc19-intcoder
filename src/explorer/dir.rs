#[derive(Debug, Copy, Clone)]
pub enum Dir {
    N, S, E, W
}

impl Dir {
    pub fn new(val: i64) -> Dir {
        match val {
            1 => Dir::N,
            2 => Dir::S,
            3 => Dir::W,
            4 => Dir::E,
            _ => panic!("invalid direction!")
        }
    }

    pub fn go(&self, coor: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::N => ( coor.0    , coor.1 + 1),
            Dir::S => ( coor.0    , coor.1 - 1),
            Dir::W => ( coor.0 - 1, coor.1    ),
            Dir::E => ( coor.0 + 1, coor.1    ),
        }
    }

    pub fn int(&self) -> i64 {
        match self {
            Dir::N => 1,
            Dir::S => 2,
            Dir::W => 3,
            Dir::E => 4,
        }
    }

    pub fn rev(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W
        }
    }
}

