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

    pub fn route(&mut self) -> Vec<Coor> {
        let mut stack = vec!(&self.start);
        let mut visited : Vec<Coor> = vec!(self.start.clone());

        loop {
            let curr = match stack.pop() {
                Some(x) => x,
                None => break
            };

            if ! self.graph.contains_key(curr) {
                continue;
            }
            let edges = &self.graph[curr];

            let mut max = 0;
            let mut biggest = curr;
            for edge in edges {
                let dest = &edge.to;
                if ! visited.contains(&dest) && edge.len > max {
                    max = edge.len;
                    biggest = &edge.to;
                }
            }

            if max > 0 {
                stack.push(biggest);
                visited.push(biggest.clone());
                self.visited_push(&mut visited, &curr, biggest, max);
            }
        }

        let mut prev = &visited.remove(0);
        enum Dir {
            U,D,L,R
        };
        let mut facing = Dir::R;
        for next in &visited {
            let is_vertical = prev.x == next.x;
            let is_forward;
            let len;

            if is_vertical {
                is_forward = prev.y < next.y;

                if is_forward {
                    len = next.y - prev.y;
                } else {
                    len = prev.y - next.y;
                }
            } else {
                is_forward = prev.x < next.x;

                if is_forward {
                    len = next.x - prev.x;
                } else {
                    len = prev.x - next.x;
                }
            }

            match facing {
                Dir::U => {
                    if is_forward && is_vertical {
                    } else if is_vertical {
                        print!("R,R,");
                    } else if is_forward {
                        print!("R,");
                    } else {
                        print!("L,");
                    }
                },
                Dir::D => {
                    if is_forward && is_vertical {
                        print!("R,R,");
                    } else if is_vertical {
                    } else if is_forward {
                        print!("L,");
                    } else {
                        print!("R,");
                    }
                },
                Dir::L => {
                    if is_forward && is_vertical {
                        print!("R,");
                    } else if is_vertical {
                        print!("L,");
                    } else if is_forward {
                        print!("R,R,");
                    } else {
                    }
                },
                Dir::R => {
                    if is_forward && is_vertical {
                        print!("L,");
                    } else if is_vertical {
                        print!("R,");
                    } else if is_forward {
                    } else {
                        print!("R,R,");
                    }
                }
            };

            if is_forward && is_vertical {
                facing = Dir::U;
            } else if is_vertical {
                facing = Dir::D;
            } else if is_forward {
                facing = Dir::R;
            } else {
                facing = Dir::L;
            }

            print!("{},", len);
            
            prev = next;
        }

        // reset visited bool
        visited
    }

    fn visited_push(&self, visited: &mut Vec<Coor>, from: &Coor, to: &Coor, len: usize) {
        let is_vertical = from.x == to.x;
        let mut x = from.x;
        let mut y = from.y;

        if x > to.x {
            x = to.x;
        }

        if y > to.y {
            y = to.y;
        }

        for i in 1..len {
            if is_vertical {
                y += 1;
            } else {
                x += 1;
            }

            let maybe = Coor{x, y};

            if self.graph.contains_key(&maybe) {
                println!("added connection! {:?}", maybe);
                visited.push(maybe.clone());
            }
        }
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

                if same || no_line {
                    continue;
                }

                let length = Board::dist(board, p1, p2);

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
        let string : &str = "000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000
000000000000000010000000000000000000000000000
000000000000000010000000000000000000000000000
000000000000000010000000111111111111100000000
000000000000000010000000100000000000100000000
000000000000000010000000100000000000100000000
000000000000000010000000100000000000100000000
000000000000000010111110100000111111111000000
000000000000000010100010100000100000101000000
000000000000001111111111101111111111101000000
000000000000001010100010001000100000001000000
000000000000001011111110001000100000001000000
000000000000001000100000001000100000001000000
000000001111111111101111111111100000001000000
000000001000001000001000001000000000001000000
100000111111111000001000001000000000001000000
100000001000000000001000001000000000001000000
000000001000000000001000001111111000001111111
000000001000000000001000000000001000000000001
000000001111111111111000000000001000000000001
000000000000000000000000000000001000000000001
000000000000000000000000000000001000000000001
000000000000000000000000000000001000000000001
000000000000000000000000000000001000000000001
000000000000000000000000000000001000000000001
000000000000000000000000000000001011111111111
000000000000000000000000000000001010000000000
000000000000000000000000000000001111111000000
000000000000000000000000000000000010001000000
000000000000000000000000000000000010001000000
000000000000000000000000000000000010001000000
000000000000000000000000000000000011111000000";

        //let mut b = Board::new(string, (15, 0), (12, 30));
        let mut b = Board::new(string, (16, 4), (6, 17));

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
