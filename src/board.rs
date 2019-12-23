use std::collections::HashMap;

const string : &str = "000000000011111000000000000000000000000000000
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

struct Coor {
    x: usize,
    y: usize,
}

struct Edge {
    to: Coor,
    len: usize,
    visited: bool

}

const start: (usize, usize) = (15, 0);
const end: (usize, usize) = (12, 30);


fn make(board: &str) -> Vec<Vec<bool>> {
    board.lines().map(
        |l| l.chars().map(
            |c| c == '1'
        ).collect()
    ).collect()
}


fn corners(board: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();

    for (y, line) in board.iter().enumerate() {

        for (x, cell) in line.iter().enumerate() {
            if ! cell {
                continue;
            }

            if is_corner(board, x, y) {
                matches.push( (x,y) );
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

fn display(board: &Vec<Vec<bool>>, corners: &Vec<(usize, usize)>) {
    for (y, line) in board.iter().enumerate() {

        for (x, cell) in line.iter().enumerate() {

            if (x,y) == end {
                print!("e");
            } else if (x,y) == start {
                print!("s");
            } else if corners.contains(&(x,y)) {
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

fn adj_list(board: &Vec<Vec<bool>>, corners: &Vec<(usize, usize)>)
    -> HashMap<Coor, Edge> {
        HashMap::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let b = make(string);
        let cs = corners(&b);

        cs.push(start);
        cs.push(end);

        println!("{}", string);
        display(&b, &cs);

        println!("{:?}", cs);

        assert_eq!(false, true);
    }
}
