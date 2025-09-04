use std::io;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];
const BOARD_SIZE: usize = 3;

fn main() {
    let mut board: [[char; BOARD_SIZE]; BOARD_SIZE] = create_scoreboard();
    show_board(&board);
    play_move(&mut board);
}

fn create_scoreboard() -> Board {
    return [['-'; BOARD_SIZE]; BOARD_SIZE];
}

fn show_board(board: &Board) {
    for row in board {
        for cell in row {
            print!(" {}", cell);
        }
        println!();
    }
}

fn play_move(score_board: &mut Board) {
    let mut moves_played: Vec<Vec<usize>> = vec![]; // khele gye moves ko save krne k liye taki check kr ske ki koi repeat tho nhi ho rha !
    let mut player = 'X';

    loop {
        let moves: Vec<usize> = get_move(&player); // getting the moves form the user to paly !

        if !moves_played.contains(&&moves) {
            score_board[moves[0]][moves[1]] = player;
            moves_played.push(moves);
            show_board(score_board);

            if moves_played.len() > 4 {
                if check_winner(player, score_board) {
                    println!("{} is the winner.", player);
                    break;
                };
            }
            if player == 'X' {
                player = 'O';
            } else {
                player = 'X';
            }
        } else {
            println!("Index already coverd!");
        }

        if moves_played.len() == 9 {
            println!("It's a draw");
            break;
        }
    }
}

// This fn is used to get the user input of the move they want to play!

fn get_move(player: &char) -> Vec<usize> {
    loop {
        println!("Play your move player {}.", player);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid input!");

        let coordinates: Vec<usize> = input
            .split_whitespace() // is space k basis pr alg kr liye
            .flat_map(str::parse::<usize>) // ise uska type str se usize kr liya taki arr ko index de sake
            .collect(); // ise usko array ki from me kr liya

        if coordinates.len() == 2 && coordinates[0] < 3 && coordinates[1] < 3 { // kuch basic pr important checks krke coordinates return kr diye
            return coordinates;
        } else {
            println!("Please enter a valid input.(row col, [ex: 1 2])");
        }
    }
}

fn check_winner(player: char, score_board: &mut Board) -> bool {
    let wins: [[(i8, i8); 3]; 8] = [
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];

    wins.iter().any(|line| {
        line.iter()
            .all(|&(r, c)| score_board[r as usize][c as usize] == player)
    }) // isko smjh tho liya hai lekin ek aur baar ache se revise krna.
}