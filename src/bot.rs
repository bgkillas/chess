use crate::possible_moves::possible_moves;
pub fn gen_move(board:Vec<Vec<char>>) -> String
{
    let mut generated_move;
    let best_move = negamax(board.clone() /*5, 100, -100*/);
    generated_move = char::from_u32(best_move[0] as u32 + 97).unwrap().to_string();
    generated_move += &(best_move[1] as i8 - board.len() as i8).abs().to_string();
    generated_move += &char::from_u32(best_move[2] as u32 + 97).unwrap().to_string();
    generated_move += &(best_move[3] as i8 - board.len() as i8).abs().to_string();
    generated_move
}
fn negamax(board:Vec<Vec<char>> /*depth:u8, min:i8, max:i8*/) -> Vec<u8>
{
    let possible_moves = possible_moves(&board);
    let mut score:Vec<usize> = vec![0, 0];
    for i in 0..2
    {
        score[i] += possible_moves[i][0].len();
        score[i] += possible_moves[i][1].len() * 5;
        score[i] += possible_moves[i][2].len() * 3;
        score[i] += possible_moves[i][3].len() * 3;
        score[i] += possible_moves[i][4].len() * 9;
    }
    let x = possible_moves[1][2][0][0][0];
    let y = possible_moves[1][2][0][0][1];
    let x2 = possible_moves[1][2][0][1][0];
    let y2 = possible_moves[1][2][0][1][1];
    vec![x, y, x2, y2]
}
//moves[0][0] = white pawns
//moves[0][1] = white rooks
//moves[0][2] = white knights
//moves[0][3] = white bishops
//moves[0][4] = white queens
//moves[0][5] = white kings