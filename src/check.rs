pub fn check(board:&Vec<Vec<char>>, turn:usize, checkmate:bool) -> u8
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
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    let moves = crate::possible_moves::possible_moves(board);
    for mov in &moves
    {
        for mo in mov
        {
            for m in mo
            {
                possible_moves.extend(m[1..].to_vec());
            }
        }
    }
    for x in 0..board.len()
    {
        for y in 0..board.len()
        {
            if board[x][y].eq_ignore_ascii_case(&'k')
            {
                //check for check
                for row in possible_moves.iter()
                {
                    let mut iter = row.iter().peekable();
                    while let Some(&value) = iter.next()
                    {
                        if value == x as u8 && iter.peek() == Some(&&(y as u8))
                        {
                            if board[x][y].is_uppercase()
                            {
                                white_check = true;
                            }
                            else
                            {
                                black_check = true;
                            }
                            break;
                        }
                    }
                }
                if checkmate
                {
                    for color in 0..2
                    {
                        let mut num_of_checks:Vec<u8> = vec![0, 0];
                        for piece in 0..6
                        {
                            for piece_moves in 0..moves[color][piece].len()
                            {
                                for i in 1..moves[color][piece][piece_moves].len()
                                {
                                    let mut copy = board.clone();
                                    copy[moves[color][piece][piece_moves][i][0] as usize][moves[color][piece][piece_moves][i][1] as usize] =
                                        copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize];
                                    copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize] = ' ';
                                    num_of_checks[0] += 1;
                                    if check(&copy, turn, false) == (1 + color) as u8
                                    {
                                        num_of_checks[1] += 1;
                                    }
                                }
                            }
                        }
                        if num_of_checks[0] == num_of_checks[1]
                        {
                            if !white_check && !black_check
                            {
                                return 4;
                            }
                            return 3;
                        }
                    }
                }
            }
        }
    }
    if turn % 2 == 1 && white_check
    {
        return 1;
    }
    else if turn % 2 == 0 && black_check
    {
        return 2;
    }
    0
}