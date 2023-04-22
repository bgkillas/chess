use crate::check::check;
use crate::pieces::{bishop, knight, rook};
use crate::possible_moves::possible_moves;
pub fn gen_move(board:Vec<Vec<char>>) -> String
{
    let mut generated_move;
    let best_move = best(board.clone() /* 5, 100, -100 */);
    generated_move = char::from_u32(best_move[0] as u32 + 97).unwrap().to_string();
    generated_move += &(best_move[1] as i8 - board.len() as i8).abs().to_string();
    generated_move += &char::from_u32(best_move[2] as u32 + 97).unwrap().to_string();
    generated_move += &(best_move[3] as i8 - board.len() as i8).abs().to_string();
    generated_move
}
fn best(board:Vec<Vec<char>> /* depth:u8, min:i8, max:i8 */) -> Vec<u8>
{
    // https://www.chessprogramming.org/Simplified_Evaluation_Function
    #[rustfmt::skip]
    const TABLE:[[i16; 64]; 6] = [
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
    let mut max = vec![[500f64, -1000f64, 0f64, 0f64, 0f64, 0f64]; 2];
    let mut is_check = false;
    let possible_move = possible_moves(&board);
    let start_score = possible_move[0][0].len() as f64
                      + possible_move[0][1].len() as f64 * 5.1
                      + possible_move[0][2].len() as f64 * 3.2
                      + possible_move[0][3].len() as f64 * 3.33
                      + possible_move[0][4].len() as f64 * 8.8;
    let i = 1;
    for j in 0..6
    {
        for k in 0..possible_move[i][j].len()
        {
            'outer: for m in 1..possible_move[i][j][k].len()
            {
                let mut board2 = board.clone();
                let piece = board2[possible_move[i][j][k][0][0] as usize][possible_move[i][j][k][0][1] as usize];
                let x = possible_move[i][j][k][0][0] as f64;
                let y = possible_move[i][j][k][0][1] as f64;
                let x2 = possible_move[i][j][k][m][0] as f64;
                let y2 = possible_move[i][j][k][m][1] as f64;
                board2[x2 as usize][y2 as usize] = piece;
                board2[x as usize][y as usize] = ' ';
                if check(&board2, 0, false, 'k') != 0
                {
                    is_check = true;
                    continue;
                }
                let possible_move2 = possible_moves(&board2);
                let n = 0;
                let score = possible_move2[n][0].len() as f64
                            + possible_move2[n][1].len() as f64 * 5.1
                            + possible_move2[n][2].len() as f64 * 3.2
                            + possible_move2[n][3].len() as f64 * 3.33
                            + possible_move2[n][4].len() as f64 * 8.8;
                if score < max[i][0]
                {
                    if !is_check
                    {
                        let piece_score:f64 = match piece
                        {
                            'p' => 1.0,
                            'r' => 5.1,
                            'n' => 3.2,
                            'b' => 3.33,
                            'q' => 8.8,
                            'k' => 10.0,
                            _ => 0.0,
                        };
                        if attackable(&board2, x2, y2)
                        {
                            let piece2_score:f64 = match board[x2 as usize][y2 as usize]
                            {
                                'P' => 1.0,
                                'R' => 5.1,
                                'N' => 3.2,
                                'B' => 3.33,
                                'Q' => 8.8,
                                'K' => 10.0,
                                _ => 0.0,
                            };
                            if piece_score > piece2_score
                            {
                                continue 'outer;
                            }
                        }
                    }
                    max[i][0] = score;
                    max[i][2] = x;
                    max[i][3] = y;
                    max[i][4] = x2;
                    max[i][5] = y2;
                }
                if start_score == max[i][0] && (board[x2 as usize][y2 as usize].is_uppercase() || board[x2 as usize][y2 as usize] == ' ')
                {
                    if !is_check
                    {
                        let piece_score:f64 = match piece
                        {
                            'p' => 1.0,
                            'r' => 5.1,
                            'n' => 3.2,
                            'b' => 3.33,
                            'q' => 8.8,
                            'k' => 10.0,
                            _ => 0.0,
                        };
                        if attackable(&board2, x2, y2)
                        {
                            let piece2_score:f64 = match board[x2 as usize][y2 as usize]
                            {
                                'P' => 1.0,
                                'R' => 5.1,
                                'N' => 3.2,
                                'B' => 3.33,
                                'Q' => 8.8,
                                'K' => 10.0,
                                _ => 0.0,
                            };
                            if piece_score > piece2_score
                            {
                                continue 'outer;
                            }
                        }
                    }
                    let mut n = 0;
                    match board[x as usize][y as usize]
                    {
                        'p' =>
                        {}
                        'r' => n = 1,
                        'n' => n = 2,
                        'b' => n = 3,
                        'q' => n = 4,
                        'k' => n = 5,
                        _ => continue,
                    }
                    let score2 = TABLE[n][((y2 - 7.0).abs() * 8.0 + x2) as usize] as f64 - TABLE[n][((y - 7.0).abs() * 8.0 + x) as usize] as f64;
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
    }
    vec![max[1][2] as u8, max[1][3] as u8, max[1][4] as u8, max[1][5] as u8]
}
// moves[0][0] = white pawns
// moves[0][1] = white rooks
// moves[0][2] = white knights
// moves[0][3] = white bishops
// moves[0][4] = white queens
// moves[0][5] = white kings
fn attackable(board:&[Vec<char>], x:f64, y:f64) -> bool
{
    let moves_from_piece:Vec<Vec<Vec<u8>>> = vec![rook::rook(board, x as usize, y as usize), bishop::bishop(board, x as usize, y as usize), knight::knight(board, x as usize, y as usize)];
    for piece in moves_from_piece
    {
        for i in &piece[1..]
        {
            let x2 = i[0] as i8;
            let y2 = i[1] as i8;
            let piece2 = board[x2 as usize][y2 as usize];
            if piece2.is_lowercase()
            {
                continue;
            }
            match piece2
            {
                'P' =>
                {
                    if y2 == y as i8 + 1 && (x2 == x as i8 - 1 || x2 == x as i8 + 1)
                    {
                        return true;
                    }
                }
                'Q' =>
                {
                    if ((x2 - x as i8).abs() == (y2 - y as i8).abs()) || (x2 == x as i8 || y2 == y as i8)
                    {
                        return true;
                    }
                }
                'R' =>
                {
                    if x2 == x as i8 || y2 == y as i8
                    {
                        return true;
                    }
                }
                'B' =>
                {
                    if (x2 - x as i8).abs() == (y2 - y as i8).abs()
                    {
                        return true;
                    }
                }
                'N' =>
                {
                    if ((x2 - x as i8).abs() == 2 && (y2 - y as i8).abs() == 1) || ((x2 - x as i8).abs() == 1 && (y2 - y as i8).abs() == 2)
                    {
                        return true;
                    }
                }
                'K' =>
                {
                    if (x2 - x as i8).abs() == 1 && (y2 - y as i8).abs() == 1
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