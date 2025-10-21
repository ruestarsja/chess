// crate::rules::knight

use crate::components::chess_board::ChessBoard;
use crate::components::chess_piece::ChessPiece;
use crate::utils::logs::log;

const POSSIBLE_MOVES: [[i8; 2]; 8] = [
    [2, 1],
    [1, 2],
    [-1, 2],
    [-2, 1],
    [-2, -1],
    [-1, -2],
    [1, -2],
    [2, -1]
];

pub fn is_valid_move(start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, chess_board: &ChessBoard) -> bool {
    log("INFO", "Validating a potential knight move...");

    // Ensure start is on the board
    if start_rank >= 8 || start_file >= 8 {
        log(
            "ERROR",
            format!("crate::rules::knight::is_valid_move received an invalid starting position: file #{}, rank #{}", start_file, start_rank)
        );
        panic!("Invalid starting rank or file: rank {}, file {}", start_rank, start_file);
    }
    log("INFO", "The start position is on the board.");

    // Ensure target is on the board
    if target_rank >= 8 || target_file >= 8 {
        log("INFO", "Bad move: The target position is not on the board.");
        return false;
    }
    log("INFO", "The target position is on the board");

    // Ensure start and target are different spaces
    if start_rank == target_rank && start_file == target_file {
        log("INFO", "Bad move: The start and target positions are the same.");
        return false;
    }
    log("INFO", "The start and target positions are different.");

    // Ensure target doesn't contain a friendly piece
    let start_space: &ChessPiece = chess_board.borrow_space_contents(start_rank, start_file);
    let target_space: &ChessPiece = chess_board.borrow_space_contents(target_rank, target_file);
    if start_space.get_color() == target_space.get_color() {
        log("INFO", "Bad move: The target position contains a friendly piece.");
        return false;
    }
    log("INFO", "The target position does not contain a friendly piece.");

    // Ensure move is a valid pattern for a knight
    let possible_move: [i8; 2] = [(target_rank as i8) - (start_rank as i8), (target_file as i8) - (start_file as i8)];
    if !POSSIBLE_MOVES.contains(&possible_move) {
        log("INFO", "Bad move: The pattern of the potential move is not in the list of valid move patterns for a knight.");
        return false;
    }
    log("INFO", "The pattern of the potential move is in the list of valid move patterns for a knight.");

    // Passed all checks
    log("INFO", "The move has passed all checks.");
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
