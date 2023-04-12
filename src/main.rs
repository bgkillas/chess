fn main()
{
    // TODO: checkmate
    // TODO: stalemate
    // TODO: implement networking
    // TODO: implement stock fish
    let mut flip = false;
    let mut numbers = false;
    let mut keep_flip = false;
    let mut file = String::new();
    for i in 0..std::env::args().len()
    {
        if std::env::args().nth(i).unwrap() == "--help"
        {
            println!("Usage: chess [OPTION]...");
            println!("to move a piece type the coordinates of the piece you want to move and the coordinates of where you want to move it");
            println!("for example: e2e4 or 5254");
            println!("--flip will flip the board each move");
            println!("--keep_flip will have black on the bottom and white on the top");
            println!("--numbers will show 1 2 3 4 5 6 7 8 on the bottom instead of a b c d e f g h");
            println!("--file CSV will load a board from a csv file");
            std::process::exit(0);
        }
        else if std::env::args().nth(i).unwrap() == "--flip"
        {
            flip = true;
        }
        else if std::env::args().nth(i).unwrap() == "--keep_flip"
        {
            keep_flip = true;
        }
        else if std::env::args().nth(i).unwrap() == "--numbers"
        {
            numbers = true;
        }
        else if std::env::args().nth(i).unwrap() == "--file"
        {
            file = std::env::args().nth(i + 1).unwrap();
        }
    }
    let mut board:Vec<Vec<char>>;
    if file != "" && std::path::Path::new(&file).exists()
    {
        let csv = std::fs::File::open(file).unwrap();
        let reader = std::io::BufReader::new(csv);
        use std::io::BufRead;
        board = reader.lines().map(|l| l.unwrap().split(',').map(|c| c.chars().nth(0).unwrap()).collect()).collect();
    }
    else
    {
        board = vec![vec!['r', 'p', ' ', ' ', ' ', ' ', 'P', 'R'],
                     vec!['n', 'p', ' ', ' ', ' ', ' ', 'P', 'N'],
                     vec!['b', 'p', ' ', ' ', ' ', ' ', 'P', 'B'],
                     vec!['q', 'p', ' ', ' ', ' ', ' ', 'P', 'Q'],
                     vec!['k', 'p', ' ', ' ', ' ', ' ', 'P', 'K'],
                     vec!['b', 'p', ' ', ' ', ' ', ' ', 'P', 'B'],
                     vec!['n', 'p', ' ', ' ', ' ', ' ', 'P', 'N'],
                     vec!['r', 'p', ' ', ' ', ' ', ' ', 'P', 'R']];
    }
    //ensure the board is a square
    if board[0].len() != board.len()
    {
        println!("Board must be a square");
        std::process::exit(1);
    }
    //turn tracker
    let mut all_turns:Vec<Vec<char>> = vec![vec![]];
    let mut turns:Vec<Vec<char>> = vec![vec!['0'; 4]; board.len()];
    let mut turn = 1;
    print_board(board.clone(), turns.clone(), flip, numbers, keep_flip, turn);
    //castling stuff
    let mut black_castle = true;
    let mut white_castle = true;
    let mut white_left_castle = true;
    let mut white_right_castle = true;
    let mut black_left_castle = true;
    let mut black_right_castle = true;
    let mut copy = board.clone();
    //en passant stuff
    let mut passant = [0; 3];
    'outer: loop
    {
        //dont allow en passant on a piece after a turn
        if turn != passant[2] + 1
        {
            passant[0] = 0;
            passant[1] = 0;
            passant[2] = 0;
        }
        println!();
        //check for draw
        'inner: for row in board.iter()
        {
            for &c in row.iter()
            {
                if !(c == ' ' || c.eq_ignore_ascii_case(&'k'))
                {
                    break 'inner;
                }
            }
            println!("Draw");
            break 'outer;
        }
        //check for checkmate
        if check(board.clone()) == 3
        {
            println!("Stalemate");
            std::process::exit(0);
        }
        else if check(board.clone()) == 2
        {
            if turn % 2 == 0
            {
                println!("White wins");
            }
            else
            {
                println!("Black wins");
            }
            std::process::exit(0);
        }
        else if check(board.clone()) == 1
        {
            if turn % 2 == 0
            {
                println!("Black is in check");
            }
            else
            {
                println!("White is in check");
            }
            copy = board.clone();
        }
        if turn % 2 == 0
        {
            println!("Black's turn");
        }
        else
        {
            println!("White's turn");
        }
        println!("Enter a move: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        //turn input from a2a4 to [1,2,1,4]
        let moves:Vec<u8> = input.chars()
                                 .flat_map(|c| {
                                     match c
                                     {
                                         'a' => Some(1),
                                         'b' => Some(2),
                                         'c' => Some(3),
                                         'd' => Some(4),
                                         'e' => Some(5),
                                         'f' => Some(6),
                                         'g' => Some(7),
                                         'h' => Some(8),
                                         'i' => Some(9),
                                         'j' => Some(10),
                                         'k' => Some(11),
                                         'l' => Some(12),
                                         'm' => Some(13),
                                         'n' => Some(14),
                                         'o' => Some(15),
                                         'p' => Some(16),
                                         'q' => Some(17),
                                         'r' => Some(18),
                                         's' => Some(19),
                                         't' => Some(20),
                                         'u' => Some(21),
                                         'v' => Some(22),
                                         'w' => Some(23),
                                         'x' => Some(24),
                                         'y' => Some(25),
                                         'z' => Some(26),
                                         'E' => Some(27),
                                         'X' => Some(28),
                                         'I' => Some(29),
                                         'T' => Some(30),
                                         _ => c.to_digit(10).map(|d| d as u8),
                                     }
                                 })
                                 .collect();
        if moves[0] == 27 && moves[1] == 28 && moves[2] == 29 && moves[3] == 30
        {
            for i in 0..all_turns.len()
            {
                for j in 0..all_turns[i].len()
                {
                    print!("{}", all_turns[i][j]);
                }
            }
            std::process::exit(0);
        }
        //ensure the input is in range
        if moves.len() != 4 || moves[0] < 1 || moves[0] > (board.len() + 2) as u8 || moves[1] < 1 || moves[1] > (board.len() + 2) as u8 || moves[2] < 1 || moves[2] > (board.len() + 2) as u8 || moves[3] < 1 || moves[3] > (board.len() + 2) as u8
        {
            println!("Invalid move");
            continue;
        }
        let x = moves[0] as usize - 1;
        let y = (moves[1] as i8 - board.len() as i8).abs() as usize;
        let x2 = moves[2] as usize - 1;
        let y2 = (moves[3] as i8 - board.len() as i8).abs() as usize;
        let piece = board[x][y];
        let piece2 = board[x2][y2];
        //dont move if the piece is the same color as the piece you are moving to
        if piece.is_uppercase() && piece2.is_uppercase() || piece.is_lowercase() && piece2.is_lowercase()
        {
            println!("Invalid move");
            continue;
        }
        //allow only white piece to move if its white's turn
        if turn % 2 == 0 && piece.is_uppercase()
        {
            println!("Invalid move");
            continue;
        }
        //allow only black piece to move if its black's turn
        else if turn % 2 == 1 && piece.is_lowercase()
        {
            println!("Invalid move");
            continue;
        }
        //pawn movement
        if piece == 'P' || piece == 'p'
        {
            //if white
            if piece.is_uppercase()
            {
                //if it is the first move for the pawn allow double move, and dont allow moving if piece is there
                if y == board.len() - 2 && y2 == y - 2 && x2 == x && piece2 == ' '
                {
                    passant = [x2, y2, turn];
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //if it is not the first move for the pawn only allow single move, and dont allow moving if piece is there
                else if y2 == y - 1 && x2 == x && piece2 == ' '
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //allow diagonal right if there is a piece to capture
                else if x != board.len() - 1 && (y2 == y - 1 && x2 == x + 1 && piece2.is_lowercase())
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //allow diagonal left if there is a piece to capture
                else if x != 0 && (y2 == y - 1 && x2 == x - 1 && piece2.is_lowercase())
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //allow en passant right
                else if x != board.len() - 1 && (y2 == y - 1 && x2 == x + 1 && x2 == passant[0] && y == passant[1])
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                    board[x2][y] = ' ';
                }
                //allow en passant left
                else if x != 0 && (y2 == y - 1 && x2 == x - 1 && x2 == passant[0] && y == passant[1])
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                    board[x2][y] = ' ';
                }
                else
                {
                    println!("Invalid move");
                    continue;
                }
                //allow for promotions, loop is if you type an invalid piece
                loop
                {
                    if y2 == 0
                    {
                        println!("Promote to: ");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");
                        let input = input.trim();
                        if input == "Q"
                        {
                            board[x2][y2] = 'Q';
                            break;
                        }
                        else if input == "R"
                        {
                            board[x2][y2] = 'R';
                            break;
                        }
                        else if input == "B"
                        {
                            board[x2][y2] = 'B';
                            break;
                        }
                        else if input == "H"
                        {
                            board[x2][y2] = 'N';
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
            //if black
            else if piece.is_lowercase()
            {
                //if it is the first move for the pawn allow double move, and dont allow moving if piece is there
                if y == 1 && y2 == y + 2 && x2 == x && piece2 == ' '
                {
                    passant = [x2, y2, turn];
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //if it is not the first move for the pawn only allow single move, and dont allow moving if piece is there
                else if y2 == y + 1 && x2 == x && piece2 == ' '
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //allow diagonal right if there is a piece to capture
                else if x != board.len() - 1 && (y2 == y + 1 && x2 == x + 1 && piece2.is_uppercase())
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //allow diagonal left if there is a piece to capture
                else if x != 0 && (y2 == y + 1 && x2 == x - 1 && piece2.is_uppercase())
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                //allow en passant right
                else if x != board.len() - 1 && (y2 == y + 1 && x2 == x + 1 && x2 == passant[0] && y == passant[1])
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                    board[x2][y] = ' ';
                }
                //allow en passant left
                else if x != 0 && (y2 == y + 1 && x2 == x - 1 && x2 == passant[0] && y == passant[1])
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                    board[x2][y] = ' ';
                }
                else
                {
                    println!("Invalid move");
                    continue;
                }
                //allow for promotions, loop is if you type an invalid piece
                loop
                {
                    if y2 == 7
                    {
                        println!("Promote to: ");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");
                        let input = input.trim();
                        if input == "q"
                        {
                            board[x2][y2] = 'q';
                            break;
                        }
                        else if input == "r"
                        {
                            board[x2][y2] = 'r';
                            break;
                        }
                        else if input == "b"
                        {
                            board[x2][y2] = 'b';
                            break;
                        }
                        else if input == "h"
                        {
                            board[x2][y2] = 'n';
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
        //if rook
        else if piece == 'R' || piece == 'r'
        {
            //dont allow moving horizontally if piece is in the path
            for i in 1..(x2 as i8 - x as i8).abs()
            {
                if x2 > x
                {
                    if board[x + i as usize][y] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
                else if x2 < x
                {
                    if board[x - i as usize][y] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
            }
            //dont allow moving vertically if piece is in the path
            for i in 1..(y2 as i8 - y as i8).abs()
            {
                if y2 > y
                {
                    if board[x][y + i as usize] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
                else if y2 < y
                {
                    if board[x][y - i as usize] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
            }
            //allow moving vertically
            if x2 == x && y2 != y
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            //allow moving horizontally
            else if x2 != x && y2 == y
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else
            {
                println!("Invalid move");
                continue;
            }
            if x == 0 && piece == 'R'
            {
                white_left_castle = false;
            }
            else if x == 7 && piece == 'R'
            {
                white_right_castle = false;
            }
            else if x == 0 && piece == 'r'
            {
                black_left_castle = false;
            }
            else if x == 7 && piece == 'r'
            {
                black_right_castle = false;
            }
        }
        //if bishop
        else if piece == 'B' || piece == 'b'
        {
            //dont allow moving if piece is in the path
            for i in 1..(x2 as i8 - x as i8).abs()
            {
                if x2 > x && y2 > y
                {
                    if board[x + i as usize][y + i as usize] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
                else if x2 < x && y2 < y
                {
                    if board[x - i as usize][y - i as usize] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
                else if x2 > x && y2 < y
                {
                    if board[x + i as usize][y - i as usize] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
                else if x2 < x && y2 > y
                {
                    if board[x - i as usize][y + i as usize] != ' '
                    {
                        println!("Invalid move");
                        continue 'outer;
                    }
                }
            }
            //only allow moving diagonally
            if (x2 as i8 - x as i8).abs() == (y2 as i8 - y as i8).abs()
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else
            {
                println!("Invalid move");
                continue;
            }
        }
        //if knight
        else if piece == 'N' || piece == 'n'
        {
            //only allow moving in an L shape
            if (x2 as i8 - x as i8).abs() == 2 && (y2 as i8 - y as i8).abs() == 1
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else if (x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 2
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else
            {
                println!("Invalid move");
                continue;
            }
        }
        //if queen
        else if piece == 'Q' || piece == 'q'
        {
            //if moving horizontally
            if x2 == x && y2 != y
            {
                //dont allow moving horizontally if piece is in the path
                for i in 1..(y2 as i8 - y as i8).abs()
                {
                    if y2 > y
                    {
                        if board[x][y + i as usize] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                    else if y2 < y
                    {
                        if board[x][y - i as usize] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                }
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            //if moving vertically
            else if x2 != x && y2 == y
            {
                //dont allow moving vertically if piece is in the path
                for i in 1..(x2 as i8 - x as i8).abs()
                {
                    if x2 > x
                    {
                        if board[x + i as usize][y] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                    else if x2 < x
                    {
                        if board[x - i as usize][y] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                }
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            //if moving diagonally
            else if (x2 as i8 - x as i8).abs() == (y2 as i8 - y as i8).abs()
            {
                //dont allow moving diagonally if piece is in the path
                for i in 1..(x2 as i8 - x as i8).abs()
                {
                    if x2 > x && y2 > y
                    {
                        if board[x + i as usize][y + i as usize] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                    else if x2 < x && y2 < y
                    {
                        if board[x - i as usize][y - i as usize] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                    else if x2 > x && y2 < y
                    {
                        if board[x + i as usize][y - i as usize] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                    else if x2 < x && y2 > y
                    {
                        if board[x - i as usize][y + i as usize] != ' '
                        {
                            println!("Invalid move");
                            continue 'outer;
                        }
                    }
                }
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else
            {
                println!("Invalid move");
                continue;
            }
        }
        //if king
        else if piece == 'K' || piece == 'k'
        {
            //allow castling
            if y2 == y && x == 4 && (x2 == 2 || x2 == 6)
            {
                if white_castle
                {
                    if piece == 'K' && (x2 == 6 && board[7][7] == 'R') || (x2 == 2 && board[0][7] == 'R')
                    {
                        if x2 == 6 && white_right_castle
                        {
                            board[7][7] = ' ';
                            board[5][7] = 'R';
                        }
                        else if x2 == 2 && white_left_castle
                        {
                            board[0][7] = ' ';
                            board[3][7] = 'R';
                        }
                        else
                        {
                            println!("Invalid move");
                            continue;
                        }
                        board[x][y] = ' ';
                        board[x2][y2] = piece;
                    }
                    else if ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 0 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 0)
                    {
                        white_castle = false;
                    }
                }
                if black_castle
                {
                    if piece == 'k' && (x2 == 6 && board[7][0] == 'r') || (x2 == 2 && board[0][0] == 'r')
                    {
                        if x2 == 6 && black_right_castle
                        {
                            board[7][0] = ' ';
                            board[5][0] = 'r';
                        }
                        else if x2 == 2 && black_left_castle
                        {
                            board[0][0] = ' ';
                            board[3][0] = 'r';
                        }
                        else
                        {
                            println!("Invalid move");
                            continue;
                        }
                        board[x][y] = ' ';
                        board[x2][y2] = piece;
                    }
                    else if ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 0 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 0)
                    {
                        black_castle = false;
                    }
                }
            }
            //allow moving one space in any direction
            else if (x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 1
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else if (x2 as i8 - x as i8).abs() == 0 && (y2 as i8 - y as i8).abs() == 1
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else if (x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 0
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
            else
            {
                println!("Invalid move");
                continue;
            }
        }
        else
        {
            println!("Invalid move");
            continue;
        }
        //ensure that the player is not in check after move
        if turn % 2 == 0 && check(board.clone()) == 2
        {
            println!("Invalid move");
            board = copy.clone();
            continue;
        }
        else if turn % 2 == 1 && check(board.clone()) == 1
        {
            println!("Invalid move");
            board = copy.clone();
            continue;
        }
        //delete the first turn of the turn tracker if there are too many to display
        if turn > board.len()
        {
            turns.remove(0);
            turns.push(input.chars().collect());
        }
        else
        {
            turns[turn - 1] = input.chars().collect();
        }
        all_turns.push(input.chars().collect());
        turn += 1;
        print_board(board.clone(), turns.clone(), flip, numbers, keep_flip, turn);
    }
}
fn print_board(board:Vec<Vec<char>>, turns:Vec<Vec<char>>, flip:bool, numbers:bool, keep_flip:bool, turn:usize)
{
    //clear line and move cursor to top left
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for i in 0..board.len()
    {
        let res;
        let ind;
        if flip
        {
            if turn == 1 || turn % 2 == 1
            {
                res = (i as i8 - board.len() as i8).abs();
                ind = i as usize;
            }
            else
            {
                res = i as i8 + 1i8;
                ind = (i as i8 - (board.len() as i8 - 1)).abs() as usize;
            }
        }
        else if keep_flip
        {
            res = i as i8 + 1i8;
            ind = (i as i8 - (board.len() as i8 - 1)).abs() as usize;
        }
        else
        {
            res = (i as i8 - board.len() as i8).abs();
            ind = i as usize;
        }
        let mut col = 'B';
        let mut opp = 'W';
        if turn > 8 && turn % 2 == 0
        {
            col = 'W';
            opp = 'B';
        }
        if board.len() > 8
        {
            print!("{} ", (res as u8 + 96) as char);
        }
        else
        {
            print!("{} ", res);
        }
        if (i + 1) % 2 == 0
        {
            for j in 0..board.len()
            {
                if j % 2 == 0
                {
                    print!("\x1b[100m\x1b[37m {} \x1b[0m", board[j][ind]);
                }
                else
                {
                    print!("\x1b[47m\x1b[90m {} \x1b[0m", board[j][ind]);
                }
            }
            print!(" {} {}{}{}{}", col, turns[i as usize][0], turns[i as usize][1], turns[i as usize][2], turns[i as usize][3]);
        }
        else
        {
            for j in 0..board.len()
            {
                if j % 2 == 0
                {
                    print!("\x1b[47m\x1b[90m {} \x1b[0m", board[j][ind]);
                }
                else
                {
                    print!("\x1b[100m\x1b[37m {} \x1b[0m", board[j][ind]);
                }
            }
            print!(" {} {}{}{}{}", opp, turns[i as usize][0], turns[i as usize][1], turns[i as usize][2], turns[i as usize][3]);
        }
        println!();
    }
    if numbers
    {
        print!(" ");
        for j in 0..board.len()
        {
            print!("  {}", j + 1);
        }
    }
    else
    {
        print!(" ");
        for j in 0..board.len()
        {
            print!("  {}", (j as u8 + 97) as char);
        }
    }
    println!();
}
fn check(board:Vec<Vec<char>>) -> u8
{
    // if no_check
    //     return 0
    // if white_check
    //     return 1
    // if black_check
    //     return 2;
    // if checkmate
    //     return 3;
    // if stalemate
    //     return 4;
    for i in 0..board.len()
    {
        for j in 0..board.len()
        {
            if board[i][j] == 'K'
            {
                //check if white king is in check via pawn
                if (j < board.len() - 1 && i < board.len() - 1 && board[i + 1][j + 1] == 'p') || (j < board.len() - 1 && i != 0 && board[i - 1][j + 1] == 'p')
                {
                    return 1;
                }
                //check if white king is in check via knight
                else if (i < board.len() - 2 && j < board.len() - 1 && board[i + 2][j + 1] == 'n')
                          || (i < board.len() - 2 && j != 0 && board[i + 2][j - 1] == 'n')
                          || (i < board.len() - 1 && j < board.len() - 2 && board[i + 1][j + 2] == 'n')
                          || (i != 0 && j < board.len() - 2 && board[i - 1][j + 2] == 'n')
                          || (i != 0 && j > 1 && board[i - 1][j - 2] == 'n')
                          || (i < board.len() - 1 && j > 1 && board[i + 1][j - 2] == 'n')
                          || (i > 1 && j != 0 && board[i - 2][j - 1] == 'n')
                          || (i > 1 && j < board.len() - 1 && board[i - 2][j + 1] == 'n')
                {
                    return 1;
                }
                else
                {
                    for x in 0..board.len()
                    {
                        for y in 0..board.len()
                        {
                            if board[x][y] == 'b' || board[x][y] == 'q' || board[x][y] == 'r'
                            {
                                //check if white king is in check via rook/queen horizontally
                                if x == i
                                {
                                    if y < j
                                    {
                                        for z in y..j
                                        {
                                            if board[x][z] != ' '
                                            {
                                                break;
                                            }
                                            if z == j - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                    else
                                    {
                                        for z in j..y
                                        {
                                            if board[x][z] != ' '
                                            {
                                                break;
                                            }
                                            if z == y - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                }
                                //check if white king is in check via rook/queen vertically
                                else if y == j
                                {
                                    if x < i
                                    {
                                        for z in x..i
                                        {
                                            if board[z][y] != ' '
                                            {
                                                break;
                                            }
                                            if z == i - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                    else
                                    {
                                        for z in i..x
                                        {
                                            if board[z][y] != ' '
                                            {
                                                break;
                                            }
                                            if z == x - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                }
                                //check if white king is in check via bishop/queen diagonally
                                else if (x as i8 - i as i8).abs() == (y as i8 - j as i8).abs()
                                {
                                    if x < i && y < j
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize + z as usize][y as usize + z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                    else if x < i && y > j
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize + z as usize][y as usize - z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                    else if x > i && y < j
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize - z as usize][y as usize + z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                    else
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize - z as usize][y as usize - z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            else if board[i][j] == 'k'
            {
                //check if black king is in check via pawn
                if (j < board.len() - 1 && i < board.len() - 1 && board[i + 1][j + 1] == 'P') || (j < board.len() - 1 && i != 0 && board[i - 1][j + 1] == 'P')
                {
                    return 2;
                }
                //check if black king is in check via knight
                else if (i < board.len() - 2 && j < board.len() - 1 && board[i + 2][j + 1] == 'N')
                          || (i < board.len() - 2 && j != 0 && board[i + 2][j - 1] == 'N')
                          || (i < board.len() - 1 && j < board.len() - 2 && board[i + 1][j + 2] == 'N')
                          || (i != 0 && j < board.len() - 2 && board[i - 1][j + 2] == 'N')
                          || (i != 0 && j > 1 && board[i - 1][j - 2] == 'N')
                          || (i < board.len() - 1 && j > 1 && board[i + 1][j - 2] == 'N')
                          || (i > 1 && j != 0 && board[i - 2][j - 1] == 'N')
                          || (i > 1 && j < board.len() - 1 && board[i - 2][j + 1] == 'N')
                {
                    return 2;
                }
                else
                {
                    for x in 0..board.len()
                    {
                        for y in 0..board.len()
                        {
                            if board[x][y] == 'B' || board[x][y] == 'Q' || board[x][y] == 'R'
                            {
                                //check if black king is in check via rook/queen horizontally
                                if x == i
                                {
                                    if y < j
                                    {
                                        for z in y..j
                                        {
                                            if board[x][z] != ' '
                                            {
                                                break;
                                            }
                                            if z == j - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                    else
                                    {
                                        for z in j..y
                                        {
                                            if board[x][z] != ' '
                                            {
                                                break;
                                            }
                                            if z == y - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                }
                                //check if black king is in check via rook/queen vertically
                                else if y == j
                                {
                                    if x < i
                                    {
                                        for z in x..i
                                        {
                                            if board[z][y] != ' '
                                            {
                                                break;
                                            }
                                            if z == i - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                    else
                                    {
                                        for z in i..x
                                        {
                                            if board[z][y] != ' '
                                            {
                                                break;
                                            }
                                            if z == x - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                }
                                //check if black king is in check via bishop/queen diagonally
                                else if (x as i8 - i as i8).abs() == (y as i8 - j as i8).abs()
                                {
                                    if x < i && y < j
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize + z as usize][y as usize + z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                    else if x < i && y > j
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize + z as usize][y as usize - z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                    else if x > i && y < j
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize - z as usize][y as usize + z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                    else
                                    {
                                        for z in 1..(x as i8 - i as i8).abs()
                                        {
                                            if board[x as usize - z as usize][y as usize - z as usize] != ' '
                                            {
                                                break;
                                            }
                                            if z == (x as i8 - i as i8).abs() - 1
                                            {
                                                return 2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0;
}