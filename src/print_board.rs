use crate::{check::check, write_all_turns};
pub fn print_board(
    board: Vec<Vec<char>>,
    turns: &[Vec<char>],
    all_turns: &Vec<Vec<char>>,
    moves: Option<Vec<Vec<u8>>>,
    arg: [bool; 8],
)
{
    let turn = if all_turns.len() % 2 == 1 { 1 } else { 0 };
    let mut last_move = vec![];
    if !all_turns.is_empty()
    {
        last_move = String::from_iter(&all_turns[all_turns.len() - 1])
            .chars()
            .filter_map(|c| match c
            {
                'a'..='t' => Some(c as u8 - b'a'),
                'A'..='Z' => Some(c as u8 - b'A' + 26),
                '0'..='9' => c.to_digit(10).map(|d| d as u8),
                _ => None,
            })
            .collect();
    }
    let mut mov: Vec<Vec<u8>> = vec![];
    if let Some(moves) = moves
    {
        mov = moves.clone();
        for i in 1..moves.len()
        {
            let mut boa = board.clone();
            boa[moves[i][0] as usize][moves[i][1] as usize] =
                boa[moves[0][0] as usize][moves[0][1] as usize];
            boa[moves[0][0] as usize][moves[0][1] as usize] = ' ';
            let num = check(&boa, turn, false, if turn % 2 == 1 { 'K' } else { 'k' });
            if ((num == 1) && (turn % 2 == 1)) || ((num == 2) && (turn % 2 == 0))
            {
                mov[i] = vec![];
            }
        }
        mov.remove(0);
    }
    let mut output = String::new();
    for x in 0..board.len()
    {
        let res;
        let ind;
        if arg[1]
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
        else if arg[5]
        {
            res = x as i8 + 1;
            ind = (x as i8 - (board.len() as i8 - 1)).unsigned_abs() as usize;
        }
        else
        {
            res = (x as i8 - board.len() as i8).abs();
            ind = x;
        }
        if board.len() > 8
        {
            output += &format!("{} ", (res as u8 + 96) as char);
        }
        else
        {
            output += &format!("{} ", res);
        }
        let mut fg_color: &str;
        let mut bg_color: &str;
        let mut y: i8 = 0;
        let mut ende: i8 = board.len() as i8;
        let mut dir: i8 = 1;
        if arg[5]
        {
            dir = -1;
            y = board.len() as i8 - 1;
            ende = -1;
        }
        'inner: loop
        {
            if y == ende
            {
                break;
            }
            if board[y as usize][ind].is_uppercase()
            {
                fg_color = "\x1b[38;2;0;0;139m";
            }
            else
            {
                fg_color = "\x1b[38;2;0;0;0m";
            }
            if !mov.is_empty()
            {
                for mo in &mov
                {
                    if mo.is_empty()
                    {
                        continue;
                    }
                    let mut x2 = x;
                    if arg[5]
                    {
                        x2 = (x as i8 - (board.len() as i8 - 1)).unsigned_abs() as usize;
                        if (y as usize + ((x + 1) % 2)) % 2 == 0
                        {
                            bg_color = "\x1b[48;2;255;250;225m";
                        }
                        else
                        {
                            bg_color = "\x1b[48;2;110;80;50m";
                        }
                    }
                    else if (y as usize + ((x + 1) % 2)) % 2 == 0
                    {
                        bg_color = "\x1b[48;2;110;80;50m";
                    }
                    else
                    {
                        bg_color = "\x1b[48;2;255;250;225m";
                    }
                    if mo[0] == y as u8
                        && mo[1]
                            == if arg[1] && turn % 2 == 0
                            {
                                board.len() - x2 - 1
                            }
                            else
                            {
                                x2
                            } as u8
                    {
                        output += &format!(
                            "{}{} {} \x1b[0m",
                            bg_color, fg_color, board[y as usize][ind]
                        );
                        y += dir;
                        continue 'inner;
                    }
                }
            }
            if arg[5]
            {
                if (y as usize + ((x + 1) % 2)) % 2 == 0
                {
                    bg_color = "\x1b[48;2;240;217;181m";
                }
                else
                {
                    bg_color = "\x1b[48;2;181;136;99m";
                }
            }
            else if (y as usize + ((x + 1) % 2)) % 2 == 0
            {
                bg_color = "\x1b[48;2;181;136;99m";
            }
            else
            {
                bg_color = "\x1b[48;2;240;217;181m";
            }
            if (!arg[0] || turn % 2 == if arg[4] { 0 } else { 1 })
                && !last_move.is_empty()
                && y as usize == last_move[2] as usize
                && if arg[5] || (arg[1] && turn % 2 == 0)
                {
                    x + 1
                }
                else
                {
                    ((x as i8 - (board.len() as i8 - 1)).unsigned_abs() + 1) as usize
                } == last_move[3] as usize
            {
                bg_color = "\x1b[48;2;247;247;105m";
            }
            output += &format!(
                "{}{} {} \x1b[0m",
                bg_color, fg_color, board[y as usize][ind]
            );
            y += dir;
        }
        output += "\n"
    }
    if arg[2]
    {
        output += " ";
        for j in 0..board.len()
        {
            output += &format!(
                "  {}",
                if arg[5]
                {
                    (j as i8 - board.len() as i8 + 1).unsigned_abs()
                }
                else
                {
                    j as u8
                } + 1
            );
        }
    }
    else
    {
        output += " ";
        for j in 0..board.len()
        {
            output += &format!(
                "  {}",
                (if arg[5]
                {
                    (j as i8 - board.len() as i8 + 1).unsigned_abs()
                }
                else
                {
                    j as u8
                } + 97) as char
            );
        }
    }
    let mut is_check = 0;
    if turn > 2
    {
        is_check = check(&board, turn, true, if turn % 2 == 1 { 'K' } else { 'k' });
    }
    if turn > 2
    {
        match is_check
        {
            1 => output += "\nWhite is in check",
            2 => output += "\nBlack is in check",
            3 =>
            {
                print_board(board, turns, all_turns, None, arg);
                println!(
                    "Checkmate. {} wins",
                    if turn % 2 == 0 { "White" } else { "Black" }
                );
                write_all_turns(all_turns, false);
            }
            4 =>
            {
                print_board(board, turns, all_turns, None, arg);
                println!("Stalemate");
                write_all_turns(all_turns, false);
            }
            _ =>
            {
                if !arg[7]
                {
                    output += if !arg[5]
                    {
                        if turn % 2 == 1 || arg[0]
                        {
                            "\nWhite's turn"
                        }
                        else
                        {
                            "\nBlack's turn"
                        }
                    }
                    else if turn % 2 == 1 || arg[0]
                    {
                        "\nBlack's turn"
                    }
                    else
                    {
                        "\nWhite's turn"
                    };
                }
            }
        }
    }
    else
    {
        if !arg[7]
        {
            output += if !arg[5]
            {
                if turn % 2 == 1 || arg[0]
                {
                    "\nWhite's turn"
                }
                else
                {
                    "\nBlack's turn"
                }
            }
            else if turn % 2 == 1 || arg[0]
            {
                "\nBlack's turn"
            }
            else
            {
                "\nWhite's turn"
            };
        }
    }
    // clear line and move cursor to top left and print board
    println!("\x1b[J\x1b[H{output}");
}