fn main()
{
    // create functions to output all available moves for a piece
    // TODO: implement move suggestions
    // TODO: checkmate
    // TODO: stalemate
    // TODO: implement networking
    // TODO: implement stock fish
    let mut flip = false;
    let mut numbers = false;
    let mut keep_flip = false;
    //    let mut bot = true;
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
            //            println!("--no_bot will disable the bot");
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
        //        else if std::env::args().nth(i).unwrap() == "--no_bot"
        //        {
        //           bot = false;
        //      }
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
    //castling stuff castle[0]= white left castle, castle[1] = white right castle, castle[2] = black left castle, castle[3] = black right castle, castle[4] = white castle, castle[5] = black castle
    let mut castle:Vec<bool> = vec![true; 6];
    let mut copy:Vec<Vec<char>>;
    //en passant stuff
    let mut passant = [0; 3];
    //let mut instant = std::time::Instant::now();
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
        match check(board.clone(), turn)
        {
            1 => println!("White is in check"),
            2 => println!("Black is in check"),
            3 => println!("Checkmate"),
            4 => println!("Stalemate"),
            _ => (),
        }
        copy = board.clone();
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
        //        if bot && turn % 2 == 0
        //        {
        //            input = bot_move(board.clone());
        //        }
        //       else
        //        {
        //println!("{}", instant.elapsed().as_nanos());
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        //instant = std::time::Instant::now();
        //        }
        //turn input from a2a4 to [1,2,1,4]
        let moves:Vec<u8> = input.chars()
                                 .filter_map(|c| {
                                     match c
                                     {
                                         'a'..='t' => Some(c as u8 - b'a' + 1),
                                         'A'..='Z' => Some(c as u8 - b'A' + 27),
                                         '0'..='9' => c.to_digit(10).map(|d| d as u8),
                                         _ => None,
                                     }
                                 })
                                 .collect();
        if moves.len() == 0
        {
            println!("Invalid input");
            continue;
        }
        if moves[0] == 31 && moves[1] == 50 && moves[2] == 35 && moves[3] == 46
        {
            for i in 0..all_turns.len()
            {
                for j in 0..all_turns[i].len()
                {
                    print!("{}", all_turns[i][j]);
                }
                println!();
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
        let mut success = false;
        //pawn movement
        if piece.eq_ignore_ascii_case(&'p')
        {
            let possible_moves = pawn(board.clone(), x, y, Some(passant));
            println!("{:?}", possible_moves);
            for row in possible_moves.iter()
            {
                let mut iter = row.iter().peekable();
                while let Some(&value) = iter.next()
                {
                    if value == x2 as u8
                    {
                        if iter.peek() == Some(&&(y2 as u8))
                        {
                            if y2 as i8 == y as i8 + 2
                            {
                                passant = [x2, y2, turn];
                            }
                            else if y2 as i8 == y as i8 - 2
                            {
                                passant = [x2, y2, turn];
                            }
                            success = true;
                            if board[x2][y2] == ' '
                            {
                                if x2 == x + 1 && y2 == y + 1
                                {
                                    board[x2][y2 - 1] = ' ';
                                }
                                else if x2 as i8 == x as i8 - 1 && y2 == y + 1
                                {
                                    board[x2][y2 - 1] = ' ';
                                }
                                else if x2 == x + 1 && y2 as i8 == y as i8 - 1
                                {
                                    board[x2][y2 + 1] = ' ';
                                }
                                else if x2 as i8 == x as i8 - 1 && y2 as i8 == y as i8 - 1
                                {
                                    board[x2][y2 + 1] = ' ';
                                }
                            }
                            board[x2][y2] = piece;
                            board[x][y] = ' ';
                            break;
                        }
                    }
                }
            }
            if !success
            {
                println!("Invalid move");
                continue;
            }
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
        //if rook
        else if piece.eq_ignore_ascii_case(&'r')
        {
            let possible_moves = rook(board.clone(), x, y);
            println!("{:?}", possible_moves);
            for row in possible_moves.iter()
            {
                let mut iter = row.iter().peekable();
                while let Some(&value) = iter.next()
                {
                    if value == x2 as u8
                    {
                        if iter.peek() == Some(&&(y2 as u8))
                        {
                            success = true;
                            board[x2][y2] = piece;
                            board[x][y] = ' ';
                            break;
                        }
                    }
                }
            }
            if !success
            {
                println!("Invalid move");
                continue;
            }
            if x == 0 && piece == 'R'
            {
                castle[0] = false; //disable white left castle
            }
            else if x == 7 && piece == 'R'
            {
                castle[1] = false; //disable white right castle
            }
            else if x == 0 && piece == 'r'
            {
                castle[2] = false; //disable black left castle
            }
            else if x == 7 && piece == 'r'
            {
                castle[3] = false; //disable black right castle
            }
        }
        //if bishop
        else if piece.eq_ignore_ascii_case(&'b')
        {
            let possible_moves = bishop(board.clone(), x, y);
            println!("{:?}", possible_moves);
            for row in possible_moves.iter()
            {
                let mut iter = row.iter().peekable();
                while let Some(&value) = iter.next()
                {
                    if value == x2 as u8
                    {
                        if iter.peek() == Some(&&(y2 as u8))
                        {
                            success = true;
                            board[x2][y2] = piece;
                            board[x][y] = ' ';
                            break;
                        }
                    }
                }
            }
            if !success
            {
                println!("Invalid move");
                continue;
            }
        }
        //if knight
        else if piece.eq_ignore_ascii_case(&'n')
        {
            let possible_moves = knight(board.clone(), x, y);
            println!("{:?}", possible_moves);
            for row in possible_moves.iter()
            {
                let mut iter = row.iter().peekable();
                while let Some(&value) = iter.next()
                {
                    if value == x2 as u8
                    {
                        if iter.peek() == Some(&&(y2 as u8))
                        {
                            success = true;
                            board[x2][y2] = piece;
                            board[x][y] = ' ';
                            break;
                        }
                    }
                }
            }
            if !success
            {
                println!("Invalid move");
                continue;
            }
        }
        //if queen
        else if piece.eq_ignore_ascii_case(&'q')
        {
            //just use rook and bishop logic together
            let mut possible_moves = bishop(board.clone(), x, y);
            possible_moves.extend(rook(board.clone(), x, y));
            println!("{:?}", possible_moves);
            for row in possible_moves.iter()
            {
                let mut iter = row.iter().peekable();
                while let Some(&value) = iter.next()
                {
                    if value == x2 as u8
                    {
                        if iter.peek() == Some(&&(y2 as u8))
                        {
                            success = true;
                            board[x2][y2] = piece;
                            board[x][y] = ' ';
                            break;
                        }
                    }
                }
            }
            if !success
            {
                println!("Invalid move");
                continue;
            }
        }
        //if king
        else if piece.eq_ignore_ascii_case(&'k')
        {
            let possible_moves = king(board.clone(), x, y, Some(castle.clone()));
            println!("{:?}", possible_moves);
            for row in possible_moves.iter()
            {
                let mut iter = row.iter().peekable();
                while let Some(&value) = iter.next()
                {
                    if value == x2 as u8
                    {
                        if iter.peek() == Some(&&(y2 as u8))
                        {
                            if y2 == y && x == 4 && (x2 == 2 || x2 == 6)
                            {
                                if castle[4]
                                //make sure whites castle still has not been done
                                {
                                    if piece == 'K' && (x2 == 6 && board[7][7] == 'R') || (x2 == 2 && board[0][7] == 'R')
                                    {
                                        if x2 == 6 && castle[1]
                                        {
                                            board[7][7] = ' ';
                                            board[5][7] = 'R';
                                        }
                                        else if x2 == 2 && castle[0]
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
                                        castle[4] = false;
                                    }
                                }
                                if castle[5]
                                //make sure blacks castle still has not been done
                                {
                                    if piece == 'k' && (x2 == 6 && board[7][0] == 'r') || (x2 == 2 && board[0][0] == 'r')
                                    {
                                        if x2 == 6 && castle[3]
                                        {
                                            board[7][0] = ' ';
                                            board[5][0] = 'r';
                                        }
                                        else if x2 == 2 && castle[2]
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
                                        castle[5] = false;
                                    }
                                }
                            }
                            success = true;
                            board[x2][y2] = piece;
                            board[x][y] = ' ';
                            break;
                        }
                    }
                }
            }
            if !success
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
        let is_check = check(board.clone(), turn);
        if turn % 2 == 0 && is_check == 2
        {
            println!("cant move in check");
            board = copy.clone();
            continue;
        }
        else if turn % 2 == 1 && is_check == 1
        {
            println!("cant move in check");
            board = copy.clone();
            continue;
        }
        let turn_str:Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
        //delete the first turn of the turn tracker if there are too many to display
        if turn > board.len()
        {
            turns.remove(0);
            turns.push(turn_str.clone());
        }
        else
        {
            turns[turn - 1] = turn_str.clone();
        }
        all_turns.push(turn_str);
        turn += 1;
        print_board(board.clone(), turns.clone(), flip, numbers, keep_flip, turn);
    }
}
fn king(board:Vec<Vec<char>>, x:usize, y:usize, castle:Option<Vec<bool>>) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    let row:usize;
    let first:usize;
    let second:usize;
    let third:usize;
    if piece.is_uppercase()
    {
        row = 7;
        first = 4; //make sure king has not moved
        second = 0; //make sure left rook has not moved
        third = 1; //make sure right rook has not moved
    }
    else
    {
        row = 0;
        first = 5; //make sure king has not moved
        second = 2; //make sure left rook has not moved
        third = 3; //make sure right rook has not moved
    }
    for x2 in 0..board.len()
    {
        for y2 in 0..board.len()
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            //allow castling
            if let Some(ref castle) = castle
            {
                if y == row && y2 == y && x == 4 && (x2 == 2 || x2 == 6) && castle[first] && castle[second] && castle[third]
                {
                    possible_moves.push(vec![x2 as u8, y2 as u8]);
                }
            }
            //allow moving one space in any direction
            if ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 0 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 0)
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    return possible_moves;
}
fn knight(board:Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    for x2 in 0..board.len()
    {
        for y2 in 0..board.len()
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            //only allow moving in an L shape
            if ((x2 as i8 - x as i8).abs() == 2 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 2)
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    return possible_moves;
}
fn bishop(board:Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    'outer: for x2 in 0..board.len()
    {
        for y2 in 0..board.len()
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            //dont allow moving if piece is in the path
            for i in 1..(x2 as i8 - x as i8).abs()
            {
                if x2 > x && y2 > y && (y + i as usize) < board.len() && (x + i as usize) < board.len()
                {
                    if board[x + i as usize][y + i as usize] != ' '
                    {
                        continue 'outer;
                    }
                }
                else if x2 < x && y2 < y && i < y.try_into().unwrap() && i < x.try_into().unwrap()
                {
                    if board[x - i as usize][y - i as usize] != ' '
                    {
                        continue 'outer;
                    }
                }
                else if x2 > x && y2 < y && i < y.try_into().unwrap() && (x + i as usize) < board.len()
                {
                    if board[x + i as usize][y - i as usize] != ' '
                    {
                        continue 'outer;
                    }
                }
                else if x2 < x && y2 > y && i < x.try_into().unwrap() && (y + i as usize) < board.len()
                {
                    if board[x - i as usize][y + i as usize] != ' '
                    {
                        continue 'outer;
                    }
                }
            }
            //only allow moving diagonally
            if (x2 as i8 - x as i8).abs() == (y2 as i8 - y as i8).abs()
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    return possible_moves;
}
fn rook(board:Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    'outer: for x2 in 0..board.len()
    {
        'inner: for y2 in 0..board.len()
        {
            let piece2 = board[x2][y2];
            if piece2.is_uppercase() && piece.is_uppercase() || piece2.is_lowercase() && piece.is_lowercase()
            {
                continue;
            }
            //dont allow moving horizontally if piece is in the path
            for i in 1..(x2 as i8 - x as i8).abs()
            {
                if x2 > x
                {
                    if board[x + i as usize][y] != ' '
                    {
                        continue 'outer;
                    }
                }
                else if x2 < x
                {
                    if board[x - i as usize][y] != ' '
                    {
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
                        continue 'inner;
                    }
                }
                else if y2 < y
                {
                    if board[x][y - i as usize] != ' '
                    {
                        continue 'inner;
                    }
                }
            }
            if (x2 == x && y2 != y) || (x2 != x && y2 == y)
            {
                possible_moves.push(vec![x2 as u8, y2 as u8]);
            }
        }
    }
    return possible_moves;
}
fn pawn(board:Vec<Vec<char>>, x:usize, y:usize, passant:Option<[usize; 3]>) -> Vec<Vec<u8>>
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
    let mut possible_moves:Vec<Vec<u8>> = vec![];
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
            if y == start && y2 as i8 == y as i8 + (2 * direction) && x2 == x && piece2 == ' '
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
//fn bot_move(board:Vec<Vec<char>>) -> String
//{
//}
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
        let mut color;
        if (i + 1) % 2 == 0
        {
            for j in 0..board.len()
            {
                if board[j][ind].is_uppercase()
                {
                    color = 97;
                }
                else
                {
                    color = 30;
                }
                if j % 2 == 0
                {
                    print!("\x1b[100m\x1b[{}m {} \x1b[0m", color, board[j][ind]);
                }
                else
                {
                    print!("\x1b[47m\x1b[{}m {} \x1b[0m", color, board[j][ind]);
                }
            }
            print!(" {} {}{}{}{}", col, turns[i as usize][0], turns[i as usize][1], turns[i as usize][2], turns[i as usize][3]);
        }
        else
        {
            for j in 0..board.len()
            {
                if board[j][ind].is_uppercase()
                {
                    color = 97;
                }
                else
                {
                    color = 30;
                }
                if j % 2 == 0
                {
                    print!("\x1b[47m\x1b[{}m {} \x1b[0m", color, board[j][ind]);
                }
                else
                {
                    print!("\x1b[100m\x1b[{}m {} \x1b[0m", color, board[j][ind]);
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
fn check(board:Vec<Vec<char>>, turn:usize) -> u8
{
    // if no_check
    //     return 0
    // if white_check
    //     return 1
    // if black_check
    //     return 2
    // if checkmate
    //     return 3
    // if stalemate
    //     return 4
    let mut white_check = false;
    let mut black_check = false;
    let checkmate = false;
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    for x in 0..board.len()
    {
        for y in 0..board.len()
        {
            if board[x][y].eq_ignore_ascii_case(&'p')
            {
                possible_moves.extend(pawn(board.clone(), x, y, None));
            }
            else if board[x][y].eq_ignore_ascii_case(&'r')
            {
                possible_moves.extend(rook(board.clone(), x, y));
            }
            else if board[x][y].eq_ignore_ascii_case(&'n')
            {
                possible_moves.extend(knight(board.clone(), x, y));
            }
            else if board[x][y].eq_ignore_ascii_case(&'b')
            {
                possible_moves.extend(bishop(board.clone(), x, y));
            }
            else if board[x][y].eq_ignore_ascii_case(&'q')
            {
                possible_moves.extend(bishop(board.clone(), x, y));
                possible_moves.extend(rook(board.clone(), x, y));
            }
            else if board[x][y].eq_ignore_ascii_case(&'k')
            {
                possible_moves.extend(king(board.clone(), x, y, None));
            }
        }
    }
    for x in 0..board.len()
    {
        for y in 0..board.len()
        {
            if board[x][y].eq_ignore_ascii_case(&'k')
            {
                //check for check
                for row in possible_moves.iter()
                {
                    let mut iter = row.iter().peekable();
                    while let Some(&value) = iter.next()
                    {
                        if value == x as u8
                        {
                            if iter.peek() == Some(&&(y as u8))
                            {
                                if board[x][y].is_uppercase()
                                {
                                    white_check = true;
                                }
                                else
                                {
                                    black_check = true;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    if checkmate
    {
        return 3;
    }
    else if turn % 2 == 1 && white_check
    {
        return 1;
    }
    else if turn % 2 == 0 && black_check
    {
        return 2;
    }
    return 0;
}