use crate::check::check;
use crate::possible_moves::possible_moves;
pub fn gen_move(board:Vec<Vec<char>>) -> String
{
    let mut generated_move;
    let best_move = best(board.clone() /*5, 100, -100*/);
    generated_move = char::from_u32(best_move[0] as u32 + 97).unwrap().to_string();
    generated_move += &(best_move[1] as i8 - board.len() as i8).abs().to_string();
    generated_move += &char::from_u32(best_move[2] as u32 + 97).unwrap().to_string();
    generated_move += &(best_move[3] as i8 - board.len() as i8).abs().to_string();
    generated_move
}
fn best(board:Vec<Vec<char>> /*depth:u8, min:i8, max:i8*/) -> Vec<u8>
{
    const TABLE:[[i8; 64]; 6] = [
                                 //pawn
                                 [
        0, 0, 0, 0, 0, 0, 0, 0, // 1st rank
        5, 10, 10, -20, -20, 10, 10, 5, // 2nd rank
        5, -5, -10, 0, 0, -10, -5, 5, // 3rd rank
        0, 0, 0, 20, 20, 0, 0, 0, // 4th rank
        5, 5, 10, 25, 25, 10, 5, 5, // 5th rank
        10, 10, 20, 30, 30, 20, 10, 10, // 6th rank
        50, 50, 50, 50, 50, 50, 50, 50, // 7th rank
        0, 0, 0, 0, 0, 0, 0, 0, // 8th rank
    ],
                                 //rook
                                 [
        0, 0, 0, 0, 0, 0, 0, 0, // 1st rank
        5, 10, 10, 10, 10, 10, 10, 5, // 2nd rank
        -5, 0, 0, 0, 0, 0, 0, -5, // 3rd rank
        -5, 0, 0, 0, 0, 0, 0, -5, // 4th rank
        -5, 0, 0, 0, 0, 0, 0, -5, // 5th rank
        -5, 0, 0, 0, 0, 0, 0, -5, // 6th rank
        -5, 0, 0, 0, 0, 0, 0, -5, // 7th rank
        0, 0, 0, 5, 5, 0, 0, 0, // 8th rank
    ],
                                 //knight
                                 [
        -50, -40, -30, -30, -30, -30, -40, -50, // 1st rank
        -40, -20, 0, 0, 0, 0, -20, -40, // 2nd rank
        -30, 0, 10, 15, 15, 10, 0, -30, // 3rd rank
        -30, 5, 15, 20, 20, 15, 5, -30, // 4th rank
        -30, 0, 15, 20, 20, 15, 0, -30, // 5th rank
        -30, 5, 10, 15, 15, 10, 5, -30, // 6th rank
        -40, -20, 0, 5, 5, 0, -20, -40, // 7th rank
        -50, -40, -30, -30, -30, -30, -40, -50, // 8th rank
    ],
                                 //bishop
                                 [
        -20, -10, -10, -10, -10, -10, -10, -20, // 1st rank
        -10, 0, 0, 0, 0, 0, 5, -10, // 2nd rank
        -10, 0, 5, 10, 10, 5, 0, -10, // 3rd rank
        -10, 5, 5, 10, 10, 5, 5, -10, // 4th rank
        -10, 0, 10, 10, 10, 10, 0, -10, // 5th rank
        -10, 10, 10, 10, 10, 10, 10, -10, // 6th rank
        -10, 5, 0, 0, 0, 0, 5, -10, // 7th rank
        -20, -10, -10, -10, -10, -10, -10, -20, // 8th rank
    ],
                                 //queen
                                 [
        -20, -10, -10, -5, -5, -10, -10, -20, // 1st rank
        -10, 0, 0, 0, 0, 0, 0, -10, // 2nd rank
        -10, 0, 5, 5, 5, 5, 0, -10, // 3rd rank
        -5, 0, 5, 5, 5, 5, 0, -5, // 4th rank
        0, 0, 5, 5, 5, 5, 0, -5, // 5th rank
        -10, 5, 5, 5, 5, 5, 0, -10, // 6th rank
        -10, 0, 5, 0, 0, 0, 0, -10, // 7th rank
        -20, -10, -10, -5, -5, -10, -10, -20, // 8th rank
    ],
                                 //king
                                 [
        -30, -40, -40, -50, -50, -40, -40, -30, // 1st rank
        -30, -40, -40, -50, -50, -40, -40, -30, // 2nd rank
        -30, -40, -40, -50, -50, -40, -40, -30, // 3rd rank
        -30, -40, -40, -50, -50, -40, -40, -30, // 4th rank
        -20, -30, -30, -40, -40, -30, -30, -20, // 5th rank
        -10, -20, -20, -20, -20, -20, -20, -10, // 6th rank
        20, 20, 0, 0, 0, 0, 20, 20, // 7th rank
        20, 30, 10, 0, 0, 10, 30, 20, // 8th rank
    ],
    ];
    // white max, black max,white x1,black x1,white y1,black y1,white x2,black x2,white y2,black y2
    //let mut min = vec![40, 40, 0, 0, 0, 0, 0, 0, 0, 0];
    // white min, black min,white x1,black x1,white y1,black y1,white x2,black x2,white y2,black y2
    let mut max = vec![[50f64, -100f64, 0f64, 0f64, 0f64, 0f64]; 2];
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
            for m in 1..possible_move[i][j][k].len()
            {
                let mut board2 = board.clone();
                let piece = board2[possible_move[i][j][k][0][0] as usize][possible_move[i][j][k][0][1] as usize];
                board2[possible_move[i][j][k][m][0] as usize][possible_move[i][j][k][m][1] as usize] = piece;
                board2[possible_move[i][j][k][0][0] as usize][possible_move[i][j][k][0][1] as usize] = ' ';
                if check(&board2, 0, false, 'k') != 0
                {
                    continue;
                }
                let possible_move2 = possible_moves(&board2);
                let n = if i == 0 { 1 } else { 0 };
                let score = possible_move2[n][0].len() as f64
                            + possible_move2[n][1].len() as f64 * 5.1
                            + possible_move2[n][2].len() as f64 * 3.2
                            + possible_move2[n][3].len() as f64 * 3.33
                            + possible_move2[n][4].len() as f64 * 8.8;
                let x = possible_move[i][j][k][0][0] as f64;
                let y = possible_move[i][j][k][0][1] as f64;
                let x2 = possible_move[i][j][k][m][0] as f64;
                let y2 = possible_move[i][j][k][m][1] as f64;
                if score < max[i][0]
                {
                    max[i][0] = score;
                    max[i][2] = x;
                    max[i][3] = y;
                    max[i][4] = x2;
                    max[i][5] = y2;
                }
                if start_score == max[0][0] && board[x2 as usize][y2 as usize].is_uppercase() || board[x2 as usize][y2 as usize] == ' '
                {
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
                    let score2 = TABLE[n][(y2 * 8.0 + x2 + 1.0) as usize] as f64 - TABLE[n][(y * 8.0 + x + 1.0) as usize] as f64;
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
//moves[0][0] = white pawns
//moves[0][1] = white rooks
//moves[0][2] = white knights
//moves[0][3] = white bishops
//moves[0][4] = white queens
//moves[0][5] = white kings