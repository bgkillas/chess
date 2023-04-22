pub fn bishop(board:&[Vec<char>], x:usize, y:usize) -> Vec<Vec<u8>>
{
    fn is_path_blocked(board:&[Vec<char>], start:(usize, usize), end:(usize, usize)) -> bool
    {
        let (x1, y1) = start;
        let (x2, y2) = end;
        let delta_x:i8 = if x1 < x2 { 1 } else { -1 };
        let delta_y:i8 = if y1 < y2 { 1 } else { -1 };
        let mut x:i8 = x1 as i8 + delta_x;
        let mut y:i8 = y1 as i8 + delta_y;
        while x != x2 as i8 && y != y2 as i8
        {
            if board[x as usize][y as usize] != ' '
            {
                return true;
            }
            x += delta_x;
            y += delta_y;
        }
        false
    }
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![vec![x as u8, y as u8]];
    for x2 in 0..board.len()
    {
        for y2 in 0..board.len()
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            if (x2 as i8 - x as i8).abs() == (y2 as i8 - y as i8).abs() && !is_path_blocked(board, (x, y), (x2, y2))
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    possible_moves
}