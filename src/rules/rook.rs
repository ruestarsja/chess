// crate::rules::rook

use crate::components::chess_board::ChessBoard;
use crate::components::chess_piece::ChessPiece;

pub fn is_valid_move(start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, chess_board: &ChessBoard) -> bool {

    println!("VALIDATING ROOK MOVE:");

    // Ensure start is on the board
    if start_rank >= 8 || start_file >= 8 {

        println!("\tOH NO! Start is not on the board.");

        panic!("Invalid starting rank or file: rank {}, file {}", start_rank, start_file);
    }

    println!("\tStart is on the board.");

    // Ensure target is on the board
    if target_rank >= 8 || target_file >= 8 {

        println!("\tOH NO! Target is not on the board.");

        return false;
    }

    println!("\tTarget is on the board.");

    // Ensure start and target are different spaces
    if start_rank == target_rank && start_file == target_file {

        println!("\tOH NO! Start and target are the same space.");

        return false;
    }

    println!("\tStart and target are different spaces.");

    // Ensure target doesn't contain a friendly piece
    let start_space: &ChessPiece = chess_board.borrow_space_contents(start_rank, start_file);
    let target_space: &ChessPiece = chess_board.borrow_space_contents(target_rank, target_file);
    if start_space.get_color() == target_space.get_color() {

        println!("\tOH NO! Target contains a friendly piece.");

        return false;
    }

    println!("\tTarget does not contain a friendly piece.");

    // Ensure start and target share a file or rank
    if ( start_file != target_file ) && ( start_rank != target_rank ) {

        println!("\tOH NO! Start and target do not share a file or rank");

        return false;
    }

    println!("\tStart and target share a file or rank.");

    // For vertical movement, ensure all spaces between start and target are empty
    if start_file == target_file {

        println!("\tSTART AND TARGET SHARE A FILE.");

        let rank_increment: i8 = if target_rank > start_rank { 1 } else { -1 };
        let mut current_rank: i8 = start_rank as i8 + rank_increment;
        while current_rank * rank_increment < target_rank as i8 * rank_increment {
            if !chess_board.borrow_space_contents(current_rank as u8, start_file).is_empty() {

                println!(
                    "\tOH NO! A space ({}{}) between start and target is not empty.",
                    ChessBoard::get_file_label(start_file),
                    ChessBoard::get_rank_label(current_rank as u8)
                );

                return false;
            }
            current_rank += rank_increment;
        }

        println!("\tAll spaces between start and target are empty.");

    // For horizontal movement, ensure all spaces between start and target are empty
    } else {

        println!("\tSTART AND TARGET SHARE A RANK.");

        let file_increment: i8 = if target_file > start_file { 1 } else { -1 };
        let mut current_file: i8 = start_file as i8 + file_increment;
        while current_file * file_increment < target_file as i8 * file_increment {
            if !chess_board.borrow_space_contents(start_rank, current_file as u8).is_empty() {

                println!(
                    "\tOH NO! A space ({}{}) between start and target is not empty.",
                    ChessBoard::get_file_label(current_file as u8),
                    ChessBoard::get_rank_label(start_rank)
                );

                return false;
            }
            current_file += file_increment;
        }

        println!("\tAll spaces between start and target are empty.");

    }

    println!("ROOK MOVE PASSED ALL CHECKS.");

    // Passed all checks
    return true;
    
}
