use labyrinth_maker::{Board, Difficulty};

fn main() {
    let board = Board::build_board_dif(Difficulty::VeryHard);
    println!("{}", board);
}
