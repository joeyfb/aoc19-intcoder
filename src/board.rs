use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coor {
    x: usize,
    y: usize,
}

impl Coor {
    pub fn clone(&self) -> Coor {
        Coor { x: self.x, y: self.y }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Edge {
    to: Coor,
    len: usize,
}

pub struct Board {
    board: Vec<Vec<bool>>,
    corners: Vec<Coor>,
    graph: HashMap<Coor, Vec<Edge>>,
    start: Coor,
    end: Coor,
}

impl Board {

    pub fn new(input: &str, start: (usize, usize), end: (usize, usize) )
        -> Board
    {
        let board = input.lines().map(
            |l| l.chars().map(
                |c| c == '1'
                ).collect()
            ).collect();
        let mut corners = Board::corners(&board);

        corners.push( Coor { x: start.0, y: start.1 } );
        corners.push( Coor { x: end.0, y: end.1 } );

        let graph = Board::graph(&board, &corners);

        Board {
            board,
            corners,
            graph,
            start: Coor { x: start.0, y: start.1 },
            end: Coor { x: end.0, y: end.1 },
        }
    }

    pub fn display(&self) {
        for (y, line) in self.board.iter().enumerate() {

            for (x, cell) in line.iter().enumerate() {
                let c = Coor{x,y};

                if c == self.end {
                    print!("e");
                } else if c == self.start {
                    print!("s");
                } else if self.corners.contains(&c) {
                    print!("o");
                } else if *cell {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    pub fn route(&mut self) -> Vec<&Coor> {
        let mut stack = vec!(&self.start);
        let mut visited : Vec<&Coor> = vec!(&self.start);

        loop {
            let curr = match stack.pop() {
                Some(x) => x,
                None => break
            };

            if ! self.graph.contains_key(curr) {
                println!("{:?}", curr);
                continue;
            }
            let edges = &self.graph[curr];

            for edge in edges {
                let dest = &edge.to;
                if ! visited.contains(&dest) {
                    stack.push(dest);
                    visited.push(dest);
                    continue;
                }
            }
        }

        // reset visited bool
        visited
    }

    fn corners(board: &Vec<Vec<bool>>) -> Vec<Coor> {
        let mut matches = Vec::new();

        for (y, line) in board.iter().enumerate() {

            for (x, cell) in line.iter().enumerate() {
                if ! cell {
                    continue;
                }

                if Board::is_corner(board, x, y) {
                    matches.push( Coor{x,y} );
                }

            }
        }

        matches
    }

    /*
     * Expects cell to be path. Returns true if turn could be needed
     * at point.
     *
     *  #    #  #   #   #       
     * ###  ##  ## ##  ##  ##  ##
     *  #                   #  #     etc
     */
    fn is_corner(board: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
        let max_x = board[0].len();
        let max_y = board.len();
        let mut prev_y = false;
        let mut next_y = false;
        let mut prev_x = false;
        let mut next_x = false;

        let mut corner = false;

        if y < (max_y - 1) {
            next_y = board[y+1][x];
        }

        if y != 0 {
            prev_y = board[y-1][x];
        }

        if x < (max_x - 1) {
            next_x = board[y][x+1];
        }

        if x != 0 {
            prev_x = board[y][x-1];
        }

        let vertical = prev_y || next_y;
        let horizontal = prev_x || next_x;

        // if there are both vertical & horizontal paths
        // then a turn could happen!
        if vertical && horizontal {
            corner = true;
        }

        corner
    }

    fn graph(board: &Vec<Vec<bool>>, corners: &Vec<Coor>) -> HashMap<Coor, Vec<Edge>>
    {
        let mut map : HashMap<Coor, Vec<Edge>> = HashMap::new();

        for p1 in corners {
            for p2 in corners {
                let same = p1 == p2;
                let no_line = p1.x != p2.x && p1.y != p2.y;

                if p1 == &(Coor{x:15, y:0}) && p2.y == 0 {
                    println!("-----!------");
                    println!("{:?}", p1);
                    println!("{:?}", p2);
                }

                if same || no_line {
                    continue;
                }

                if p1 == &(Coor{x:15, y:0}) && p2.y == 0 {
                    println!("no_line: {:?}", no_line);
                }
                let length = Board::dist(board, p1, p2);

                if p1 == &(Coor{x:15, y:0}) && p2.y == 0 {
                    println!("len: {:?}", length);
                }

                if length == 0 {
                    continue;
                }

                if map.contains_key(p1) {
                    let list = map.get_mut(p1).unwrap();
                    list.push( Edge{to: p2.clone(), len: length} );
                } else {
                    map.insert(p1.clone(), vec!(Edge{to: p2.clone(), len: length}) );
                }
            }
        }

        map
    }

    fn dist(board: &Vec<Vec<bool>>, from: &Coor, to: &Coor) -> usize
    {
        let x = from.x;
        let y = from.y;
        let end = 0;

        let is_vertical = from.x == to.x;
        let length;
        let grow;

        if is_vertical {
            grow = to.y > y;

            if to.y > y {
                length = to.y - y;
            } else {
                length = y - to.y;
            }
        } else {
            grow = to.x > x;

            if to.x > x {
                length = to.x - x;
            } else {
                length = x - to.x;
            }
        }

        for i in 1..length {
            let mut dx = x;
            let mut dy = y;

            if is_vertical {
                if grow {
                    dy = y + i;
                } else {
                    dy = y - i;
                }
            } else {
                if grow {
                    dx = x + i;
                } else {
                    dx = x - i;
                }
            }
        }

        length
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let string : &str = "000000000011111000000000000000000000000000000
000000000010000000000000000000000000000000000
000000000010000000000000000000000000000000000
000000000010000000000000000000000000000000000
000000000011111110000000000000000000000000000
000000000000000010000000000000000000000000000
000000000000000010000000111111111111100000000
000000000000000010000000100000000000100000000
111111100000000010000000100000000000100000000
100000100000000010000000100000000000100000000
100000100000000010111110100000111111111000000
100000100000000010100010100000100000101000000
100000100000001111111111101111111111101000000
100000100000001010100010001000100000001000000
100000100000001011111110001000100000001000000
100000100000001000100000001000100000001000000
100000101111111111101111111111100000001000000
100000101000001000001000001000000000001000000
100000111111111000001000001000000000001000000
100000001000000000001000001000000000001000000
111111101000000000001000001111111000001111111
000000101000000000001000000000001000000000001
000000101111111111111000000000001000000000001
000000100000000000000000000000001000000000001
000000100000000000000000000000001000000000001
000000100000000000000000000000001000000000001
000000100000000000000000000000001000000000001
000000100000000000000000000000001000000000001
000000100000000000000000000000001011111111111
000000100000000000000000000000001010000000000
000000111111100000000000000000001111111000000
000000000000000000000000000000000010001000000
000000000000000000000000000000000010001000000
000000000000000000000000000000000010001000000
000000000000000000000000000000000011111000000";

        let mut b = Board::new(string, (15, 0), (12, 30));

        b.display();

        println!("{:?}", b.route());

        assert_eq!(false, true);

        //  cs.push(START);
        //  cs.push(END);

        //  println!("{}", string);
        //  display(&b, &cs);

        //  let al = adj_list(&b, &cs);

        //  println!("{:?}", cs);

        //  println!("{:?}", al);

        //  assert_eq!(false, true);
    }
}
