use crate::intcoder::{Intcode, IntResponse};

#[derive(Debug)]
enum Tile {
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



pub fn game(icoder : &mut Intcode) -> bool {
    let mut rows = Vec::new();
    let mut row = Vec::new();

    loop {
        let t = match icoder.run(0) {
            IntResponse::Output(i) => Tile::new(i as u8),
            IntResponse::Halt => break,
            IntResponse::Input => break,
        };

        let c = t.toChar();

        if c == '\n' {
            rows.push(row);
            row = Vec::new();
        } else {
            row.push(c);
        }
    }

    // numbering
    for i in 0..45 {
        if i < 10 {
            print!(" ");
        } else {
            print!("{}", i/10);
        }
    }
    println!();
    for i in 0..5 {
        for j in 0..10 {
            print!("{}", j);

            if i == 4 && j == 5 {
                break;
            }
        }
    }
    println!("");

    // graph printing
    for (i, r) in rows.iter().enumerate() {
        for (j, t) in r.iter().enumerate() {
            print!("{}", t);
        }
        println!("{}", i);
    }

    true
}
