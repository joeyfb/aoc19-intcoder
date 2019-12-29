#[derive(Clone, Copy, Debug)]
pub enum Tile {
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
