use std::collections::HashSet;

use rand::Rng;

#[test]
fn example_1() {
    let mut random = rand::thread_rng();

    let mut game = Game::new_random();
    game.print();
    game.print_mines();

    let mut iterations = 0;
    loop {
        let x = random.gen_range(0..GAME_SIZE);
        let y = random.gen_range(0..GAME_SIZE);
        let input = MoveInput::new(x, y);
        game.recv_input(input);

        game.print();

        iterations += 1;
        if iterations > 100 {
            println!("done");
            break;
        }
        println!("onto next input!");
    }
}

// class Game {
//
// private:
// const static int SIZE = 8;
// Square* board[SIZE][SIZE];
// int numberOfMines = 10;
// bool mineGrid[SIZE][SIZE];

const GAME_SIZE: usize = 8;
const NUMBER_OF_MINES: u32 = 3;

struct Game {
    board: [[Square; GAME_SIZE]; GAME_SIZE],
    minegrid: [[bool; GAME_SIZE]; GAME_SIZE],
    is_done: bool,
}

impl Game {
    fn new_random() -> Self {

        let mut random = rand::thread_rng();

        let mut minegrid = [[false; GAME_SIZE]; GAME_SIZE];

        let mut current_number_of_mines = 0;
        loop {
            let x = random.gen_range(0..GAME_SIZE);
            let y = random.gen_range(0..GAME_SIZE);
            if minegrid[x][y] {
                continue;
            } else {
                minegrid[x][y] = true;
                current_number_of_mines += 1;
                if current_number_of_mines == NUMBER_OF_MINES {
                    break;
                }
            }
        }

        Self {
            board: [[Square::Unrevealed; GAME_SIZE]; GAME_SIZE],
            minegrid,
            is_done: false,
        }
    }

    fn print(&self) {
        println!("Game State:");
        for x in 0..GAME_SIZE {
            let mut next_line = String::new();
            for y in 0..GAME_SIZE {
                let cell_char = self.board[x][y].to_char();
                next_line.push(cell_char);
            }
            println!("{}", next_line);
        }
        println!("---");
    }

    fn print_mines(&self) {
        println!("Mine State:");
        for x in 0..GAME_SIZE {
            let mut next_line = String::new();
            for y in 0..GAME_SIZE {
                let cell_char = match self.minegrid[x][y] {
                    true => 'X',
                    false => '.',
                };
                next_line.push(cell_char);
            }
            println!("{}", next_line);
        }
        println!("---");
    }

    fn recv_input(&mut self, input: MoveInput) {

        let ix = input.0;
        let iy = input.1;

        println!("Received Input: ({}, {})", ix, iy);

        let cell_state = self.board[ix][iy];
        match cell_state {
            Square::Empty | Square::Hint(_) => {
                println!("Cell ({}, {}) is already revealed. Input does nothing.", ix, iy);
            }
            Square::Unrevealed => {
                // okay, reveal the cell
                // it is a hint, a mine, or an empty space
                let is_mine = self.minegrid[ix][iy];
                if is_mine {
                    println!("YOU LOSE!");
                    self.is_done = true;
                    return;
                }

                let adjacent_mines = self.adjacent_mines(ix, iy);
                if adjacent_mines > 0 {
                    self.board[ix][iy] = Square::Hint(adjacent_mines);
                } else {
                    // reveal everything!
                    self.board[ix][iy] = Square::Empty;
                    self.clear_from_empty_cell(ix, iy);
                }
            }
        }
    }

    fn adjacent_mines(&self, x: usize, y: usize) -> u8 {
        let mut output = 0;

        if self.get_mine_cell(x + 1, y) == Some(true) {
            output += 1;
        }
        if x > 0 && self.get_mine_cell(x - 1, y) == Some(true) {
            output += 1;
        }
        if self.get_mine_cell(x, y + 1) == Some(true) {
            output += 1;
        }
        if y > 0 && self.get_mine_cell(x, y - 1) == Some(true) {
            output += 1;
        }

        if y > 0 && self.get_mine_cell(x + 1, y - 1) == Some(true) {
            output += 1;
        }
        if x > 0 && y > 0 && self.get_mine_cell(x - 1, y - 1) == Some(true) {
            output += 1;
        }
        if self.get_mine_cell(x + 1, y + 1) == Some(true) {
            output += 1;
        }
        if x > 0 && self.get_mine_cell(x - 1, y + 1) == Some(true) {
            output += 1;
        }

        output
    }

