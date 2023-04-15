pub fn gen_move(board:Vec<Vec<char>>, castle:Vec<bool>, passant:[usize; 3]) -> String
{
    let mut generated_move;
    let possible_moves = crate::possible_moves::possible_moves(board.clone());
    generated_move = (char::from_u32(possible_moves[1][0][0][0][0] as u32 + 97).unwrap() as char).to_string();
    generated_move += &(possible_moves[1][0][0][0][1] as i8 - board.len() as i8).abs().to_string();
    generated_move += &(char::from_u32(possible_moves[1][0][0][1][0] as u32 + 97).unwrap() as char).to_string();
    generated_move += &(possible_moves[1][0][0][1][1] as i8 - board.len() as i8).abs().to_string();
    return generated_move;
}