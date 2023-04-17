use crate::pieces::*;
pub fn possible_moves(board:&Vec<Vec<char>>) -> Vec<Vec<Vec<Vec<Vec<u8>>>>>
{
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
    //moves[0][0][0][0] = white pawn 1 position
    //moves[0][0][0][0][0] = white pawn 1 position x
    //moves[0][0][0][0][1] = white pawn 1 position y
    //moves[0][0][0][1] = white pawn 1 move 1
    //moves[0][0][0][1][0] = white pawn 1 move 1 x
    //moves[0][0][0][1][1] = white pawn 1 move 1 y
    let mut num;
    for x in 0..board.len()
    {
        for y in 0..board.len()
        {
            if board[x][y] != ' '
            {
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
    moves
}