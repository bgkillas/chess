#[cfg(unix)]
use libc::{tcgetattr, tcsetattr, ECHO, ICANON, TCSANOW, VMIN, VTIME};
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
//TODO: implement bots
fn main()
{
    let mut flip = false;
    let mut numbers = false;
    let mut keep_flip = false;
    let mut file = String::new();
    let mut color = 0;
    let mut ip = String::new();
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
            println!("--black will make you play as black");
            println!("--ip IP will connect to a server at IP:port");
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
        else if std::env::args().nth(i).unwrap() == "--black"
        {
            color = 1;
            keep_flip = true;
        }
        else if std::env::args().nth(i).unwrap() == "--ip"
        {
            ip = std::env::args().nth(i + 1).unwrap();
        }
    }
    let mut board:Vec<Vec<char>>;
    if file != "" && std::path::Path::new(&file).exists()
    {
        let csv = std::fs::File::open(file).unwrap();
        let reader = std::io::BufReader::new(csv);
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
    if color == 1 && ip == ""
    {
        turn = 2;
    }
    print_board(board.clone(), turns.clone(), flip, numbers, keep_flip, turn, None);
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
        match check(board.clone(), turn, true)
        {
            1 => println!("White is in check"),
            2 => println!("Black is in check"),
            3 =>
            {
                println!("Checkmate {} wins", if turn % 2 == 0 { "White" } else { "Black" });
                write_all_turns(&all_turns);
            }
            4 =>
            {
                println!("Stalemate");
                write_all_turns(&all_turns);
            }
            _ => (),
        }
        let mut are_you_moving = false;
        if ip == ""
        {
            are_you_moving = true;
        }
        else if turn % 2 == 0 && color == 1
        {
            are_you_moving = true;
        }
        else if turn % 2 == 1 && color == 1
        {
            are_you_moving = false;
        }
        else if turn % 2 == 0 && color == 0
        {
            are_you_moving = false;
        }
        else if turn % 2 == 1 && color == 0
        {
            are_you_moving = true;
        }
        let mut input = String::new();
        //println!("{}", instant.elapsed().as_nanos());
        if are_you_moving
        {
            #[cfg(target_os = "windows")]
            {
                std::io::stdin().read_line(&mut input).unwrap();
            }
            #[cfg(unix)]
            {
                loop
                {
                    let move_char = read_single_char();
                    print!("{}", move_char);
                    std::io::stdout().flush().unwrap();
                    if input.len() == 1 && move_char != '\u{7f}'
                    {
                        let mut piece_moves:Vec<Vec<u8>>;
                        let x:usize = input.chars()
                                           .filter_map(|c| {
                                               match c
                                               {
                                                   'a'..='t' => Some(c as u8 - b'a' + 1),
                                                   'A'..='Z' => Some(c as u8 - b'A' + 27),
                                                   '0'..='9' => c.to_digit(10).map(|d| d as u8),
                                                   _ => None,
                                               }
                                           })
                                           .nth(0)
                                           .map(|val| val as usize - 1)
                                           .unwrap_or_default();
                        let y:usize = (move_char.to_string()
                                                .chars()
                                                .filter_map(|c| {
                                                    match c
                                                    {
                                                        'a'..='t' => Some(c as u8 - b'a' + 1),
                                                        'A'..='Z' => Some(c as u8 - b'A' + 27),
                                                        '0'..='9' => c.to_digit(10).map(|d| d as u8),
                                                        _ => None,
                                                    }
                                                })
                                                .nth(0)
                                                .map(|val| val as i8 - board.len() as i8)
                                                .unwrap_or_default()).abs() as usize;
                        if input == "E" && move_char == 'X'
                        {
                            println!();
                            write_all_turns(&all_turns);
                            std::process::exit(0);
                        }
                        if x >= board.len() || y >= board.len()
                        {
                            println!("Invalid move");
                            input = String::new();
                            continue;
                        }
                        match board[x][y]
                        {
                            'P' => piece_moves = pawn(board.clone(), x, y, Some(passant)),
                            'p' => piece_moves = pawn(board.clone(), x, y, Some(passant)),
                            'R' => piece_moves = rook(board.clone(), x, y),
                            'r' => piece_moves = rook(board.clone(), x, y),
                            'N' => piece_moves = knight(board.clone(), x, y),
                            'n' => piece_moves = knight(board.clone(), x, y),
                            'B' => piece_moves = bishop(board.clone(), x, y),
                            'b' => piece_moves = bishop(board.clone(), x, y),
                            'Q' =>
                            {
                                let mut bishop_moves:Vec<Vec<u8>> = bishop(board.clone(), x, y);
                                let mut rook_moves:Vec<Vec<u8>> = rook(board.clone(), x, y);
                                rook_moves.remove(0);
                                bishop_moves.extend(rook_moves);
                                piece_moves = bishop_moves;
                            }
                            'q' =>
                            {
                                let mut bishop_moves:Vec<Vec<u8>> = bishop(board.clone(), x, y);
                                let mut rook_moves:Vec<Vec<u8>> = rook(board.clone(), x, y);
                                rook_moves.remove(0);
                                bishop_moves.extend(rook_moves);
                                piece_moves = bishop_moves;
                            }
                            'K' => piece_moves = king(board.clone(), x, y, Some(castle.clone())),
                            'k' => piece_moves = king(board.clone(), x, y, Some(castle.clone())),
                            _ =>
                            {
                                println!("Invalid move");
                                continue;
                            }
                        }
                        piece_moves.remove(0);
                        print_board(board.clone(), turns.clone(), flip, numbers, keep_flip, turn, Some(piece_moves));
                        println!();
                        if turn % 2 == 0
                        {
                            println!("Black's turn");
                        }
                        else
                        {
                            println!("White's turn");
                        }
                        println!("Enter a move: ");
                        print!("{}{}", input, move_char);
                        std::io::stdout().flush().unwrap();
                    }
                    if move_char == '\u{7f}'
                    {
                        print!("{}", '\x08');
                        std::io::stdout().flush().unwrap();
                        input.pop();
                    }
                    else
                    {
                        input += &move_char.to_string();
                    }
                    if move_char == '\n'
                    {
                        break;
                    }
                }
            }
        }
        else if ip != ""
        {
            input = receive_data(ip.splitn(2, ':').nth(1).unwrap().parse::<u16>().unwrap()).unwrap();
        }
        if ip != ""
        {
            match send_data(input.clone(), &ip)
            {
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
        //instant = std::time::Instant::now();
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
            write_all_turns(&all_turns);
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
        let is_check = check(board.clone(), turn, false);
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
        print_board(board.clone(), turns.clone(), flip, numbers, keep_flip, turn, None);
    }
}
fn write_all_turns(all_turns:&Vec<Vec<char>>)
{
    for i in 1..all_turns.len()
    {
        for j in 0..all_turns[i].len()
        {
            print!("{}", all_turns[i][j]);
        }
        print!(" ");
    }
    println!();
    std::process::exit(0);
}
fn king(board:Vec<Vec<char>>, x:usize, y:usize, castle:Option<Vec<bool>>) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![vec![x as u8, y as u8]];
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
    fn is_path_blocked(board:Vec<Vec<char>>, start:(usize, usize), end:(usize, usize)) -> bool
    {
        let (x1, y1) = start;
        let (x2, y2) = end;
        let delta_x:i8 = if x1 < x2 { 1 } else { -1 };
        let delta_y:i8 = if y1 < y2 { 1 } else { -1 };
        let mut x:i8 = x1 as i8 + delta_x as i8;
        let mut y:i8 = y1 as i8 + delta_y as i8;
        while x != x2 as i8 && y != y2 as i8
        {
            if board[x as usize][y as usize] != ' '
            {
                return true;
            }
            x += delta_x;
            y += delta_y;
        }
        return false;
    }
    let piece = board[x][y];
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
            if (x2 as i8 - x as i8).abs() == (y2 as i8 - y as i8).abs()
            {
                if !is_path_blocked(board.clone(), (x, y), (x2, y2))
                {
                    possible_moves.push(vec![x2 as u8, y2 as u8]);
                }
            }
        }
    }
    return possible_moves;
}
fn rook(board:Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<u8>>
{
    let piece = board[x][y];
    let mut possible_moves:Vec<Vec<u8>> = vec![vec![x as u8, y as u8]];
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
fn print_board(board:Vec<Vec<char>>, turns:Vec<Vec<char>>, flip:bool, numbers:bool, keep_flip:bool, turn:usize, moves:Option<Vec<Vec<u8>>>)
{
    //clear line and move cursor to top left
    print!("\n{esc}[2J{esc}[1;1H", esc = 27 as char);
    for x in 0..board.len()
    {
        let res;
        let ind;
        if flip
        {
            if turn == 1 || turn % 2 == 1
            {
                res = (x as i8 - board.len() as i8).abs();
                ind = x as usize;
            }
            else
            {
                res = x as i8 + 1i8;
                ind = (x as i8 - (board.len() as i8 - 1)).abs() as usize;
            }
        }
        else if keep_flip
        {
            res = x as i8 + 1;
            ind = (x as i8 - (board.len() as i8 - 1)).abs() as usize;
        }
        else
        {
            res = (x as i8 - board.len() as i8).abs();
            ind = x as usize;
        }
        let mut col = 'W';
        if turn > 8 && turn % 2 == 0
        {
            col = 'B';
        }
        if (x + 1) % 2 == 0
        {
            if col == 'W'
            {
                col = 'B';
            }
            else if col == 'B'
            {
                col = 'W';
            }
        }
        if board.len() > 8
        {
            print!("{} ", (res as u8 + 96) as char);
        }
        else
        {
            print!("{} ", res);
        }
        let mut fg_color:u8;
        let mut bg_color:u8;
        'inner: for y in 0..board.len()
        {
            if let Some(ref moves) = moves
            {
                for i in 0..moves.len()
                {
                    let mut x2 = x;
                    if keep_flip
                    {
                        x2 = (x as i8 - (board.len() as i8 - 1)).abs() as usize;
                    }
                    if moves[i][0] == y as u8 && moves[i][1] == x2 as u8
                    {
                        print!("\x1b[48;5;226m\x1b[30m {} \x1b[0m", board[y][ind]);
                        continue 'inner;
                    }
                }
            }
            if board[y][ind].is_uppercase()
            {
                fg_color = 97;
            }
            else
            {
                fg_color = 30;
            }
            if (y + ((x + 1) % 2)) % 2 == 0
            {
                bg_color = 100;
            }
            else
            {
                bg_color = 47;
            }
            print!("\x1b[{}m\x1b[{}m {} \x1b[0m", bg_color, fg_color, board[y][ind]);
        }
        println!(" {} {}{}{}{}", col, turns[x as usize][0], turns[x as usize][1], turns[x as usize][2], turns[x as usize][3]);
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
fn check(board:Vec<Vec<char>>, turn:usize, checkmate:bool) -> u8
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
    //moves[0][0][0][0] = white pawn 1 move 1
    //moves[0][0][0][0][0] = white pawn 1 move 1 x
    //moves[0][0][0][0][1] = white pawn 1 move 1 y
    for x in 0..board.len()
    {
        for y in 0..board.len()
        {
            if board[x][y] != ' '
            {
                let num;
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
                    moves[num][0].push(pawn(board.clone(), x, y, None));
                }
                else if board[x][y].eq_ignore_ascii_case(&'r')
                {
                    moves[num][1].push(rook(board.clone(), x, y));
                }
                else if board[x][y].eq_ignore_ascii_case(&'n')
                {
                    moves[num][2].push(knight(board.clone(), x, y));
                }
                else if board[x][y].eq_ignore_ascii_case(&'b')
                {
                    moves[num][3].push(bishop(board.clone(), x, y));
                }
                else if board[x][y].eq_ignore_ascii_case(&'q')
                {
                    let mut bishop_moves:Vec<Vec<u8>> = bishop(board.clone(), x, y);
                    let mut rook_moves:Vec<Vec<u8>> = rook(board.clone(), x, y);
                    rook_moves.remove(0);
                    bishop_moves.extend(rook_moves);
                    moves[num][4].push(bishop_moves);
                }
                else if board[x][y].eq_ignore_ascii_case(&'k')
                {
                    moves[num][5].push(king(board.clone(), x, y, None));
                }
            }
        }
    }
    let mut possible_moves:Vec<Vec<u8>> = vec![];
    for i in 0..2
    {
        for j in 0..6
        {
            for k in 0..moves[i][j].len()
            {
                possible_moves.extend(moves[i][j][k][1..].to_vec());
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
                if checkmate
                {
                    for color in 0..2
                    {
                        let mut num_of_checks:Vec<u8> = vec![0, 0];
                        for piece in 0..6
                        {
                            for piece_moves in 0..moves[color][piece].len()
                            {
                                for i in 1..moves[color][piece][piece_moves].len()
                                {
                                    let mut copy = board.clone();
                                    copy[moves[color][piece][piece_moves][i][0] as usize][moves[color][piece][piece_moves][i][1] as usize] = copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize];
                                    copy[moves[color][piece][piece_moves][0][0] as usize][moves[color][piece][piece_moves][0][1] as usize] = ' ';
                                    num_of_checks[0] += 1;
                                    if check(copy, turn, false) == (1 + color) as u8
                                    {
                                        num_of_checks[1] += 1;
                                    }
                                }
                            }
                        }
                        if num_of_checks[0] == num_of_checks[1]
                        {
                            if !white_check && !black_check
                            {
                                return 4;
                            }
                            return 3;
                        }
                    }
                }
            }
        }
    }
    if turn % 2 == 1 && white_check
    {
        return 1;
    }
    else if turn % 2 == 0 && black_check
    {
        return 2;
    }
    return 0;
}
#[cfg(unix)]
pub fn read_single_char() -> char
{
    let stdin_fd = std::io::stdin().as_raw_fd();
    let orig_termios = unsafe {
        let mut termios = std::mem::zeroed();
        tcgetattr(stdin_fd, &mut termios);
        termios
    };
    let mut new_termios = orig_termios;
    new_termios.c_lflag &= !(ICANON | ECHO);
    new_termios.c_cc[VMIN] = 1;
    new_termios.c_cc[VTIME] = 0;
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &new_termios);
    }
    let mut input = [0u8; 1];
    std::io::stdin().read_exact(&mut input).unwrap();
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &orig_termios);
    }
    return input[0] as char;
}
fn send_data(moves:String, addr:&str) -> std::io::Result<String>
{
    let mut stream = std::net::TcpStream::connect(addr)?;
    stream.write_all(moves.as_bytes())?;
    let mut buf = [0; 3];
    stream.read_exact(&mut buf)?;
    let message = String::from_utf8_lossy(&buf).to_string();
    Ok(message)
}
fn receive_data(port:u16) -> std::io::Result<String>
{
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))?;
    for stream in listener.incoming()
    {
        let mut stream = stream?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        let message = String::from_utf8_lossy(&buf).to_string();
        println!("Received message: {}", message);
        stream.write_all(b"ACK")?;
        return Ok(message);
    }
    unreachable!()
}