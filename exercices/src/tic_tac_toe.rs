mod errors;
use std::fmt::{self, Display, Formatter};
use std::io;

struct Board {
    matrix: [[Option<char>; 3]; 3],
    score: (u32, u32),
    round: (u32),
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        println!("\nScore:\n\tX: {}\n\tO: {}", self.score.0, self.score.1);
        println!(
            "Game: {}, round: {}",
            self.score.0 + self.score.1 + 1,
            self.round
        );
        // TODO: Iterate over matrix instead of using range for scalability
        for i in 0..3 {
            println!("-------------");
            for j in 0..3 {
                print!("|");
                let square = self.matrix[i][j];
                match square {
                    None => print!("   "),
                    Some(val) => print!(" {} ", val),
                }
            }
            print!("|\n");
        }
        println!("-------------");
        write!(f, "")
    }
}

impl Board {
    fn new() -> Board {
        println!("\n\n\n\t\t\tTIC TAC TOE YO");
        println!("To play, specify the square using row,col");
        println!("Example: to play in the top right square, enter '1,3'");
        Board {
            matrix: [[None; 3]; 3],
            score: (0, 0),
            round: 1,
        }
    }

    fn play(&mut self, input: &mut String, player: &mut char) -> Result<(i8, i8), errors::MyErr> {
        // Extract to proper coordinates
        // Validate nr of inputs
        let mut v: Vec<String> = input.split(',').map(|s| String::from(s)).collect();

        if v.len() > 2 {
            return Err(errors::MyErr::new("Error: Too many inputs"));
        }

        // TODO: Replace replace with trim
        v[1] = v[1].replace('\n', "");

        // println!("Vector contains: {} and {}", v[0], v[1]);
        // println!("{:?}, {:?}", v[0].parse::<i8>(), v[1].parse::<i8>());
        // println!("{:?}, {:?}", v.pop(), v.pop());

        // TODO: Combine both if let into one
        let (mut cx, mut cy): (i8, i8) = (-1, -1);
        if let Ok(value) = v[0].parse::<i8>() {
            cx = value - 1;
        } else {
            return Err(errors::MyErr::new(
                "Error: could not parse first val to a valid index\nPlease try again",
            ));
        }

        if let Ok(value) = v[1].parse::<i8>() {
            cy = value - 1;
        } else {
            return Err(errors::MyErr::new(
                "Error: could not parse first val to a valid index\nPlease try again",
            ));
        }

        // println!("PARSED INPUT: row {}, col {}", cx, cy);

        // Do input validation
        // Attempt legal move
        if (cx < 3 && cx >= 0) && (cy < 3 && cy >= 0) {
            if self.matrix[cx as usize][cy as usize] != None {
                return Err(errors::MyErr::new("Square already occupied dude..."));
            } else {
                self.matrix[cx as usize][cy as usize] = Some(*player);
                self.round += 1;
                Ok((cx, cy))
            }
        } else {
            Err(errors::MyErr::new(
                "Entered square is out of range\nPlease try again",
            ))
        }
    }
}

pub fn run() {
    /*
    * FLOW:
    Player plays as both X and Y.
    Draw board
    Wait for user input
    Validate input, then either re-prompt or write to board
    Check if game over
    */
    let alternate_player = |player: char| -> char {
        if player == 'O' {
            'X'
        } else {
            'O'
        }
    };

    let mut board: Board = Board::new();
    let mut curr_player: char = 'X';

    loop {
        println!("{}", board);
        println!("{}'s turn to play", curr_player);

        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        // Pass to board, which will take care of input validation
        let last_move: (i8, i8);
        match board.play(&mut input, &mut curr_player) {
            Ok((x, y)) => last_move = (x, y),
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        if game_is_over(&board, last_move, &curr_player) == "YES" {
            println!("{}", board);
            end_game(&curr_player, &mut board);
        } else if game_is_over(&board, last_move, &curr_player) == "DRAW" {
            println!("{}", board);
            end_game(&'D', &mut board);
        }
        curr_player = alternate_player(curr_player);
    }
}

// TODO: Both functions should perhaps be methods of Board if we imagine Board as Game
fn game_is_over(board: &Board, last_move: (i8, i8), player: &char) -> String {
    // Check draw
    if board
        .matrix
        .iter()
        .all(|row| row.iter().all(|square| *square != None))
    {
        return String::from("DRAW");
    }

    // TODO: Look into scalability
    // Check win
    let mut count = 0;
    let row: usize = last_move.0 as usize;
    let col: usize = last_move.1 as usize;

    for i in 0..3 {
        if board.matrix[row][i] == Some(*player) {
            count += 1;
            if count == 3 {
                return String::from("YES");
            }
        }
    }
    count = 0;

    for j in 0..3 {
        if board.matrix[j][col] == Some(*player) {
            count += 1;
            if count == 3 {
                return String::from("YES");
            }
        }
    }
    count = 0;

    let mut k = 0;
    while k < 3 {
        if board.matrix[k][k] == Some(*player) {
            count += 1;
            if count == 3 {
                return String::from("YES");
            }
        }
        k += 1;
    }

    if (board.matrix[0][2] == Some(*player))
        && (board.matrix[1][1] == Some(*player))
        && (board.matrix[2][0] == Some(*player))
    {
        return String::from("YES");
    }
    String::from("NO")
}

fn end_game(player: &char, board: &mut Board) {
    match player {
        'X' => {
            board.score.0 += 1;
            println!("Winner: {}", player);
        }
        'O' => {
            board.score.1 += 1;
            println!("Winner: {}", player);
        }
        'D' => println!("Match is a draw"),
        _ => (),
    }
    board.matrix = [[None; 3]; 3];
    board.round = 0;
}
