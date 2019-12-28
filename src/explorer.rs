use crate::intcoder::{Intcode, IntResponse};

#[derive(Clone)]
enum Tile {
    Space,
    Start,
    Droid,
    Oxy,
    Wall,
    Path,
}

impl Tile {
    pub fn new(val: i64) -> Tile {
        match val {
            -1 => Tile::Start,
            0 => Tile::Wall,
            1 => Tile::Path,
            2 => Tile::Oxy,
            _ => panic!("invalid response! {}", val)
        }
    }

    pub fn char(&self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::Space => ' ',
            Tile::Droid => 'D',
            Tile::Oxy   => 'O',
            Tile::Wall  => '#',
            Tile::Path  => '.',
        }
    }
}

enum Dir {
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

    pub fn rev(&self) -> i64 {
        match self {
            Dir::N => 2,
            Dir::S => 1,
            Dir::W => 4,
            Dir::E => 3,
        }
    }
}

struct Space {
    coor: (usize, usize),
    tile: Tile,
    conns: Option<Box<(Node, Node, Node, Node)>>,
}

// some or none node? to explore or dead
enum Node {
    None,
    Some( Space ),
}

impl Node {
    pub fn copy(&self) -> Node {
        match self {
            Node::None => Node::None,
            Node::Some(space) => Node::Some( Space {
                coor: (space.coor.0, space.coor.1),
                tile: space.tile.clone(),
                conns: None,
            } )
        }
    }
}

pub struct Explorer<'a> {
    icoder: &'a mut Intcode,
    start:  (usize, usize),
    oxy:    Option<Node>,
    map:    Vec<Vec<Tile>>,
    path:   Vec<Space>
}

impl<'a> Explorer<'a> {
    pub fn new(icoder : &mut Intcode) -> Explorer {
        let size = 60;
        let mut Null = Node::None;
        let mut map: Vec<Vec<Tile>> = Vec::new();

        for i in 0..size {
            let mut row = Vec::new();
            for j in 0..size {
                row.push(Tile::Space);
            }
            map.push(row);
        }

        Explorer {
            icoder,
            map,
            start: (size/2, size/2),
            oxy: None,
            path: Vec::new(),
        }
    }

    pub fn print(&self) {
        for line in &self.map {
            for t in line {
                print!("{}", t.char());
            }
            println!("");
        }
    }

    pub fn run(&mut self) -> bool {
        let mut coor = self.start;

        loop {
            let mut space = self.explore(coor.0, coor.1);
            let mut directions = self.explorable(&space);

            // store 1's as potential routes to explor on stack
                // probably just coordinates
                // need to figure out way to navigate back to

            // after fully exploring area choose one of the routes to explore
                // if chose direction, add direction to PATH
                // routes branch from PATH, need to pop from path to reverse back to where paths
                // were that need to explore

            self.path.push(space);
        }

        // after all routes are explored display map
        
        true
    }

    fn explorable(&self, space: &Space) -> (bool, bool, bool, bool) {
        let mut valids = (false, false, false, false);

        match space.conns {
            None => valids,
            Some(nodes) => {
                for i in 0..4 {
                    let node = match i {
                        0 => nodes.0,
                        1 => nodes.1,
                        2 => nodes.2,
                        3 => nodes.3,
                        _ => break
                    };

                    match node {
                        Node::Some(space) => {
                            match space.conns {
                                Some(_i) => continue,
                                None => {},
                            };
                            
                            match space.tile {
                                Tile::Path | Tile::Oxy => {
                                    valids[i] = true;
                                },
                                _ => continue,
                            }
                        },
                        Node::None => continue
                    }

                }
            }
        }
        

        match space.conns {
            Some(nodes) => {
                for i in 0..4 {
                    match nodes[i] {
                        None => {},
                        Some(space) => {
                            match space.tile {
                                Tile::Space => {

                                },
                                _ => {},
                            }
                        }
                    };
                }
            },
            None => {
                // go back
            }
        };
    }

    /*
     * Droid explors current square, returns information on surrounding spaces
     * as Space struct.
     */
    fn explore(&mut self, x: usize, y: usize) -> Space {
        let mut nodes = Vec::new();

        for i in 1..5 {
            let dir = Dir::new(i);
            let coor = dir.go((x,y));
            let tile = self.go(dir.int());

            match &tile {
                Tile::Wall => {},
                Tile::Oxy | Tile::Path  => {
                    self.go(dir.rev());
                },
                _ => panic!("invalid tile response!")
            };

            self.map[coor.1][coor.0] = tile.clone();
            
            nodes.push( Node::Some( Space{ tile, coor, conns: None } ) );
        }

        Space { 
            coor: (x, y),
            tile: self.map[y][x].clone(),
            conns: Some(Box::new((nodes[0].copy(), nodes[1].copy(), nodes[2].copy(), nodes[3].copy())))
        }
    }

    fn go(&mut self, input: i64) -> Tile {
        self.icoder.input(input);

        match self.icoder.run() {
            IntResponse::Output(i) => Tile::new(i),
            IntResponse::Input => panic!("need input!"),
            IntResponse::Halt => panic!("intcoder program halted!"),
        }
    }
}
