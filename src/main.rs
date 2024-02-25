use std::io;
use rand::Rng;

struct GameTable([[char; 3]; 3], [[char; 4]; 4]);

impl GameTable {
    fn calculate_winner(&self) -> char {
        const EMPTY: char = ' ';

        const X_PLAYER: char = 'X';
        const Y_PLAYER: char = 'Y';
        const CONTINUE: char = 'C';
        const NO_WIN: char = 'N';

        return CONTINUE;
        
    }

    fn player_sign(&mut self, point: Choord, symbol: char) -> bool {
        const MAX_RANGE: usize = 3;
        const MIN_RANGE: usize = 0;

        let index_greater: bool = point.x > MAX_RANGE || point.y > MAX_RANGE;
        let index_lesser: bool = point.x < MIN_RANGE || point.y < MIN_RANGE;

        if index_greater || index_lesser {
            return false;
        }
        // TODO check if the point is already used
        self.0[point.x][point.y] = symbol;
        self.1[point.x+1][point.y+1] = symbol;
        true
    }

    fn print_game_table(&self) {
        let table = self.1;
        for row in table {
            println!(
                "{} {} {} {}",
                row[0],
                row[1],
                row[2],
                row[3],
            )
        }
    }

    fn default() -> Self {
        let mut game_table = Self([['_'; 3]; 3], [['_'; 4]; 4]);

        game_table.1[0] = [' ', '0', '1', '2'];
        game_table.1[1][0] = '0';
        game_table.1[2][0] = '1';
        game_table.1[3][0] = '2';

        return game_table
    }
}

struct Player {
    symbol: char,
}

struct Choord {
    x: usize,
    y: usize,
}

impl Choord {
    fn read_from_line(line: &str) -> Self {
        const LINE_WIDTH: usize = 4;

        assert_eq!(line.len(), LINE_WIDTH);

        let x = line[0..1].trim().parse::<usize>()
            .expect("This is not a valid input for x");

        let y = line[2..3].trim().parse::<usize>()
            .expect("This is not a valid input for y");

        Self { x, y }
    }
}

struct TicTacToe {
    table: GameTable,
    players: [Player; 2],
}

impl TicTacToe {

    fn start(&mut self) -> bool {
        let start_msg = String::from("the game is on");

        println!("{}", start_msg);

        let mut current_player: usize = rand::thread_rng().gen_range(0..=1);

        loop {
            let result: char = self.table.calculate_winner();
            if result == 'N' {
                println!("No one win, you are terrible");
                break false
            } else if result == 'C' {
                self.table.print_game_table();
                println!("Player {} is your turn!", self.players[current_player].symbol);
                self.table.player_sign(
                    TicTacToe::player_input(), 
                    self.players[current_player].symbol
                );
                current_player = TicTacToe::swap_player(current_player);
            } else {
                println!("You win"); // TODO
                break true
            }
        }
    }

    fn swap_player(player_index: usize) -> usize {
        if player_index == 0 { 1 } else { 0 }
    }

    fn player_input() -> Choord{
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        
        
        return Choord::read_from_line(&mut input);
    }

    fn build_game() -> Self {
        Self {
            table: GameTable::default(),
            players: [
                Player { symbol: 'X' },
                Player { symbol: 'Y' },
            ]
        }
    }
}

fn main() {
    let mut game = TicTacToe::build_game();
    game.start();
}
