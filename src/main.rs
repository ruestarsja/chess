extern crate text_io;

mod components;
mod rules;
mod utils;


use text_io::read;

use components::chess_board::ChessBoard;
use utils::logs::init_logs;


fn main() {
    
    let mut chess_board = ChessBoard::default();
    let mut is_black_turn: bool = false;
    init_logs();

    let mut running: bool = true;
    while running {

        println!();
        println!("{}", chess_board);
        println!();
        print!(" > ");
        
        let input: String = read!("{}\n");

        if input == "exit" {
            println!();
            println!("Goodbye!");
            println!();
            running = false;

        } else if input == "move" {
            println!();
            print!(" Starting File Label: ");
            let start_file_label: String = read!("{}\n");
            print!(" Starting Rank Label: ");
            let start_rank_label: String = read!("{}\n");
            print!(" Target File Label: ");
            let target_file_label: String = read!("{}\n");
            print!(" Target Rank Label: ");
            let target_rank_label: String = read!("{}\n");
            chess_board.move_piece(
                ChessBoard::get_rank(start_rank_label),
                ChessBoard::get_file(start_file_label),
                ChessBoard::get_rank(target_rank_label),
                ChessBoard::get_file(target_file_label)
            );
        }

    }

}
