pub fn pawn(board:Vec<Vec<char>>, x:usize, y:usize, passant:Option<[usize; 3]>) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let start:usize;
    let direction:i8;
    if piece.is_uppercase()
    {
        start = board.len() - 2;
        direction = -1;
    }
    else
    {
        start = 1;
        direction = 1;
    }
    let mut possible_moves:Vec<Vec<u8>> = vec![vec![x as u8, y as u8]];
    for x2 in 0..board.len()
    {
        for y2 in 0..board.len()
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            //if it is the first move for the pawn allow double move, and dont allow moving if piece is there
            if y == start && y2 as i8 == y as i8 + (2 * direction) && board[x][(y as i8 + direction) as usize] == ' ' && x2 == x && piece2 == ' '
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
            //if it is not the first move for the pawn only allow single move, and dont allow moving if piece is there
            else if y2 as i8 == y as i8 + direction && x2 == x && piece2 == ' '
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
            //allow diagonal right if there is a piece to capture
            else if x != board.len() - 1 && (y2 as i8 == y as i8 + direction && x2 == x + 1) && piece2 != ' '
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
            //allow diagonal left if there is a piece to capture
            else if x != 0 && (y2 as i8 == y as i8 + direction && x2 == x - 1) && piece2 != ' '
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
            //allow en passant right
            else if let Some(passant) = passant
            {
                if x != board.len() - 1 && (y2 as i8 == y as i8 + direction && x2 == x + 1 && x2 == passant[0] && y == passant[1])
                {
                    possible_moves.push(vec![x2 as u8, y2 as u8]);
                }
                //allow en passant left
                else if x != 0 && (y2 as i8 == y as i8 + direction && x2 == x - 1 && x2 == passant[0] && y == passant[1])
                {
                    possible_moves.push(vec![x2 as u8, y2 as u8]);
                }
            }
        }
    }
    return possible_moves;
}
pub fn promotion(board:&mut Vec<Vec<char>>, x2:usize, y2:usize, piece:char)
{
    if y2 == 0 || y2 == board.len() - 1
    {
        let rook:char;
        let bishop:char;
        let night:char;
        let queen:char;
        let end:usize;
        if piece.is_uppercase()
        {
            queen = 'Q';
            rook = 'R';
            bishop = 'B';
            night = 'N';
            end = 0;
        }
        else
        {
            queen = 'q';
            rook = 'r';
            bishop = 'b';
            night = 'n';
            end = board.len() - 1;
        }
        //allow for promotions, loop is if you type an invalid piece
        loop
        {
            if y2 == end
            {
                println!("Promote to: ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                let input = input.trim();
                if input.eq_ignore_ascii_case("q")
                {
                    board[x2][y2] = queen;
                    break;
                }
                else if input.eq_ignore_ascii_case("r")
                {
                    board[x2][y2] = rook;
                    break;
                }
                else if input.eq_ignore_ascii_case("b")
                {
                    board[x2][y2] = bishop;
                    break;
                }
                else if input.eq_ignore_ascii_case("n")
                {
                    board[x2][y2] = night;
                    break;
                }
                else
                {
                    println!("Invalid piece");
                }
            }
            else
            {
                break;
            }
        }
    }
}