    fn clear_from_empty_cell(&mut self, x: usize, y: usize) {
        if self.board[x][y] != Square::Empty {
            panic!("shouldn't have called this on a non-empty cell")
        }
        self.board[x][y] = Square::Empty;

        let mut visited_cells = HashSet::new();
        visited_cells.insert((x, y));

        if y > 0 { self.recursive_flood_fill(&mut visited_cells, x, y-1); }
        if x > 0 { self.recursive_flood_fill(&mut visited_cells, x-1, y); }
        self.recursive_flood_fill(&mut visited_cells, x, y+1);
        self.recursive_flood_fill(&mut visited_cells, x+1, y);
    }

    fn recursive_flood_fill(&mut self, visited_cells: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
        if visited_cells.contains(&(x, y)) {
            return;
        }
        if x >= GAME_SIZE || y >= GAME_SIZE {
            return;
        }
        visited_cells.insert((x, y));

        match self.board[x][y] {
            Square::Empty | Square::Hint(_) => return,
            Square::Unrevealed => {
                self.recv_input(MoveInput::new(x, y));
            }
        }
    }

    // fn set_board_cell(&mut self, x: usize, y: usize, sq: Square) {
    //     if x >= GAME_SIZE || y >= GAME_SIZE {
    //         return None;
    //     }
    // }

    fn get_mine_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x >= GAME_SIZE || y >= GAME_SIZE {
            return None;
        }

