// crate::rules::king

use crate::components::chess_board::ChessBoard;
use crate::components::chess_piece::ChessPiece;

pub fn is_valid_move(start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, chess_board: &ChessBoard) -> bool {

    println!("VALIDATING KING MOVE:");

    // TODO: CHECK FOR AND ALLOW CASTLING (have to figure out a way to move the rook too)
    // TODO: ENSURE THERE'S NO ACTIVE THREAT ON TARGET

    // Ensure start is on the board
    if start_rank >= 8 || start_file >= 8 {

        println!("\tOH NO! Start is not on the board");

        panic!("Invalid starting rank or file: rank {}, file {}", start_rank, start_file);
    }

    println!("Start is on the board.");

    // Ensure target is on the board
    if target_rank >= 8 || target_file >= 8 {

        println!("\tOH NO! Target is not on the board.");

        return false;
    }

    println!("Target is on the board.");

    // Ensure start and target are different spaces
    if start_rank == target_rank && start_file == target_file {

        println!("\tOH NO! Start and target are the same space.");

        return false;
    }

    println!("Start and target are different spaces.");

    // Ensure target doesn't contain a friendly piece
    let start_space: &ChessPiece = chess_board.borrow_space_contents(start_rank, start_file);
    let target_space: &ChessPiece = chess_board.borrow_space_contents(target_rank, target_file);
    if start_space.get_color() == target_space.get_color() {

        println!("\tOH NO! Target contains a friendly piece.");

        return false;
    }

    println!("Target doesn't contain a friendly piece.");

    // Ensure target is within one square of start
    if ( (start_file as i8 - target_file as i8).abs() > 1 )
    || ( (start_rank as i8 - target_rank as i8).abs() > 1 ) {

        println!("\tOH NO! Target is not within one square of start.");

        return false;
    }

    println!("Target is within one square of start.");

    println!("KING MOVE PASSED ALL CHECKS.");

    // Passed all checks
    return true;
    
}

// fn get_valid_moves(start_rank: u8, start_file: u8) -> Vec<[u8; 2]> {
//     if start_rank >= 8 || start_file >= 8 {
//         panic!("Invalid starting rank or file: rank {}, file {}", start_rank, start_file);
//     }
//     let mut moves: Vec<[u8; 2]> = vec![];
//     for possible_move in POSSIBLE_MOVES {
//         let end_rank: i8 = (start_rank as i8) + possible_move[0];
//         let end_file: i8 = (start_file as i8) + possible_move[0];
//         if (0..8).contains(&end_rank) && (0..8).contains(&end_file) {
//             moves.push([end_rank as u8, end_file as u8]);
//         }
//     }
//     return moves;
// }
