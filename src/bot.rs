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
    // white max, black max,white x1,black x1,white y1,black y1,white x2,black x2,white y2,black y2
    //let mut min = vec![40, 40, 0, 0, 0, 0, 0, 0, 0, 0];
    // white min, black min,white x1,black x1,white y1,black y1,white x2,black x2,white y2,black y2
    let mut max = vec![40, 40, 0, 0, 0, 0, 0, 0, 0, 0];
    let possible_move = possible_moves(&board);
    for i in 0..2
    {
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
                    let score = possible_move2[n][0].len() + possible_move2[n][1].len() * 5 + possible_move2[n][2].len() * 3 + possible_move2[n][3].len() * 3 + possible_move2[n][4].len() * 9;
                    if score < max[i]
                    {
                        max[i] = score;
                        max[2 + i] = possible_move[i][j][k][0][0] as usize;
                        max[4 + i] = possible_move[i][j][k][0][1] as usize;
                        max[6 + i] = possible_move[i][j][k][m][0] as usize;
                        max[8 + i] = possible_move[i][j][k][m][1] as usize;
                    }
                }
            }
        }
    }
    vec![max[3] as u8, max[5] as u8, max[7] as u8, max[9] as u8]
}
//moves[0][0] = white pawns
//moves[0][1] = white rooks
//moves[0][2] = white knights
//moves[0][3] = white bishops
//moves[0][4] = white queens
//moves[0][5] = white kings