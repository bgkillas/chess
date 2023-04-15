pub fn print_board(board:Vec<Vec<char>>, turns:Vec<Vec<char>>, flip:bool, numbers:bool, keep_flip:bool, turn:usize, moves:Option<Vec<Vec<u8>>>)
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