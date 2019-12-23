#[derive(Debug)]
pub enum Tile {
    Scaffold,       // 35 - #
    Space,          // 46 - .
    NewLine,        // 10 - \n 
    Robot(u8),      // 118 | 60 | 61 | 94
    Message(u8),    // Anything else positive
}

impl Tile {
    pub fn new(val: u8) -> Tile {
        match val {
            10 => Tile::NewLine,
            46 => Tile::Space,
            35 => Tile::Scaffold,
            118 | 60 | 61 | 94 => Tile::Robot(val),
            _ => Tile::Message(val),
        }
    }

    pub fn toChar(&self) -> char {
        match self {
            Tile::Scaffold => '#',
            Tile::Space => '.',
            Tile::NewLine => '\n',
            Tile::Robot(i) => *i as char,
            Tile::Message(i) => *i as char,
        }
    }
}
