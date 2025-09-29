use std::io;
use std::io::Write;

fn main() {
    let mut board = [['.'; 8]; 8];
    board[3][3] = 'W';
    board[3][4] = 'B';
    board[4][3] = 'B';
    board[4][4] = 'W';

    print_board(&board);

    let mut player = 'B';

    loop{
        if has_valid(&mut board, player){
            check_and_place(&mut board, player);

            print_board(&board);

            player = opponent(player);
        } else if has_valid(&mut board, opponent(player)){
            println!("{} player has no valid move.", player);
            player = opponent(player);
        } else {
            println!("{} player has no valid move.", player);
            println!("{} player has no valid move.", opponent(player));
            // both player has no valid position, to terminate stage
            termination(&board);
            break;
        }
        
    }
}

fn print_board(board:&[[char; 8]; 8]){
    print!("  abcdefgh");
    println!();
    let alpha = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    for i in 0..8 {
        print!("{} ", alpha[i]);
        for j in 0..8 {
            print!("{}", board[i][j]);
        }
        println!("");
    }
}

fn check_and_place(board: &mut [[char; 8]; 8], player: char){
    loop{
        print!("Enter move for colour {} (RowCol): ", player);
        io::stdout().flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // remove \r \n or something
        let input = input.trim();

        // string to byte
        let bytes = input.as_bytes();
        let row_byte = bytes[0];
        let col_byte = bytes[1];

        // check range
        if !(b'a'..=b'h').contains(&row_byte) || !(b'a'..=b'h').contains(&col_byte) {
            println!("Invalid move. Try again.");
            print_board(&board);
            continue;
        }

        // where 97 is byte for 'a', so minus 97 transfer byte to row and col number
        let row = row_byte-97 ;
        let col = col_byte-97;
        //println!("Move is {} {}", row, col);

        // valid check
        if is_valid(board, player, row, col){
            reversi(board, player, row, col);
            break;
        } else {
            println!("Invalid move. Try again.");
            print_board(&board);
            continue;
        }
    }
    
}

fn is_valid(board: &mut [[char; 8]; 8], player: char, row: u8, col: u8) -> bool{
    // if this position is not blank then not valid
    if board[row as usize][col as usize] != '.' {
        return false;
    }

    let eight_direct = [(1,0),(0,1),(-1,0),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)];

    let opponent = opponent(player);

    for (x_shift,y_shift) in eight_direct.iter(){
        let mut check_x = row as i32 + x_shift;
        let mut check_y = col as i32 + y_shift;

        let mut check_opponent = false;

        while check_x >= 0 && check_x <= 7 && check_y >= 0 && check_y <= 7{
            if board[check_x as usize][check_y as usize] == opponent{
                // if there's opponent colour near
                check_opponent = true;
            } else if board[check_x as usize][check_y as usize] == player{
                // when find self
                if check_opponent == true{
                    // true if already found opponent, which means a valid position
                    return true;
                } else {
                    // beark if has'nt found opponent, which means this position is near by a self colour, in this direction
                    break;
                }
            } else {
                // break if nothing but blank was found in this direction
                break;
            }

            check_x += x_shift;
            check_y += y_shift;
        }
    }

    // go through all eight direction, if still nothing, then this is not a valid position
    return false;
}

fn opponent(player:char) -> char{
    if player == 'B'{
        return 'W';
    } else {
        return 'B';
    }
}

// loop all position to check if any available
fn has_valid(board: &mut [[char; 8]; 8], player: char) -> bool{
    for i in 0..8{
        for j in 0..8{
            if is_valid(board,player,i,j){
                return true;
            }
        }
    }
    return false;
}

// could combine with valid check function
fn reversi(board: &mut [[char; 8]; 8],player:char,row:u8,col:u8){
    let eight_direct = [(1,0),(0,1),(-1,0),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)];

    let opponent = opponent(player);

    board[row as usize][col as usize] = player;

    for (x_shift,y_shift) in eight_direct.iter(){
        let mut check_x = row as i32 + x_shift;
        let mut check_y = col as i32 + y_shift;

        let mut temp = Vec::new();

        while check_x >= 0 && check_x <= 7 && check_y >= 0 && check_y <= 7{
            if board[check_x as usize][check_y as usize] == opponent{
                // temp store
                temp.push((check_x as usize, check_y as usize));
            } else if board[check_x as usize][check_y as usize] == player{
                for (temp_x, temp_y) in temp{
                    // reversi opponent
                    board[temp_x][temp_y] = player;
                }
                break;
            } else {
                break;
            }
            check_x += x_shift;
            check_y += y_shift;
        }
    }
}

fn termination(board: &[[char; 8]; 8]){
    let mut b_num = 0;
    let mut w_num = 0;

    for i in board {
        for &j in i{
            if j == 'B' {
                b_num += 1;
            } else if j == 'W' {
                w_num += 1;
            }
        }
    }

    if b_num > w_num{
        println!("Black wins by {} points!", b_num-w_num);
    } else if w_num > b_num {
        println!("White wins by {} points!", w_num-b_num);
    } else {
        println!("Draw!");
    }
}