        return Some(self.minegrid[x][y]);
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Square {
    Unrevealed,
    // Mine,
    Hint(u8),
    Empty,
}

impl Square {
    fn to_char(&self) -> char {
        match self {
            Square::Unrevealed => '?',
            // Square::Mine => 'X',
            Square::Hint(number_of_adjacent_mines) => {
                match number_of_adjacent_mines {
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    _ => panic!("should not be possible"),
                }
            },
            Square::Empty => '.',
        }
    }
}

struct MoveInput(usize, usize);

impl MoveInput {
    fn new(x: usize, y: usize) -> Self {
        if x >= GAME_SIZE || y >= GAME_SIZE {
            panic!("{}, {} is not a valid coordinate", x, y)
        }
        Self(x, y)
    }
}

// /*
//  * Part 3: Minesweeper is a single player game where the player attempts to discover the
//  * location of a given number of randomly placed mines on a 2-dimensional grid.
//  *
//  * When game play starts, the grid is actually uninitialized. The player's first move will cause
//  * the mine locations to be randomly generated on the map but guaranteed to not have a mine on
//  * or neighboring the location of the player's move.
//  *
//  * After the board is initialized, the player's move will execute.
//  * 1. If the selected square is a mine, the player loses.
//  * 2. If the revealed square has any mines in its 8 neighboring squares, then a numeral hint is
//  *    shown on that square indicating how many of the square's neighbors are mines.
//  * 3. If the revealed square has no neighboring mines, then each of its neighbors are recursively
//  *    revealed in the same manner.
//  *
//  * For the purposes of this game both orthogonal and diagonal count as neighbors
//  * The player is then to deduce from the revealed hints as much as possible what other spaces are
//  * safe to reveal.
//  * Every turn, the player selects a square on the board to reveal, and the map is revealed
//  * according to the 3 rules above.
//  *
//  * Consider the following 4x4 game configured to place 4 mines.
//  * The player starts by selecting a1. At this point, the map is generated in a way that guarantees
//  * that a1 is Empty (no mine in neighbors).
//  * In this case the underlying map is generated as follows (E's represent empty locations on the
//  * board, and M's represent mines):
//  *
//  *   a b c d
//  * 1 E E E E
//  * 2 E E M E
//  * 3 E E E M
//  * 4 E M M E
//  *
//  * Then, the a1 move then executes as follows:
//  *
//  * a1 is empty, and has no neighboring mines, and is revealed as such. Its neighbors are then also
//  * revealed.
//  * b1 has one neighboring mine (c2), so it is shown as a hint with the number 1.
//  * b2 has one neighboring mine (c2), so it is shown as a hint with the number 1.
//  * a2 is explored, and it has no neighboring mines, so it is revealed as such, and its contiguous
//  * neighbors are revealed.
//  * b3 has three neighboring mines (c2, b4, and c4), so the numeral 3 is shown.
//  * a3 has one neighboring mine (b4), the numeral 1 is shown.
//  * With that, the search stops. And the board looks as follows:
//  *
//  *   a b c d
//  * 1   1 ? ?
//  * 2   1 ? ?
//  * 3 1 3 ? ?
//  * 4 ? ? ? ?
//  *
//  * =========================================================
//  *  The game strategy you may need to create a computer player
//  *  You don't need understand this part to implement game
//  * =========================================================
//  * From here the player can deduce that c3 must not be a mine. (b1 has two neighbors one of which is a
//  * mine, and both of b1's neighbors are neighbors of b2. c3 is a unique neighbor of b2, but b2's one
//  * mine must be in one of b1's neighbors, so c3 must be safe.)
//  * For the player's next move, they select c3, and the square is revealed as a 4. That is the
//  * only square revealed for this move.
//  *
//  *   a b c d
//  * 1   1 ? ?
//  * 2   1 ? ?
//  * 3 1 3 4 ?
//  * 4 ? ? ? ?
//  * =========================================================
//  *
//  * Gameplay will continue until the user either reveals a mine and loses, or reveals every non-mine
//  * space and wins.
//  *
//  * An online game example: (The rule to initialize game might be different)
//  * http://minesweeperonline.com/#beginner
//  *
//  * Extra Credit Ideas (no particular order):
//  * - On startup, allow the player to input the width, height, and number of mines
//  * - Add a timer. After each move display how long has elapsed. Display the final time when the player wins
//  * - Implement a feature for the player to "flag" squares they believe to be mines. Don't allow the
//  *    player to select a flag square.
//  * - Create a computer player to find a safe move if one exists (without knowing `mineGrid`), and then take a
//  *    random safe move
//  */
//
// class Square {
// protected:
// char value;
//
// public:
// Square(char _value) {
// value = _value;
// }
//
// void display() {
// cout << " " << value;
// }
// };
//
// class Unrevealed: public Square {
// public:
// Unrevealed():Square('?') { }
// };
//
// class Mine: public Square {
// public:
// Mine():Square('*') {}
// };
// class NoNeighbors: public Square {
// public:
// NoNeighbors():Square(' ') {}
// };
//
// class Hint: public Square {
// public:
// Hint(int hint):Square((char)hint + '0') {}
// };
//
// class Coordinate {
// public:
// int x;
// int y;
//
// Coordinate(int _x, int _y) {
// x = _x;
// y = _y;
// }
// };
//
//
// class Game {
//
// private:
// const static int SIZE = 8;
// Square* board[SIZE][SIZE];
// int numberOfMines = 10;
// bool mineGrid[SIZE][SIZE];
//
// public:
// Game() {
// for (int row = 0; row < SIZE; row++) {
// for (int column = 0; column < SIZE; column++) {
// board[row][column] = new Unrevealed();
// mineGrid[row][column] = false;
// }
// }
// }
//
// /**
//  * Render the board
//  * You can ignore this function.
//  */
// void render() {
// cout << "   ";
// for (int column = 0; column < SIZE; column++) {
// cout << (char)(column + 'a') << " ";
// }
// cout << endl;
//
// for (int row = 0; row < SIZE; row++) {
// cout << setw(2) << row + 1;
// for (int column = 0; column < SIZE; column++) {
// board[row][column]->display();
// }
// cout << endl; // move to new line
// }
// }
//
// /* +==========+
//  * TODO - Implement game logic:
//  * 1. On the player's first move, randomly initialize the `mineGrid` with `numberOfMines`
//  *    randomly placed mines (ensuring the selected coordinate and all 8 neighboring
//  *    coordinates are not mines).
//  * 2. Implement the lose condition (if the player selects a mine), and display all mines
//  *    before exiting.
//  * 3. Implement the hint condition (if the player selects an empty space neighboring 1 or
//  *    more mines)
//  * 4. Implement the recursive reveal (if the revealed space has no neighboring mines)
//  * 5. Implement the win condition (if every un-revealed space has a mine underlying it),
//  *    and reveal the board.
//  */
// void play() {
// while (true) {
// render();
// Coordinate c = getMoveInput();
//
// //TODO: Fill in this function with the actual game logic.
//
// board[c.x][c.y] = new NoNeighbors();
// }
// }
//
// private:
// /**
//  * Parse input into coordinate
//  * For example:
//  * a3 will be new Coordinate(2, 0)
//  * You can ignore this function.
//  */
// Coordinate getMoveInput() {
// string move;
// cin >> move;
// return Coordinate(max(0, min(SIZE - 1, move[1] - '1')), max(0, min(SIZE - 1, move[0] - 'a')));
// }
// };