pub fn print_board(board:Vec<Vec<char>>, turns:&[Vec<char>], flip:bool, numbers:bool, keep_flip:bool, turn:usize, moves:Option<Vec<Vec<u8>>>)
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
                ind = x;
            }
            else
            {
                res = x as i8 + 1i8;
                ind = (x as i8 - (board.len() as i8 - 1)).unsigned_abs() as usize;
            }
        }
        else if keep_flip
        {
            res = x as i8 + 1;
            ind = (x as i8 - (board.len() as i8 - 1)).unsigned_abs() as usize;
        }
        else
        {
            res = (x as i8 - board.len() as i8).abs();
            ind = x;
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
        let mut fg_color:&str;
        let mut bg_color:&str;
        'inner: for y in 0..board.len()
        {
            if let Some(ref moves) = moves
            {
                for mov in moves
                {
                    let mut x2 = x;
                    if keep_flip
                    {
                        x2 = (x as i8 - (board.len() as i8 - 1)).unsigned_abs() as usize;
                    }
                    if board[y][ind].is_uppercase()
                    {
                        fg_color = "38;5;255";
                    }
                    else
                    {
                        fg_color = "38;5;232";
                    }
                    if (y + ((x + 1) % 2)) % 2 == 0
                    {
                        bg_color = "48;5;236";
                    }
                    else
                    {
                        bg_color = "48;5;252";
                    }
                    if mov[0] == y as u8 && mov[1] == x2 as u8
                    {
                        print!("\x1b[{}m\x1b[{}m {} \x1b[0m", bg_color, fg_color, board[y][ind]);
                        continue 'inner;
                    }
                }
            }
            if board[y][ind].is_uppercase()
            {
                fg_color = "38;5;255";
            }
            else
            {
                fg_color = "38;5;232";
            }
            if (y + ((x + 1) % 2)) % 2 == 0
            {
                bg_color = "48;5;239";
            }
            else
            {
                bg_color = "48;5;247";
            }
            print!("\x1b[{}m\x1b[{}m {} \x1b[0m", bg_color, fg_color, board[y][ind]);
        }
        println!(" {} {}{}{}{}", col, turns[x][0], turns[x][1], turns[x][2], turns[x][3]);
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