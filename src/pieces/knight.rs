pub fn knight(board:Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![vec![x as u8, y as u8]];
    let xmin = if x > 1 { x - 2 } else { 0 };
    let xmax = if x < board.len() - 2 { x + 2 } else { board.len() - 1 };
    let ymin = if y > 1 { y - 2 } else { 0 };
    let ymax = if y < board.len() - 2 { y + 2 } else { board.len() - 1 };
    for x2 in xmin..=xmax
    {
        for y2 in ymin..=ymax
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            //only allow moving in an L shape
            if ((x2 as i8 - x as i8).abs() == 2 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 2)
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    possible_moves
}