use crossterm::{
    cursor, event,
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseButton, MouseEvent, MouseEventKind,
    },
    terminal, ExecutableCommand,
};
use std::{
    env::args,
    fs::File,
    io::{stdout, BufRead, BufReader, Read, Result, Write},
    net::{TcpListener, TcpStream},
    process::exit,
    time::Instant,
};
mod pieces {
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
use bot::gen_move;
use check::check;
use pieces::{bishop, king, knight, pawn, rook};
use print_board::print_board;
fn main() {
    // 0=bot
    // 1=flip
    // 2=numbers
    // 3=debug
    // 4=color
    // 5=keep_flip
    // 6=double
    let mut arg = [false; 8];
    arg[0] = true;
    let mut file = String::new();
    let mut ip = String::new();
    for i in 0..args().len() {
        match args().nth(i).unwrap().as_str() {
            "--help" => {
                println!("Usage: chess [OPTION]...");
                println!("to move a piece type the coordinates of the piece you want to move and the coordinates of where you want to move it");
                println!("for example: e2e4 or 5254");
                println!("--flip will flip the board each move");
                println!("--keep_flip will have black on the bottom and white on the top");
                println!("--numbers will show on the bottom instead of letters");
                println!("--file CSV will load a board from a csv file");
                println!("--black will make you play as black");
                println!("--ip IP will connect to a server at IP:port");
                println!("--no_bot will disable playing against a bot");
                println!("--debug will show time to calculate moves");
                exit(0);
            }
            "--flip" => arg[1] = true,
            "--keep_flip" => arg[5] = !arg[5],
            "--numbers" => arg[2] = true,
            "--file" => {
                arg[0] = !arg[0];
                file = args().nth(i + 1).unwrap()
            }
            "--black" => {
                arg[4] = true;
                arg[5] = !arg[5];
            }
            "--ip" => {
                arg[0] = false;
                ip = args().nth(i + 1).unwrap();
            }
            "--no_bot" => arg[0] = false,
            "--debug" => arg[3] = true,
            "--double" => arg[6] = true,
            "--no_extra_output" => arg[7] = true,
            _ => {}
        }
    }
    // disable line blinking
    let mut board: Vec<Vec<char>>;
    if !file.is_empty() && File::open(&file).is_ok() {
        let csv = File::open(file).unwrap();
        let reader = BufReader::new(csv);
        board = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .split(',')
                    .map(|c| c.chars().next().unwrap())
                    .collect()
            })
            .collect();
    } else {
        board = vec![
            vec!['r', 'p', ' ', ' ', ' ', ' ', 'P', 'R'],
            vec!['n', 'p', ' ', ' ', ' ', ' ', 'P', 'N'],
            vec!['b', 'p', ' ', ' ', ' ', ' ', 'P', 'B'],
            vec!['q', 'p', ' ', ' ', ' ', ' ', 'P', 'Q'],
            vec!['k', 'p', ' ', ' ', ' ', ' ', 'P', 'K'],
            vec!['b', 'p', ' ', ' ', ' ', ' ', 'P', 'B'],
            vec!['n', 'p', ' ', ' ', ' ', ' ', 'P', 'N'],
            vec!['r', 'p', ' ', ' ', ' ', ' ', 'P', 'R'],
        ];
    }
    // ensure the board is a square
    if board[0].len() != board.len() {
        println!("Board must be a square");
        exit(1);
    }
    // turn tracker
    let mut all_turns: Vec<Vec<char>> = vec![vec![]];
    let mut turns: Vec<Vec<char>> = vec![vec!['0'; 4]; board.len()];
    let mut turn = 1;
    if arg[4] && ip.is_empty() && !arg[0] {
        turn = 2;
    }
    // print_board(board.clone(), &turns, flip, numbers, keep_flip, turn, None);
    // castling stuff castle[0]= white left castle, castle[1] = white right castle, castle[2] = black left castle, castle[3] = black right castle, castle[4] = white castle, castle[5] = black castle
    let mut castle: Vec<bool> = vec![true; 6];
    let mut copy: Vec<Vec<char>>;
    // en passant stuff
    let mut passant = [0; 3];
    let mut instant: Option<Instant> = if arg[3] { Some(Instant::now()) } else { None };
    stdout().execute(terminal::EnterAlternateScreen).unwrap();
    stdout().execute(cursor::Hide).unwrap();
    stdout().execute(EnableMouseCapture).unwrap();
    print!("\x1b[H\x1b[J");
    stdout().flush().unwrap();
    loop {
        // dont allow en passant on a piece after a turn
        if turn != passant[2] + 1 {
            passant[0] = 0;
            passant[1] = 0;
            passant[2] = 0;
        }
        copy = board.clone();
        let mut are_you_moving = false;
        if (ip.is_empty() && !arg[0]) || (turn % 2 == 0 && arg[4]) || (turn % 2 == 1 && !arg[4]) {
            are_you_moving = true;
        }
        if arg[6] {
            are_you_moving = false;
        }
        let mut input = String::new();
        if are_you_moving {
            input = get_input(
                &board,
                &all_turns,
                &castle,
                passant,
                instant.map(|d| d.elapsed().as_nanos()),
                arg,
            );
            if arg[3] {
                instant = Some(Instant::now())
            }
        } else if !ip.is_empty() {
            input = receive_data(ip.split_once(':').unwrap().1.parse::<u16>().unwrap()).unwrap();
        } else if arg[0] {
            input = gen_move(&board, &castle, passant, &all_turns);
        }
        if !ip.is_empty() {
            match send_data(input.clone(), &ip) {
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
        let moves: Vec<u8> = convert_to_num(input.clone());
        if moves.is_empty() {
            if !arg[7] {
                println!("Invalid input");
            }
            continue;
        }
        if moves[0] == 31 && moves[1] == 50 && moves[2] == 35 && moves[3] == 46 {
            write_all_turns(&all_turns, arg[0]);
        }
        // ensure the input is in range
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
            if !arg[7] {
                println!("Invalid move");
            }
            continue;
        }
        let mut x = moves[0] as usize - 1;
        let y = (moves[1] as i8 - board.len() as i8).unsigned_abs() as usize;
        let mut x2 = moves[2] as usize - 1;
        let y2 = (moves[3] as i8 - board.len() as i8).unsigned_abs() as usize;
        let is_flipped = !(arg[1] && turn != 1 && turn % 2 == 0);
        if !is_flipped {
            x = board.len() - x - 1;
            x2 = board.len() - x2 - 1;
        }
        let piece = board[x][y];
        let piece2 = board[x2][y2];
        // dont move if the piece is the same color as the piece you are moving to
        if piece.is_uppercase() && piece2.is_uppercase()
            || piece.is_lowercase() && piece2.is_lowercase()
        {
            if !arg[7] {
                println!("Invalid move");
            }
            continue;
        }
        // allow only white/black piece to move if its white's/black's turn
        if (turn % 2 == 0 && piece.is_uppercase()) || (turn % 2 == 1 && piece.is_lowercase()) {
            if !arg[7] {
                println!("Invalid move");
            }
            continue;
        }
        // pawn movement
        if piece.eq_ignore_ascii_case(&'p') {
            let possible_moves = pawn::pawn(&board, x, y, Some(passant));
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves) {
                if !arg[7] {
                    println!("Invalid move");
                }
                continue;
            }
            pawn::promotion(&mut board, x2, y2, piece, arg[0]);
            // if pawn moved 2 spaces
            if y + 2 == y2 || (y > 1 && y - 2 == y2) {
                passant[0] = x2;
                passant[1] = y2;
                passant[2] = turn;
            }
            if piece2 == ' ' && x2 == passant[0] && y == passant[1] && turn - passant[2] == 1 {
                board[passant[0]][passant[1]] = ' ';
            }
        }
        // if rook
        else if piece.eq_ignore_ascii_case(&'r') {
            let possible_moves = rook::rook(&board, x, y);
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves) {
                if !arg[7] {
                    println!("Invalid move");
                }
                continue;
            }
            if x == 0 && piece == 'R' {
                castle[0] = false; // disable white left castle
            } else if x == 7 && piece == 'R' {
                castle[1] = false; // disable white right castle
            } else if x == 0 && piece == 'r' {
                castle[2] = false; // disable black left castle
            } else if x == 7 && piece == 'r' {
                castle[3] = false; // disable black right castle
            }
        }
        // if bishop
        else if piece.eq_ignore_ascii_case(&'b') {
            let possible_moves = bishop::bishop(&board, x, y);
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves) {
                if !arg[7] {
                    println!("Invalid move");
                }
                continue;
            }
        }
        // if knight
        else if piece.eq_ignore_ascii_case(&'n') {
            let possible_moves = knight::knight(&board, x, y);
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves) {
                if !arg[7] {
                    println!("Invalid move");
                }
                continue;
            }
        }
        // if queen
        else if piece.eq_ignore_ascii_case(&'q') {
            // just use rook and bishop logic together
            let mut possible_moves = bishop::bishop(&board, x, y);
            possible_moves.extend(rook::rook(&board, x, y));
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves) {
                if !arg[7] {
                    println!("Invalid move");
                }
                continue;
            }
        }
        // if king
        else if piece.eq_ignore_ascii_case(&'k') {
            let possible_moves = king::king(&board, x, y, Some(castle.as_ref()));
            if !can_move(&mut board, x, y, x2, y2, piece, possible_moves) {
                if !arg[7] {
                    println!("Invalid move");
                }
                continue;
            }
            if (x2 as i8 - x as i8).abs() == 2 {
                let piece3 = match piece {
                    'K' => 'R',
                    'k' => 'r',
                    _ => ' ',
                };
                if x2 == 2 {
                    board[0][y2] = ' ';
                    board[3][y2] = piece3;
                } else if x2 == 6 {
                    board[7][y2] = ' ';
                    board[5][y2] = piece3;
                }
            }
        } else {
            if !arg[7] {
                println!("Invalid move");
            }
            continue;
        }
        // ensure that the player is not in check after move
        let is_check = check(&board, turn, false, if turn % 2 == 1 { 'K' } else { 'k' });
        if (turn % 2 == 0 && is_check == 2) || (turn % 2 == 1 && is_check == 1) {
            println!("cant move in check");
            board = copy.clone();
            continue;
        }
        let turn_str: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
        // delete the first turn of the turn tracker if there are too many to display
        if turn > board.len() {
            turns.remove(0);
            turns.push(turn_str.clone());
        } else {
            turns[turn - 1] = turn_str.clone();
        }
        all_turns.push(turn_str);
        turn += 1;
        if !(arg[0] && turn % 2 == 1) {
            print_board(&board, &all_turns, None, arg);
        }
    }
}
fn convert_to_num(input: String) -> Vec<u8> {
    input
        .chars()
        .filter_map(|c| match c {
            'a'..='t' => Some(c as u8 - b'a' + 1),
            'A'..='Z' => Some(c as u8 - b'A' + 27),
            '0'..='9' => c.to_digit(10).map(|d| d as u8),
            _ => None,
        })
        .collect()
}
fn get_input(
    board: &[Vec<char>],
    all_turns: &[Vec<char>],
    castle: &[bool],
    passant: [usize; 3],
    instant: Option<u128>,
    arg: [bool; 8],
) -> String {
    let turn = if all_turns.len() % 2 == 1 { 1 } else { 2 };
    let mut input = String::new();
    print_board(board, all_turns, None, arg);
    if let Some(instant) = instant {
        println!("{}", instant);
    }
    let mut piece_moves: Vec<Vec<u8>>;
    let mut str = String::new();
    let is_flipped = !(arg[1] && turn != 1 && turn % 2 == 0);
    loop {
        let move_char = if str.is_empty() {
            read_input()
        } else {
            (str.remove(0), None)
        };
        if let Some(n) = move_char.1 {
            let x = (n.0.saturating_sub(2)) / 3;
            if (x as usize) < board.len() && (n.1 as usize) < board.len() {
                str = format!(
                    "{}{}",
                    (x as u8 + b'a') as char,
                    if turn % 2 == 0 && arg[1] {
                        n.1 as usize + 1
                    } else {
                        board.len() - n.1 as usize
                    }
                );
                if input.len() == 2 {
                    str += "\n"
                }
            }
        } else if move_char.0 == '\x14' {
            write_all_turns(all_turns, true);
            stdout().execute(terminal::LeaveAlternateScreen).unwrap();
            exit(0);
        } else if input.len() == 1 && move_char.0 != '\x08' {
            let mut x: usize = convert_to_num(input.clone())
                .first()
                .map(|val| *val as usize - 1)
                .unwrap_or_default();
            if !is_flipped {
                x = board.len() - x - 1;
            }
            let y: usize = convert_to_num(move_char.0.to_string())
                .first()
                .map(|val| *val as i8 - board.len() as i8)
                .unwrap_or_default()
                .unsigned_abs() as usize;
            if input == "E" && move_char.0 == 'X' {
                println!();
                write_all_turns(all_turns, arg[0]);
            }
            if x >= board.len() || y >= board.len() {
                if !arg[7] {
                    println!("\nInvalid move");
                }
                input = String::new();
                continue;
            }
            if turn % 2 == 1 && board[x][y].is_lowercase()
                || turn % 2 == 0 && board[x][y].is_uppercase()
            {
                if !arg[7] {
                    println!("\nInvalid move");
                }
                input = String::new();
                continue;
            }
            match board[x][y].to_ascii_uppercase() {
                'P' => piece_moves = pawn::pawn(board, x, y, Some(passant)),
                'R' => piece_moves = rook::rook(board, x, y),
                'N' => piece_moves = knight::knight(board, x, y),
                'B' => piece_moves = bishop::bishop(board, x, y),
                'Q' => {
                    let mut bishop_moves: Vec<Vec<u8>> = bishop::bishop(board, x, y);
                    let mut rook_moves: Vec<Vec<u8>> = rook::rook(board, x, y);
                    rook_moves.remove(0);
                    bishop_moves.extend(rook_moves);
                    piece_moves = bishop_moves;
                }
                'K' => piece_moves = king::king(board, x, y, Some(castle)),
                _ => {
                    input = String::new();
                    if !arg[7] {
                        println!("\nNot a valid piece\x1b[A\x1b[A  ");
                    }
                    continue;
                }
            }
            print_board(board, all_turns, Some(piece_moves), arg);
            if let Some(instant) = instant {
                println!("{}", instant);
            }
            input += &move_char.0.to_string();
            if !arg[7] {
                print!("\x1b[G\x1b[K{}", input);
            }
            stdout().flush().unwrap();
        } else if move_char.0 == '\x08' {
            input.pop();
            if input.len() == 1 {
                print_board(board, all_turns, None, arg);
            }
            if let Some(instant) = instant {
                println!("{}", instant);
            }
            if !arg[7] {
                print!("\x1b[G\x1b[K{}", input);
            }
            stdout().flush().unwrap();
        } else if move_char.0 == '\n' || move_char.0 == '_' {
            break;
        } else if move_char.0 != '\0' {
            input += &move_char.0.to_string();
            if !arg[7] {
                print!("\x1b[G\x1b[K{}", input);
            }
            stdout().flush().unwrap();
        }
    }
    input
}
fn can_move(
    board: &mut [Vec<char>],
    x: usize,
    y: usize,
    x2: usize,
    y2: usize,
    piece: char,
    possible_moves: Vec<Vec<u8>>,
) -> bool {
    let mut success = false;
    for row in possible_moves {
        let mut iter = row.iter().peekable();
        while let Some(&value) = iter.next() {
            if value == x2 as u8 && iter.peek() == Some(&&(y2 as u8)) {
                success = true;
                board[x2][y2] = piece;
                board[x][y] = ' ';
                break;
            }
        }
    }
    success
}
fn write_all_turns(all_turns: &[Vec<char>], bot: bool) {
    terminal::disable_raw_mode().unwrap();
    stdout().execute(DisableMouseCapture).unwrap();
    stdout().execute(cursor::Show).unwrap();
    print!("\x1b[G\x1b[K");
    for (x, row) in all_turns.iter().enumerate().take(all_turns.len()) {
        if bot && x % 2 == 0 {
            continue;
        }
        if x > 1 {
            print!("_");
        }
        for val in row.iter() {
            print!("{}", val);
        }
    }
    println!();
    stdout().flush().unwrap();
}
pub fn read_input() -> (char, Option<(u16, u16)>) {
    terminal::enable_raw_mode().unwrap();
    stdout().execute(cursor::Hide).unwrap();
    loop {
        let read = event::read().unwrap();
        match read {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    terminal::disable_raw_mode().unwrap();
                    return ('\x14', None);
                }
                (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    terminal::disable_raw_mode().unwrap();
                    return (c, None);
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    terminal::disable_raw_mode().unwrap();
                    return ('\n', None);
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    terminal::disable_raw_mode().unwrap();
                    return ('\x08', None);
                }
                _ => {}
            },
            Event::Mouse(MouseEvent {
                kind, column, row, ..
            }) => {
                if kind == MouseEventKind::Down(MouseButton::Left) {
                    terminal::disable_raw_mode().unwrap();
                    return ('\0', Some((column, row)));
                }
            }
            _ => {}
        }
    }
}
fn send_data(moves: String, addr: &str) -> Result<String> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(moves.as_bytes())?;
    let mut buf = [0; 3];
    stream.read_exact(&mut buf)?;
    let message = String::from_utf8_lossy(&buf).to_string();
    Ok(message)
}
fn receive_data(port: u16) -> Result<String> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    if let Some(stream) = listener.incoming().next() {
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
