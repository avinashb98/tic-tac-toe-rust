extern crate rprompt;
type Board = [[char; 3]; 3];
const PLAYERS:[char;2] = ['x', 'o'];

fn main() {
    let mut board: Board = get_empty_board();
    let mut current_player = 'x';
    let mut row:usize;
    let mut col:usize;
    loop {
        cls();
        print_board(board);
        println!("Current Player: {}", current_player);
        let mut input_row: String;
        loop {
            input_row = rprompt::prompt_reply_stdout("Enter Row: ").unwrap();
            if !is_valid_input(&input_row) {
                println!("Invalid Row. Acceptable Values: 0, 1, 2");
                continue;
            }
            row = input_row.to_string().parse().unwrap();
            break;
        }
        let mut input_col: String;
        loop {
            input_col = rprompt::prompt_reply_stdout("Enter Column: ").unwrap();
            if !is_valid_input(&input_col) {
                println!("Invalid Col. Acceptable Values: 0, 1, 2");
                continue;
            }
            col = input_col.parse().unwrap();
            break;
        }

        if !is_valid_entry(row, col, board) {
            println!("Spot already filled. Enter a different spot.");
            continue;
        }

        if current_player == 'x' {
            board[row][col] = 'x';
            current_player = 'o';
        } else {
            board[row][col] = 'o';
            current_player = 'x';
        }

        let winner = get_winner(board);

        if winner == 'x' || winner == 'o' {
            cls();
            print_board(board);
            println!("{} Won", winner);
            break;
        }
    }
}

fn get_empty_board() -> Board {
    let board:Board = [
        [' ', ' ', ' '],
        [' ', ' ', ' '],
        [' ', ' ', ' ']
    ];
    return board;
}

fn print_board(board:Board) {

    println!(" {} | {} | {} ", board[0][0], board[0][1], board[0][2]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[1][0], board[1][1], board[1][2]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[2][0], board[2][1], board[2][2]);
}

fn is_valid_input(input: &String) -> bool {
    let valid_inputs = ["0", "1", "2"];
    for i in valid_inputs {
        if input == i {
            return true;
        }
    }
    return false;
}

fn is_valid_entry(row:usize, col:usize, board:Board) -> bool {
    return board[row][col] == ' ';
}

fn get_winner(board:Board) -> char {
    // horizontals and verticals
    for i in 0..3 {
        for ch in PLAYERS {
            if board[0][i] == ch && board[1][i] == ch && board[2][i] == ch {
                return ch;
            }
            if board[i][0] == ch && board[i][1] == ch && board[i][2] == ch {
                return ch;
            }
        }
    }

    // diagonals
    for ch in PLAYERS {
        if board[0][0] == ch && board[1][1] == ch && board[2][2] == ch {
            return ch;
        }
        if board[0][2] == ch && board[1][1] == ch && board[2][0] == ch {
            return ch;
        }
    }
    return 'n';
}

fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}