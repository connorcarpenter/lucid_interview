// /*******************************************************************************
//  * NOTE: For g++ or clang++ compile with flag --std=c++17 if needed.
//  *******************************************************************************/
//
// /*******************************************************************************
//  * CONFIDENTIAL
//  *
//  * Please do not share this code with anyone outside of Lucid Software.
//  ******************************************************************************/
//
// #include <algorithm>
// #include <iostream>
// #include <iomanip>
// #include <map>
// #include <string>
// #include <sstream>
// #include <vector>
//
// using namespace std;
//
// /*
//  * Part 1: Given a list of numbers, return the 3 largest numbers that are divisible by 7
//  * sorted from largest to smallest, for example, given the list
//  * [14,3,21,17,7,70]  this function should return [70,21,14]
//  * Note:  you can assume for purposes of this problem that all input will have at least
//  * three integers that are divisible by 7
//  */
// vector<int> findLargestThreeIntsDivisibleBySeven(vector<int> input)
// {
// vector<int> result;
// // your code here
// return result;
// }
//
// /*
//  * Part 2: Given a string of space separated words return a new string
//  * that is composed of the last letter of each word in the input
//  *    Example:
//  *    Given: "sell emu anthropomorphic alveoli acid"
//  *    Result: "lucid"
//  */
// string catLastLetter(string input)
// {
// string result = "";
// // your code here
// return result;
// }
//
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
//
// /*
//  * END YOUR WORK HERE
//  */
// void printIntVector(vector<int> v)
// {
// string result;
// for (unsigned i = 0; i < v.size(); i++)
// {
// if (i > 0) {
// cout << ",";
// }
// cout << v[i];
// }
// }
//
// void TestPart1(vector<int> input, vector<int> expectedResult)
// {
// vector<int> result = findLargestThreeIntsDivisibleBySeven(input);
// if (result == expectedResult)
// {
// cout << "Expected and got ";
// printIntVector(expectedResult);
// cout << " from input of ";
// printIntVector(input);
// cout << "\n";
// }
// else
// {
// cout << "Expected ";
// printIntVector(expectedResult);
// cout << " but got ";
// printIntVector(result);
// cout << " from input of ";
// printIntVector(input);
// cout << "\n";
// }
// }
//
// void TestPart2(string input, string expectedResult)
// {
// string result = catLastLetter(input);
// if (result == expectedResult)
// {
// cout << "Expected and got " << result << " from input of " << input << "\n";
// }
// else
// {
// cout << "Expected " << expectedResult << " but got " << result << " for input string of " << input << "\n";;
// }
// }
//
// int main(void) {
// cout << "Part 1:" << endl;
// cout << "------" << endl;
// TestPart1(vector<int>{ 7, 4, 21, 49, 6, 20, 14 }, vector<int>{ 49, 21, 14 });
// TestPart1(vector<int>{ 7, 14, 21 }, vector<int>{ 21, 14, 7 });
// TestPart1(vector<int>{ 35, 7, 14, 20, 28, 42 }, vector<int>{ 42, 35, 28 });
// TestPart1(vector<int>{ 105, 7, 140, 11, 12, 13 }, vector<int>{ 140, 105, 7 });
//
// cout << "Part 2:" << endl;
// cout << "------" << endl;
// TestPart2("sell emu anthropomorphic alveoli acid", "lucid");
// TestPart2("w o r d", "word");
// TestPart2("z", "z");
// TestPart2("correct horse battery staple", "teye");
//
// cout << "Part 3:" << endl;
// Game g;
// g.play();
// }
//
// /*******************************************************************************
//  * CONFIDENTIAL
//  *
//  * Please do not share this code with anyone outside of Lucid Software.
//  ******************************************************************************/