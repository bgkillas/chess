pub fn rook(board:Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![vec![x as u8, y as u8]];
    'outer: for x2 in 0..board.len()
    {
        'inner: for y2 in 0..board.len()
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            //dont allow moving horizontally if piece is in the path
            for i in 1..(x2 as i8 - x as i8).abs()
            {
                if x2 > x
                {
                    if board[x + i as usize][y] != ' '
                    {
                        continue 'outer;
                    }
                }
                else if x2 < x
                {
                    if board[x - i as usize][y] != ' '
                    {
                        continue 'outer;
                    }
                }
            }
            //dont allow moving vertically if piece is in the path
            for i in 1..(y2 as i8 - y as i8).abs()
            {
                if y2 > y
                {
                    if board[x][y + i as usize] != ' '
                    {
                        continue 'inner;
                    }
                }
                else if y2 < y
                {
                    if board[x][y - i as usize] != ' '
                    {
                        continue 'inner;
                    }
                }
            }
            if (x2 == x && y2 != y) || (x2 != x && y2 == y)
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    return possible_moves;
}