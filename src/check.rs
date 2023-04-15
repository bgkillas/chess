use crate::pieces::*;
pub fn check(board:Vec<Vec<char>>, turn:usize, checkmate:bool) -> u8
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
    let mut moves:Vec<Vec<Vec<Vec<Vec<u8>>>>> = vec![vec![vec![]; 6], vec![vec![]; 6]];
    //moves[0] = white
    //moves[1] = black
    //moves[0][0] = white pawns
    //moves[0][1] = white rooks
    //moves[0][2] = white knights
    //moves[0][3] = white bishops
    //moves[0][4] = white queens
    //moves[0][5] = white kings
    //moves[0][0][0] = white pawn 1
    //moves[0][0][0][0] = white pawn 1 move 1
    //moves[0][0][0][0][0] = white pawn 1 move 1 x
    //moves[0][0][0][0][1] = white pawn 1 move 1 y
    for x in 0..board.len()
    {
        for y in 0..board.len()
        {
            if board[x][y] != ' '
            {
                let num;
                if board[x][y].is_uppercase()
                {
                    num = 0;
                }
                else
                {
                    num = 1;
                }
                if board[x][y].eq_ignore_ascii_case(&'p')
                {
                    moves[num][0].push(pawn::pawn(board.clone(), x, y, None));
                }
                else if board[x][y].eq_ignore_ascii_case(&'r')
                {
                    moves[num][1].push(rook::rook(board.clone(), x, y));
                }
                else if board[x][y].eq_ignore_ascii_case(&'n')
                {
                    moves[num][2].push(knight::knight(board.clone(), x, y));
                }
                else if board[x][y].eq_ignore_ascii_case(&'b')
                {
                    moves[num][3].push(bishop::bishop(board.clone(), x, y));
                }
                else if board[x][y].eq_ignore_ascii_case(&'q')
                {
                    let mut bishop_moves:Vec<Vec<u8>> = bishop::bishop(board.clone(), x, y);
                    let mut rook_moves:Vec<Vec<u8>> = rook::rook(board.clone(), x, y);
                    rook_moves.remove(0);
                    bishop_moves.extend(rook_moves);
                    moves[num][4].push(bishop_moves);
                }
                else if board[x][y].eq_ignore_ascii_case(&'k')
                {
                    moves[num][5].push(king::king(board.clone(), x, y, None));
                }
            }
        }
    }
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    for i in 0..2
    {
        for j in 0..6
        {
            for k in 0..moves[i][j].len()
            {
                possible_moves.extend(moves[i][j][k][1..].to_vec());
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
                        if value == x as u8
                        {
                            if iter.peek() == Some(&&(y as u8))
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
                                    copy[moves[color][piece][piece_moves][i][0] as usize][moves[color][piece][piece_moves][i][1] as usize] = copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize];
                                    copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize] = ' ';
                                    num_of_checks[0] += 1;
                                    if check(copy, turn, false) == (1 + color) as u8
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
    return 0;
}