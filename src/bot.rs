use crate::{
    check::check,
    pieces::{bishop, knight, rook},
    possible_moves::possible_moves,
    write_all_turns,
};
pub fn gen_move(
    board: &[Vec<char>],
    castle: &Vec<bool>,
    passant: [usize; 3],
    all_turns: &Vec<Vec<char>>,
) -> String
{
    let mut generated_move;
    let best_move = best(board, castle, passant, all_turns);
    generated_move = char::from_u32(best_move[0] as u32 + 97)
        .unwrap()
        .to_string();
    generated_move += &(best_move[1] as i8 - board.len() as i8).abs().to_string();
    generated_move += &char::from_u32(best_move[2] as u32 + 97)
        .unwrap()
        .to_string();
    generated_move += &(best_move[3] as i8 - board.len() as i8).abs().to_string();
    generated_move
}
fn best(
    board: &[Vec<char>],
    castle: &Vec<bool>,
    passant: [usize; 3],
    all_turns: &Vec<Vec<char>>,
) -> Vec<u8>
{
    // https://www.chessprogramming.org/Simplified_Evaluation_Function
    #[rustfmt::skip]
        let table: [[i8; 64]; 6] = [
                                  // pawn
                                  [
                                   0,  0,  0,  0,  0,  0,  0,  0,
                                  50, 50, 50, 50, 50, 50, 50, 50,
                                  10, 10, 20, 30, 30, 20, 10, 10,
                                   5,  5, 10, 25, 25, 10,  5,  5,
                                   0,  0,  0, 20, 20,  0,  0,  0,
                                   5, -5,-10,  0,  0,-10, -5,  5,
                                   5, 10, 10,-20,-20, 10, 10,  5,
                                   0,  0,  0,  0,  0,  0,  0,  0
                                  ],
                                  // rook
                                  [
                                   0,  0,  0,  0,  0,  0,  0,  0,
                                   5, 10, 10, 10, 10, 10, 10,  5,
                                  -5,  0,  0,  0,  0,  0,  0, -5,
                                  -5,  0,  0,  0,  0,  0,  0, -5,
                                  -5,  0,  0,  0,  0,  0,  0, -5,
                                  -5,  0,  0,  0,  0,  0,  0, -5,
                                  -5,  0,  0,  0,  0,  0,  0, -5,
                                   0,  0,  0,  5,  5,  0,  0,  0
                                  ],
                                  // knight
                                  [
                                 -20,-10,-10,-10,-10,-10,-10,-20,
                                 -10,  0,  0,  0,  0,  0,  0,-10,
                                 -10,  0,  5, 10, 10,  5,  0,-10,
                                 -10,  5,  5, 10, 10,  5,  5,-10,
                                 -10,  0, 10, 10, 10, 10,  0,-10,
                                 -10, 10, 10, 10, 10, 10, 10,-10,
                                 -10,  5,  0,  0,  0,  0,  5,-10,
                                 -20,-10,-10,-10,-10,-10,-10,-20
                                  ],
                                  // bishop
                                  [
                                 -29,  4,-82,-37,-25,-42,  7, -8,
                                 -26, 16,-18,-13, 30, 59, 18,-47,
                                 -16, 37, 43, 40, 35, 50, 37, -2,
                                  -4,  5, 19, 50, 37, 37,  7, -2,
                                  -6, 13, 13, 26, 34, 12, 10,  4,
                                   0, 15, 15, 15, 14, 27, 18, 10,
                                   4, 15, 16,  0,  7, 21, 33,  1,
                                 -33, -3,-14,-21,-13,-12,-39,-21
                                  ],
                                  // queen
                                  [
                                 -20,-10,-10, -5, -5,-10,-10,-20,
                                 -10,  0,  0,  0,  0,  0,  0,-10,
                                 -10,  0,  5,  5,  5,  5,  0,-10,
                                  -5,  0,  5,  5,  5,  5,  0, -5,
                                   0,  0,  5,  5,  5,  5,  0, -5,
                                 -10,  5,  5,  5,  5,  5,  0,-10,
                                 -10,  0,  5,  0,  0,  0,  0,-10,
                                 -20,-10,-10, -5, -5,-10,-10,-20
                                  ],
                                  // king
                                  [
                                 -30,-40,-40,-50,-50,-40,-40,-30,
                                 -30,-40,-40,-50,-50,-40,-40,-30,
                                 -30,-40,-40,-50,-50,-40,-40,-30,
                                 -30,-40,-40,-50,-50,-40,-40,-30,
                                 -20,-30,-30,-40,-40,-30,-30,-20,
                                 -10,-20,-20,-20,-20,-20,-20,-10,
                                  20, 20,  0,  0,  0,  0, 20, 20,
                                  20, 30, 10,  0,  0, 10, 30, 20
                                  ]
    ];
    let mut max = [[127i8, -128i8, 0, 0, 0, 0]; 2];
    let n = if all_turns.len() % 2 == 1 { 1 } else { 0 };
    let possible_move = possible_moves(board, Some(castle), Some(passant));
    let start_score = get_score(n, &possible_move);
    let mut pieces_attacked = false;
    let i = if all_turns.len() % 2 == 0 { 1 } else { 0 };
    let mut j = 0;
    while j < 6
    {
        for possible_move in &possible_move[i][j]
        {
            let x = possible_move[0][0] as i8;
            let y = possible_move[0][1] as i8;
            let piece = board[x as usize][y as usize];
            let piece_score = score_of(piece);
            let mut is_attacked = false;
            if (start_score > 20 && j != 0) && attackable(board, x, y)
            {
                is_attacked = true;
                if !pieces_attacked
                {
                    pieces_attacked = true;
                    max[i][0] = 127;
                    max[i][1] = -128;
                }
            }
            for possible_move in possible_move
            {
                let mut board2 = board.to_vec();
                let x2 = possible_move[0] as i8;
                let y2 = possible_move[1] as i8;
                let piece2 = board[x2 as usize][y2 as usize];
                if !((piece2.is_ascii_uppercase() && piece.is_ascii_lowercase())
                    || (piece2.is_ascii_lowercase() && piece.is_ascii_uppercase())
                    || piece2 == ' ')
                {
                    continue;
                }
                board2[x2 as usize][y2 as usize] = piece;
                board2[x as usize][y as usize] = ' ';
                if check(
                    &board2,
                    0,
                    false,
                    if all_turns.len() % 2 == 0 { 'k' } else { 'K' },
                ) != 0
                {
                    continue;
                }
                let score = get_score(n, &possible_moves(&board2, Some(castle), Some(passant)));
                if score < max[i][0] && !(is_attacked ^ pieces_attacked)
                {
                    if attackable(&board2, x2, y2) && piece_score > score_of(piece2)
                    {
                        continue;
                    }
                    max[i][0] = score;
                    max[i][2] = x;
                    max[i][3] = y;
                    max[i][4] = x2;
                    max[i][5] = y2;
                }
                if start_score == max[i][0] && !(is_attacked ^ pieces_attacked)
                {
                    if attackable(&board2, x2, y2) && piece_score > score_of(piece2)
                    {
                        continue;
                    }
                    let num = match board[x as usize][y as usize].to_ascii_lowercase()
                    {
                        'p' => 0,
                        'r' => 1,
                        'n' => 2,
                        'b' => 3,
                        'q' => 4,
                        'k' => 5,
                        _ => continue,
                    };
                    let mut offset = -7;
                    if i == 0
                    {
                        offset += 7;
                    }
                    let t1 = ((y + offset).abs() * 8 + x) as usize;
                    let t2 = ((y2 + offset).abs() * 8 + x2) as usize;
                    let score2 = table[num][t2] - table[num][t1];
                    if score2 > max[i][1]
                    {
                        max[i][1] = score2;
                        max[i][2] = x;
                        max[i][3] = y;
                        max[i][4] = x2;
                        max[i][5] = y2;
                    }
                }
            }
        }
        j += 1;
    }
    if max[i][2] == 0 && max[i][3] == 0 && max[i][4] == 0 && max[i][5] == 0
    {
        println!("Checkmate. {} wins", if i == 1 { "White" } else { "Black" });
        write_all_turns(all_turns, true);
    }
    vec![
        max[i][2] as u8,
        max[i][3] as u8,
        max[i][4] as u8,
        max[i][5] as u8,
    ]
}
fn score_of(piece: char) -> i8
{
    match piece.to_ascii_uppercase()
    {
        'P' => 1,
        'R' => 5,
        'N' => 3,
        'B' => 4,
        'Q' => 9,
        'K' => 10,
        _ => 0,
    }
}
fn get_score(n: usize, possible_move: &[Vec<Vec<Vec<Vec<u8>>>>]) -> i8
{
    (possible_move[n][0].len()
        + possible_move[n][1].len() * 5
        + possible_move[n][2].len() * 3
        + possible_move[n][3].len() * 4
        + possible_move[n][4].len() * 9) as i8
}
fn attackable(board: &[Vec<char>], x: i8, y: i8) -> bool
{
    let moves_from_piece: Vec<Vec<Vec<u8>>> = vec![
        rook::rook(board, x as usize, y as usize),
        bishop::bishop(board, x as usize, y as usize),
        knight::knight(board, x as usize, y as usize),
    ];
    for piece in moves_from_piece
    {
        for i in &piece[1..]
        {
            let x2 = i[0] as i8;
            let y2 = i[1] as i8;
            let piece2 = board[x2 as usize][y2 as usize];
            if !((piece2.is_ascii_uppercase()
                && board[x as usize][y as usize].is_ascii_lowercase())
                || (piece2.is_ascii_lowercase()
                    && board[x as usize][y as usize].is_ascii_uppercase()))
                || piece2 == ' '
            {
                continue;
            }
            match piece2
            {
                'P' =>
                {
                    if y2 == y + 1 && (x2 == x - 1 || x2 == x + 1)
                    {
                        return true;
                    }
                }
                'p' =>
                {
                    if y2 == y - 1 && (x2 == x - 1 || x2 == x + 1)
                    {
                        return true;
                    }
                }
                _ =>
                {}
            }
            match piece2.to_ascii_uppercase()
            {
                'Q' =>
                {
                    if ((x2 - x).abs() == (y2 - y).abs()) || (x2 == x || y2 == y)
                    {
                        return true;
                    }
                }
                'R' =>
                {
                    if x2 == x || y2 == y
                    {
                        return true;
                    }
                }
                'B' =>
                {
                    if (x2 - x).abs() == (y2 - y).abs()
                    {
                        return true;
                    }
                }
                'N' =>
                {
                    if ((x2 - x).abs() == 2 && (y2 - y).abs() == 1)
                        || ((x2 - x).abs() == 1 && (y2 - y).abs() == 2)
                    {
                        return true;
                    }
                }
                'K' =>
                {
                    if (x2 - x).abs() <= 1 && (y2 - y).abs() <= 1
                    {
                        return true;
                    }
                }
                _ =>
                {}
            }
        }
    }
    false
}