use console::{Key, Term};
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
mod pieces
{
    pub mod bishop;
    pub mod king;
    pub mod knight;
    pub mod pawn;
    pub mod rook;
}
pub mod bot;
pub mod check;
pub mod possible_moves;
pub mod print_board;
use pieces::*;
use check::*;
use print_board::*;
use bot::*;
fn main()
{
    let mut flip = false;
    let mut numbers = false;
    let mut keep_flip = false;
    let mut file = String::new();
    let mut color = 0;
    let mut ip = String::new();
    let mut bot = false;
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
            println!("--bot will play against a bot");
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
            bot = false;
            ip = std::env::args().nth(i + 1).unwrap();
        }
        else if std::env::args().nth(i).unwrap() == "--bot"
        {
            bot = true;
        }
    }
    let mut board:Vec<Vec<char>>;
    if !file.is_empty() && std::path::Path::new(&file).exists()
    {
        let csv = std::fs::File::open(file).unwrap();
        let reader = std::io::BufReader::new(csv);
        board = reader.lines().map(|l| l.unwrap().split(',').map(|c| c.chars().next().unwrap()).collect()).collect();
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
    if color == 1 && ip.is_empty()
    {
        turn = 2;
    }
    print_board(board.clone(), &turns, flip, numbers, keep_flip, turn, None);
    //castling stuff castle[0]= white left castle, castle[1] = white right castle, castle[2] = black left castle, castle[3] = black right castle, castle[4] = white castle, castle[5] = black castle
    let mut castle:Vec<bool> = vec![true; 6];
    let mut copy:Vec<Vec<char>>;
    //en passant stuff
    let mut passant = [0; 3];
    //let mut instant = std::time::Instant::now();
    loop
    {
        //dont allow en passant on a piece after a turn
        if turn != passant[2] + 1
        {
            passant[0] = 0;
            passant[1] = 0;
            passant[2] = 0;
        }
        println!();
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
        if turn > 2
        {
            match check(&board, turn, true)
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
        }
        let mut are_you_moving = false;
        if (ip.is_empty() && !bot) || (turn % 2 == 0 && color == 1) || (turn % 2 == 1 && color == 0)
        {
            are_you_moving = true;
        }
        else if (turn % 2 == 1 && color == 1) || (turn % 2 == 0 && color == 0)
        {
            are_you_moving = false;
        }
        let mut input = String::new();
        //println!("{}", instant.elapsed().as_nanos());
        if are_you_moving
        {
            get_input(flip, numbers, keep_flip, &board, &all_turns, &turns, turn, &castle, passant, &mut input);
        }
        else if !ip.is_empty()
        {
            input = receive_data(ip.split_once(':').unwrap().1.parse::<u16>().unwrap()).unwrap();
        }
        else if bot
        {
            input = gen_move(board.clone());
        }
        if !ip.is_empty()
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
        if moves.is_empty()
        {
            println!("Invalid input");
            continue;
        }
        if moves[0] == 31 && moves[1] == 50 && moves[2] == 35 && moves[3] == 46
        {
            write_all_turns(&all_turns);
        }
        //ensure the input is in range
        if moves.len() != 4
           || moves[0] < 1
           || moves[0] > (board.len() + 2) as u8
           || moves[1] < 1
           || moves[1] > (board.len() + 2) as u8
           || moves[2] < 1
           || moves[2] > (board.len() + 2) as u8
           || moves[3] < 1
           || moves[3] > (board.len() + 2) as u8
        {
            println!("Invalid move");
            continue;
        }
        let x = moves[0] as usize - 1;
        let y = (moves[1] as i8 - board.len() as i8).unsigned_abs() as usize;
        let x2 = moves[2] as usize - 1;
        let y2 = (moves[3] as i8 - board.len() as i8).unsigned_abs() as usize;
        let piece = board[x][y];
        let piece2 = board[x2][y2];
        //dont move if the piece is the same color as the piece you are moving to
        if piece.is_uppercase() && piece2.is_uppercase() || piece.is_lowercase() && piece2.is_lowercase()
        {
            println!("Invalid move");
            continue;
        }
        //allow only white/black piece to move if its white's/black's turn
        if (turn % 2 == 0 && piece.is_uppercase()) || (turn % 2 == 1 && piece.is_lowercase())
        {
            println!("Invalid move");
            continue;
        }
        //pawn movement
        if piece.eq_ignore_ascii_case(&'p')
        {
            let possible_moves = pawn::pawn(board.clone(), x, y, Some(passant));
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves)
            {
                println!("Invalid move");
                continue;
            }
            pawn::promotion(&mut board, x2, y2, piece);
            //if pawn moved 2 spaces
            if y + 2 == y2 || (y > 1 && y - 2 == y2)
            {
                passant[0] = x2;
                passant[1] = y2;
                passant[2] = turn;
            }
            if piece2 == ' ' && x2 == passant[0] && y == passant[1] && turn - passant[2] == 1
            {
                board[passant[0]][passant[1]] = ' ';
            }
        }
        //if rook
        else if piece.eq_ignore_ascii_case(&'r')
        {
            let possible_moves = rook::rook(board.clone(), x, y);
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves)
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
            let possible_moves = bishop::bishop(board.clone(), x, y);
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves)
            {
                println!("Invalid move");
                continue;
            }
        }
        //if knight
        else if piece.eq_ignore_ascii_case(&'n')
        {
            let possible_moves = knight::knight(board.clone(), x, y);
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves)
            {
                println!("Invalid move");
                continue;
            }
        }
        //if queen
        else if piece.eq_ignore_ascii_case(&'q')
        {
            //just use rook and bishop logic together
            let mut possible_moves = bishop::bishop(board.clone(), x, y);
            possible_moves.extend(rook::rook(board.clone(), x, y));
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves)
            {
                println!("Invalid move");
                continue;
            }
        }
        //if king
        else if piece.eq_ignore_ascii_case(&'k')
        {
            let possible_moves = king::king(board.clone(), x, y, Some(castle.clone()));
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves)
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
        let is_check = check(&board, turn, false);
        if (turn % 2 == 0 && is_check == 2) || (turn % 2 == 1 && is_check == 1)
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
        print_board(board.clone(), &turns, flip, numbers, keep_flip, turn, None);
    }
}
fn get_input(flip:bool, numbers:bool, keep_flip:bool, board:&Vec<Vec<char>>, all_turns:&Vec<Vec<char>>, turns:&[Vec<char>], turn:usize, castle:&[bool], passant:[usize; 3], input:&mut String)
{
    loop
    {
        let move_char = read_single_char();
        print!("{}", move_char);
        std::io::stdout().flush().unwrap();
        if input.len() == 1 && move_char != '\x08'
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
                               .next()
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
                                    .next()
                                    .map(|val| val as i8 - board.len() as i8)
                                    .unwrap_or_default()).unsigned_abs() as usize;
            if input == "E" && move_char == 'X'
            {
                println!();
                write_all_turns(all_turns);
                std::process::exit(0);
            }
            if x >= board.len() || y >= board.len()
            {
                println!("Invalid move");
                *input = String::new();
                continue;
            }
            if turn % 2 == 1
            {
                match board[x][y]
                {
                    'P' => piece_moves = pawn::pawn(board.clone(), x, y, Some(passant)),
                    'R' => piece_moves = rook::rook(board.clone(), x, y),
                    'N' => piece_moves = knight::knight(board.clone(), x, y),
                    'B' => piece_moves = bishop::bishop(board.clone(), x, y),
                    'Q' =>
                    {
                        let mut bishop_moves:Vec<Vec<u8>> = bishop::bishop(board.clone(), x, y);
                        let mut rook_moves:Vec<Vec<u8>> = rook::rook(board.clone(), x, y);
                        rook_moves.remove(0);
                        bishop_moves.extend(rook_moves);
                        piece_moves = bishop_moves;
                    }
                    'K' => piece_moves = king::king(board.clone(), x, y, Some(castle.to_owned())),
                    _ =>
                    {
                        println!("Invalid move");
                        continue;
                    }
                }
            }
            else
            {
                match board[x][y]
                {
                    'p' => piece_moves = pawn::pawn(board.clone(), x, y, Some(passant)),
                    'r' => piece_moves = rook::rook(board.clone(), x, y),
                    'n' => piece_moves = knight::knight(board.clone(), x, y),
                    'b' => piece_moves = bishop::bishop(board.clone(), x, y),
                    'q' =>
                    {
                        let mut bishop_moves:Vec<Vec<u8>> = bishop::bishop(board.clone(), x, y);
                        let mut rook_moves:Vec<Vec<u8>> = rook::rook(board.clone(), x, y);
                        rook_moves.remove(0);
                        bishop_moves.extend(rook_moves);
                        piece_moves = bishop_moves;
                    }
                    'k' => piece_moves = king::king(board.clone(), x, y, Some(castle.to_owned())),
                    _ =>
                    {
                        println!("Invalid move");
                        continue;
                    }
                }
            }
            piece_moves.remove(0);
            print_board(board.clone(), turns, flip, numbers, keep_flip, turn, Some(piece_moves));
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
        if move_char == '\x08'
        {
            print!("\x08");
            std::io::stdout().flush().unwrap();
            input.pop();
        }
        else
        {
            *input += &move_char.to_string();
        }
        if move_char == '\n'
        {
            break;
        }
    }
}
fn can_move(board:&mut [Vec<char>], x:usize, y:usize, x2:usize, y2:usize, piece:char, possible_moves:Vec<Vec<u8>>) -> bool
{
    let mut success = false;
    for row in possible_moves.iter()
    {
        let mut iter = row.iter().peekable();
        while let Some(&value) = iter.next()
        {
            if value == x2 as u8 && iter.peek() == Some(&&(y2 as u8))
            {
                success = true;
                board[x2][y2] = piece;
                board[x][y] = ' ';
                break;
            }
        }
    }
    success
}
fn write_all_turns(all_turns:&Vec<Vec<char>>)
{
    for row in all_turns
    {
        if row.is_empty()
        {
            continue;
        }
        for val in row
        {
            print!("{}", val);
        }
        print!(" ");
    }
    println!();
    std::process::exit(0);
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) => c,
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        _ => panic!("Invalid key"),
    }
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
    if let Some(stream) = listener.incoming().next()
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