use crate::check::check;
pub fn king(board: &[Vec<char>], x: usize, y: usize, castle: Option<Vec<bool>>) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves: Vec<Vec<u8>> = vec![vec![x as u8, y as u8]];
    let row: usize;
    let first: usize;
    let second: usize;
    let third: usize;
    if piece.is_uppercase()
    {
        row = 7;
        first = 4; // make sure king has not moved
        second = 0; // make sure left rook has not moved
        third = 1; // make sure right rook has not moved
    }
    else
    {
        row = 0;
        first = 5; // make sure king has not moved
        second = 2; // make sure left rook has not moved
        third = 3; // make sure right rook has not moved
    }
    let ymin = if y == 0 { 0 } else { y - 1 };
    let ymax = if y == board.len() - 1
    {
        board.len() - 1
    }
    else
    {
        y + 1
    };
    let xmax = if x == board.len() - 1
    {
        board.len() - 1
    }
    else if x == 4
    {
        x + 2
    }
    else
    {
        x + 1
    };
    let xmin = if x == 0
    {
        0
    }
    else if x == 4
    {
        x - 2
    }
    else
    {
        x - 1
    };
    for x2 in xmin..=xmax
    {
        for y2 in ymin..=ymax
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase()
                || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            // allow castling
            if let Some(ref castle) = castle
            {
                let mut copy = board.to_vec();
                copy[if x2 == 2 { 3 } else { 5 }][y2] = piece;
                if y == row
                    && y2 == y
                    && x == 4
                    && (x2 == 2 || x2 == 6)
                    && castle[first]
                    && castle[second]
                    && castle[third]
                    && piece2 == ' '
                    && (board[5][y] == ' ' || x2 == 2)
                    && ((board[1][y] == ' ' && board[3][y] == ' ') || x2 == 6)
                    && check(&copy, 1, false, board[x][y]) == 0
                {
                    possible_moves.push(vec![x2 as u8, y2 as u8]);
                }
            }
            // allow moving one space in any direction
            if ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 1)
                || ((x2 as i8 - x as i8).abs() == 0 && (y2 as i8 - y as i8).abs() == 1)
                || ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 0)
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    possible_moves
}