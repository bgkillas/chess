use crate::pieces::{bishop, king, knight, pawn, rook};
pub fn possible_moves(board:&[Vec<char>]) -> Vec<Vec<Vec<Vec<Vec<u8>>>>>
{
    let mut moves:Vec<Vec<Vec<Vec<Vec<u8>>>>> = vec![vec![vec![]; 6], vec![vec![]; 6]];
    // moves[0] = white
    // moves[1] = black
    // moves[0][0] = white pawns
    // moves[0][1] = white rooks
    // moves[0][2] = white knights
    // moves[0][3] = white bishops
    // moves[0][4] = white queens
    // moves[0][5] = white kings
    // moves[0][0][0] = white pawn 1
    // moves[0][0][0][0] = white pawn 1 position
    // moves[0][0][0][0][0] = white pawn 1 position x
    // moves[0][0][0][0][1] = white pawn 1 position y
    // moves[0][0][0][1] = white pawn 1 move 1
    // moves[0][0][0][1][0] = white pawn 1 move 1 x
    // moves[0][0][0][1][1] = white pawn 1 move 1 y
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
                match board[x][y].to_ascii_lowercase()
                {
                    'p' => moves[num][0].push(pawn::pawn(board, x, y, None)),
                    'r' => moves[num][1].push(rook::rook(board, x, y)),
                    'n' => moves[num][2].push(knight::knight(board, x, y)),
                    'b' => moves[num][3].push(bishop::bishop(board, x, y)),
                    'q' =>
                    {
                        let mut bishop_moves:Vec<Vec<u8>> = bishop::bishop(board, x, y);
                        let mut rook_moves:Vec<Vec<u8>> = rook::rook(board, x, y);
                        rook_moves.remove(0);
                        bishop_moves.extend(rook_moves);
                        moves[num][4].push(bishop_moves);
                    }
                    'k' => moves[num][5].push(king::king(board, x, y, None)),
                    _ => (),
                }
            }
        }
    }
    moves
}