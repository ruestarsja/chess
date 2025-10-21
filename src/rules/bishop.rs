// crate::rules::bishop

use crate::components::chess_board::ChessBoard;
use crate::components::chess_piece::ChessPiece;
use crate::utils::logs::log;

pub fn is_valid_move(start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, chess_board: &ChessBoard) -> bool {
    log("INFO", "Validating a potential bishop move...");

    // Ensure start is on the board
    if start_rank >= 8 || start_file >= 8 {
        log(
            "ERROR",
            format!("crate::rules::bishop::is_valid_move received an invalid starting position: file #{}, rank #{}", start_file, start_rank)
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

    // Ensure start and target share a diagonal
    if (start_rank as i8 - target_rank as i8).abs() != (start_file as i8 - target_file as i8).abs() {
        log("INFO", "Bad move: The start and target positions do not share a diagonal.");
        return false;
    }
    log("INFO", "The start and target positions share a diagonal.");

    // Ensure all spaces between start and target are empty
    let rank_increment: i8 = if target_rank > start_rank { 1 } else { -1 };
    let file_increment: i8 = if target_file > start_file { 1 } else { -1 };
    let mut current_rank: i8 = start_rank as i8 + rank_increment;
    let mut current_file: i8 = start_file as i8 + file_increment;
    while current_rank * rank_increment < target_rank as i8 * rank_increment
       && current_file * file_increment < target_file as i8 * file_increment {
        if !chess_board.borrow_space_contents(current_rank as u8, current_file as u8).is_empty() {
            log(
                "INFO",
                format!(
                    "Bad move: The position {}{}, between the start and target positions, is not empty.",
                    ChessBoard::get_file_label(current_file as u8),
                    ChessBoard::get_rank_label(current_rank as u8)
                )
            );
            return false;
        }
        current_rank += rank_increment;
        current_file += file_increment;
    }
    log("INFO", "All positions between the start and target positions are empty.");

    // Passed all checks
    log("INFO", "The move has passed all checks.");
    return true;

}
