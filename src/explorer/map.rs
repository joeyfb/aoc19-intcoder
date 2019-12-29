use crate::explorer::tile::{Tile};
use crate::explorer::dir::{Dir};


struct Space {
    coor: (usize, usize),
    tile: Option<Tile>,
    visited: bool,
}

impl Space {
    pub fn new(x: usize, y: usize) -> Space {
        Space {
            coor: (x ,y),
            tile: None,
            visited: false
        }
    }

    pub fn char(&self) -> char {
        match &self.tile {
            Some(t) => t.char(),
            None => ' '
        }
    }

    pub fn unseen(&self) -> bool {
        let is_unseen = false;
        let mut not_wall = false;

        if let Some(tile) = &self.tile {
            not_wall = match tile {
                Tile::Wall => false,
                _ => true
            }
        }

        match self.tile {
            None => true,
            _    => ! self.visited && not_wall
        }
    }
}

pub struct Map {
    map:    Vec<Vec<Space>>,
    curr:   (usize, usize),
    start:  (usize, usize),
    oxy:    (usize, usize),
}

impl Map {
    pub fn new(size: usize) -> Map {
        let mut map: Vec<Vec<Space>> = Vec::new();

        for i in 0..size {
            let mut row = Vec::new();
            for j in 0..size {
                row.push(Space::new(j,i));
            }
            map.push(row);
        }

        map[size/2][size/2].visited = true;
        map[size/2][size/2].tile = Some(Tile::Start);

        Map {
            map,
            curr: (size/2, size/2),
            start: (size/2, size/2),
            oxy: (0, 0),
        }
    }

    pub fn go(&mut self, dir: Dir) {
        self.curr = dir.go(self.curr);
        self.map[self.curr.1][self.curr.0].visited = true;
    }

    pub fn set(&mut self, dir: Dir, tile: &Tile) {
        let (x, y) = dir.go(self.curr);

        if self.map[y][x].visited { return };

        if self.map.len() > x && self.map.len() > y {
            self.map[y][x].tile = Some(tile.clone());
        }
    }

    pub fn print(&self) {
        for (y, line) in self.map.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                if (x,y) == self.curr {
                    print!("D");
                } else {
                    print!("{}", t.char());
                }
            }
            println!("");
        }
    }

    pub fn explorable(&self) -> [bool; 4] {
        let mut valids = [false; 4];

        for (x, opt) in self.neighbors().iter().enumerate() {
            if let Some(space) = opt {
                valids[x] = space.unseen();
            }
        }

        valids
    }

    fn neighbors(&self) -> [Option<&Space>; 4] {
        let (x, y) = self.curr;
        let mut neighs = [None, None, None, None];

        if y+1 < self.map.len() {
            neighs[0] = Some(&self.map[y+1][x]);
        }

        if y > 0 {
            neighs[1] = Some(&self.map[y-1][x]);
        }

        if x+1 < self.map.len() {
            neighs[2] = Some(&self.map[y][x-1]);
        }

        if x > 0 {
            neighs[3] = Some(&self.map[y][x+1]);
        }

        neighs
    }
}
