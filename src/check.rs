use crate::pieces::{bishop, knight, rook};
pub fn check(board:&[Vec<char>], turn:usize, checkmate:bool, king:char) -> u8
{
    // if no_check
    //     return 0
    // if white_check
    //     return 1
    // if black_check
    //     return 2
    // if checkmate
    //     return 3
    // if stalemate
    //     return 4
    let mut white_check = false;
    let mut black_check = false;
    'outer: for x in 0..board.len()
    {
        for y in 0..board.len()
        {
            let piece1 = board[x][y];
            if piece1 == king
            {
                // check for check
                let moves_from_king:Vec<Vec<Vec<u8>>> = vec![rook::rook(board, x, y), bishop::bishop(board, x, y), knight::knight(board, x, y)];
                for piece in moves_from_king
                {
                    for i in &piece[1..]
                    {
                        let x2 = i[0] as i8;
                        let y2 = i[1] as i8;
                        let piece2 = board[x2 as usize][y2 as usize];
                        if piece2.is_uppercase() && piece1.is_lowercase()
                        {
                            black_check = true;
                        }
                        else if piece2.is_lowercase() && piece1.is_uppercase()
                        {
                            white_check = true;
                        }
                        else
                        {
                            continue;
                        }
                        if piece2.eq_ignore_ascii_case(&'p')
                        {
                            match piece2
                            {
                                'P' =>
                                {
                                    if y2 == y as i8 + 1 && (x2 == x as i8 - 1 || x2 == x as i8 + 1)
                                    {
                                        break 'outer;
                                    }
                                    else if piece1.is_uppercase()
                                    {
                                        white_check = false;
                                    }
                                    else
                                    {
                                        black_check = false;
                                    }
                                }
                                'p' =>
                                {
                                    if y2 == y as i8 - 1 && (x2 == x as i8 - 1 || x2 == x as i8 + 1)
                                    {
                                        break 'outer;
                                    }
                                    else if piece1.is_uppercase()
                                    {
                                        white_check = false;
                                    }
                                    else
                                    {
                                        black_check = false;
                                    }
                                }
                                _ =>
                                {}
                            }
                        }
                        else
                        {
                            match piece2.to_ascii_lowercase()
                            {
                                'q' =>
                                {
                                    if ((x2 - x as i8).abs() == (y2 - y as i8).abs()) || (x2 == x as i8 || y2 == y as i8)
                                    {
                                        break 'outer;
                                    }
                                    else if piece1.is_uppercase()
                                    {
                                        white_check = false;
                                    }
                                    else
                                    {
                                        black_check = false;
                                    }
                                }
                                'r' =>
                                {
                                    if x2 == x as i8 || y2 == y as i8
                                    {
                                        break 'outer;
                                    }
                                    else if piece1.is_uppercase()
                                    {
                                        white_check = false;
                                    }
                                    else
                                    {
                                        black_check = false;
                                    }
                                }
                                'b' =>
                                {
                                    if (x2 - x as i8).abs() == (y2 - y as i8).abs()
                                    {
                                        break 'outer;
                                    }
                                    else if piece1.is_uppercase()
                                    {
                                        white_check = false;
                                    }
                                    else
                                    {
                                        black_check = false;
                                    }
                                }
                                'n' =>
                                {
                                    if ((x2 - x as i8).abs() == 2 && (y2 - y as i8).abs() == 1) || ((x2 - x as i8).abs() == 1 && (y2 - y as i8).abs() == 2)
                                    {
                                        break 'outer;
                                    }
                                    else if piece1.is_uppercase()
                                    {
                                        white_check = false;
                                    }
                                    else
                                    {
                                        black_check = false;
                                    }
                                }
                                'k' =>
                                {
                                    if (x2 - x as i8).abs() <= 1 && (y2 - y as i8).abs() <= 1
                                    {
                                        break 'outer;
                                    }
                                    else if piece1.is_uppercase()
                                    {
                                        white_check = false;
                                    }
                                    else
                                    {
                                        black_check = false;
                                    }
                                }
                                _ =>
                                {}
                            }
                        }
                    }
                }
            }
        }
    }
    if checkmate
    {
        use crate::possible_moves::possible_moves;
        let mut all_possible_moves:Vec<Vec<u8>> = vec![];
        let moves = possible_moves(board, None, None);
        for mov in &moves
        {
            for mo in mov
            {
                for m in mo
                {
                    all_possible_moves.extend(m[1..].to_vec());
                }
            }
        }
        let color = if turn % 2 == 1 { 0 } else { 1 };
        let mut num_of_checks:Vec<u8> = vec![0, 0];
        for piece in 0..6
        {
            for piece_moves in 0..moves[color][piece].len()
            {
                for i in 1..moves[color][piece][piece_moves].len()
                {
                    let mut copy = board.to_vec();
                    copy[moves[color][piece][piece_moves][i][0] as usize][moves[color][piece][piece_moves][i][1] as usize] =
                        copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize];
                    copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize] = ' ';
                    num_of_checks[0] += 1;
                    if check(&copy, turn, false, king) == (1 + color) as u8
                    {
                        num_of_checks[1] += 1;
                    }
                }
            }
        }
        if num_of_checks[0] == num_of_checks[1]
        {
            return if !white_check && !black_check { 4 } else { 3 };
        }
    }
    if turn % 2 == 1 && white_check
    {
        1
    }
    else if turn % 2 == 0 && black_check
    {
        2
    }
    else
    {
        0
    }
}