fn main()
{
    // TODO: allow for normal chess notation
    // TODO: show the last 8 moves to the right of the board
    // TODO: en passant
    // TODO: check
    // TODO: checkmate
    // TODO: stalemate
    // TODO: dont allow castling after the rook moved
    // TODO: implement networking
    // TODO: implement stock fish
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("8 \x1b[47m\x1b[90m r \x1b[37m\x1b[100m n \x1b[47m\x1b[90m b \x1b[37m\x1b[100m q \x1b[47m\x1b[90m k \x1b[37m\x1b[100m b \x1b[47m\x1b[90m n \x1b[37m\x1b[100m r \x1b[0m");
    println!("7 \x1b[37m\x1b[100m p \x1b[47m\x1b[90m p \x1b[37m\x1b[100m p \x1b[47m\x1b[90m p \x1b[37m\x1b[100m p \x1b[47m\x1b[90m p \x1b[37m\x1b[100m p \x1b[47m\x1b[90m p \x1b[0m");
    println!("6 \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[0m");
    println!("5 \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[0m");
    println!("4 \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[0m");
    println!("3 \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[37m\x1b[100m   \x1b[47m\x1b[90m   \x1b[0m");
    println!("2 \x1b[47m\x1b[90m P \x1b[37m\x1b[100m P \x1b[47m\x1b[90m P \x1b[37m\x1b[100m P \x1b[47m\x1b[90m P \x1b[37m\x1b[100m P \x1b[47m\x1b[90m P \x1b[37m\x1b[100m P \x1b[0m");
    println!("1 \x1b[37m\x1b[100m R \x1b[47m\x1b[90m N \x1b[37m\x1b[100m B \x1b[47m\x1b[90m Q \x1b[37m\x1b[100m K \x1b[47m\x1b[90m B \x1b[37m\x1b[100m N \x1b[47m\x1b[90m R \x1b[0m");
    println!("   1  2  3  4  5  6  7  8");
    let mut board:Vec<Vec<char>> = vec![vec!['r', 'p', ' ', ' ', ' ', ' ', 'P', 'R'],
                                        vec!['n', 'p', ' ', ' ', ' ', ' ', 'P', 'N'],
                                        vec!['b', 'p', ' ', ' ', ' ', ' ', 'P', 'B'],
                                        vec!['q', 'p', ' ', ' ', ' ', ' ', 'P', 'Q'],
                                        vec!['k', 'p', ' ', ' ', ' ', ' ', 'P', 'K'],
                                        vec!['b', 'p', ' ', ' ', ' ', ' ', 'P', 'B'],
                                        vec!['n', 'p', ' ', ' ', ' ', ' ', 'P', 'N'],
                                        vec!['r', 'p', ' ', ' ', ' ', ' ', 'P', 'R']];
    let mut turn = 1;
    let mut black_castle = true;
    let mut white_castle = true;
    'outer: loop
    {
        println!();
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
        let moves:Vec<u8> = input.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap() as u8).collect();
        if moves.len() != 4 || moves[0] < 1 || moves[0] > 9 || moves[1] < 1 || moves[1] > 9 || moves[2] < 1 || moves[2] > 9 || moves[3] < 1 || moves[3] > 9
        {
            println!("Invalid move");
            continue;
        }
        let x = moves[0] as usize - 1;
        let y = (moves[1] as i8 - 8).abs() as usize;
        let x2 = moves[2] as usize - 1;
        let y2 = (moves[3] as i8 - 8).abs() as usize;
        let piece = board[x][y];
        let piece2 = board[x2][y2];
        if piece.is_uppercase() && piece2.is_uppercase() || piece.is_lowercase() && piece2.is_lowercase()
        {
            println!("Invalid move");
            continue;
        }
        if turn % 2 == 0 && piece.is_uppercase()
        {
            println!("Invalid move");
            continue;
        }
        else if turn % 2 == 1 && piece.is_lowercase()
        {
            println!("Invalid move");
            continue;
        }
        if piece == 'P' || piece == 'p'
        {
            if piece.is_uppercase()
            {
                if y == 6 && y2 == y - 2 && x2 == x
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else if y2 == y - 1 && x2 == x
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else if y2 == y - 1 && x2 == x + 1 && piece2.is_lowercase()
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else if y2 == y - 1 && x2 == x - 1 && piece2.is_lowercase()
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else
                {
                    println!("Invalid move");
                    continue;
                }
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
            else if piece.is_lowercase()
            {
                if y == 1 && y2 == y + 2 && x2 == x
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else if y2 == y + 1 && x2 == x
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else if y2 == y + 1 && x2 == x + 1 && piece2.is_uppercase()
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else if y2 == y + 1 && x2 == x - 1 && piece2.is_uppercase()
                {
                    board[x][y] = ' ';
                    board[x2][y2] = piece;
                }
                else
                {
                    println!("Invalid move");
                    continue;
                }
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
        else if piece == 'R' || piece == 'r'
        {
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
            if x2 == x && y2 != y
            {
                board[x][y] = ' ';
                board[x2][y2] = piece;
            }
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
        }
        else if piece == 'B' || piece == 'b'
        {
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
        else if piece == 'N' || piece == 'n'
        {
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
        else if piece == 'Q' || piece == 'q'
        {
            if x2 == x && y2 != y
            {
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
            else if x2 != x && y2 == y
            {
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
            else if (x2 as i8 - x as i8).abs() == (y2 as i8 - y as i8).abs()
            {
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
        else if piece == 'K' || piece == 'k'
        {
            if y2 == y && x == 4 && (x2 == 2 || x2 == 6)
            {
                if white_castle
                {
                    if piece == 'K' && (x2 == 6 && board[7][7] == 'R') || (x2 == 2 && board[0][7] == 'R')
                    {
                        board[x][y] = ' ';
                        board[x2][y2] = piece;
                        if x2 == 6
                        {
                            board[7][7] = ' ';
                            board[5][7] = 'R';
                        }
                        else if x2 == 2
                        {
                            board[0][7] = ' ';
                            board[3][7] = 'R';
                        }
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
                        board[x][y] = ' ';
                        board[x2][y2] = piece;
                        if x2 == 6
                        {
                            board[7][0] = ' ';
                            board[5][0] = 'r';
                        }
                        else if x2 == 2
                        {
                            board[0][0] = ' ';
                            board[3][0] = 'r';
                        }
                    }
                    else if ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 0 && (y2 as i8 - y as i8).abs() == 1) || ((x2 as i8 - x as i8).abs() == 1 && (y2 as i8 - y as i8).abs() == 0)
                    {
                        black_castle = false;
                    }
                }
            }
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
        turn += 1;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for i in 0..8
        {
            let res;
            let ind;
            if std::env::args().nth(1) == Some("flip".to_string())
            {
                if turn % 2 == 1
                {
                    res = (i - 8 as i8).abs();
                    ind = i as usize;
                }
                else
                {
                    res = i + 1;
                    ind = (i - 7 as i8).abs() as usize;
                }
            }
            else
            {
                res = (i - 8 as i8).abs();
                ind = i as usize;
            }
            if (i + 1) % 2 == 0
            {
                println!("{} \x1b[37m\x1b[100m {} \x1b[47m\x1b[90m {} \x1b[37m\x1b[100m {} \x1b[47m\x1b[90m {} \x1b[37m\x1b[100m {} \x1b[47m\x1b[90m {} \x1b[37m\x1b[100m {} \x1b[47m\x1b[90m {} \x1b[0m",
                         res, board[0][ind], board[1][ind], board[2][ind], board[3][ind], board[4][ind], board[5][ind], board[6][ind], board[7][ind]);
            }
            else
            {
                println!("{} \x1b[47m\x1b[90m {} \x1b[37m\x1b[100m {} \x1b[47m\x1b[90m {} \x1b[37m\x1b[100m {} \x1b[47m\x1b[90m {} \x1b[37m\x1b[100m {} \x1b[47m\x1b[90m {} \x1b[37m\x1b[100m {} \x1b[0m",
                         res, board[0][ind], board[1][ind], board[2][ind], board[3][ind], board[4][ind], board[5][ind], board[6][ind], board[7][ind]);
            }
        }
        println!("   1  2  3  4  5  6  7  8");
    }
}