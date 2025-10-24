// crate::rules::king

use crate::components::chess_board::ChessBoard;
use crate::components::chess_piece::ChessPiece;
use crate::utils::logs::log;

pub fn is_valid_move(
    start_rank: u8,
    start_file: u8,
    target_rank: u8,
    target_file: u8,
    chess_board: &ChessBoard
) -> bool {
    log("INFO", "Validating a potential king move...");
    log("NOTE", "crate::rules::king::is_valid_move does not currently ensure that the space the king moves to is unthreatened.");

    // TODO: CHECK FOR AND ALLOW CASTLING (have to figure out a way to move the rook too)
    // TODO: ENSURE THERE'S NO ACTIVE THREAT ON TARGET

    // Ensure start is on the board
    if start_rank >= 8 || start_file >= 8 {
        log(
            "ERROR",
            format!(
                "crate::rules::king::is_valid_move received an invalid starting position: file #{}, rank #{}",
                start_file,
                start_rank
            )
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

    // For a target greater than one square away:
    if ( (start_file as i8 - target_file as i8).abs() > 1 )
    || ( (start_rank as i8 - target_rank as i8).abs() > 1 ) {
        log("INFO", "The target position is not within one file and one rank of the starting position.");
        
        // Ensure that the target is in the same rank as the start
        if start_rank != target_rank {
            log("INFO", "Bad move: The target position is not in the same rank as the starting position.");
            return false;
        }

        // Ensure that the target is exactly two squares to the left or right of the start
        if (start_file as i8 - target_file as i8).abs() != 2 {
            log("INFO", "Bad move: The target position is not exactly two squares left or right of the starting position.");
            return false;
        }

        // Ensure that the king has not yet moved
        if chess_board.borrow_space_contents(start_rank, start_file).has_moved() {
            log("INFO", "Bad move: The king has already been moved.");
            return false;
        }

        // For a queenside castle:
        if target_file < start_file {
            log("INFO", "Detected a queenside castle attempt.");

            let rook_spot = chess_board.borrow_space_contents(start_rank, ChessBoard::get_file(String::from("a")));

            // Ensure there is a rook in file a
            if !rook_spot.is_rook() {
                log("INFO", "Bad move: File a in this rank does not contain a rook.");
                return false;
            }
            log("INFO", "File a in this rank contains a rook.");

            // Ensure the rook is friendly
            if rook_spot.get_color() != chess_board.borrow_space_contents(start_rank, start_file).get_color() {
                log("INFO", "Bad move: The rook in file a is not friendly.");
                return false;
            }
            log("INFO", "The rook in file a is friendly.");

            // Ensure the rook has not moved
            if rook_spot.has_moved() {
                log("INFO", "Bad move: The rook has already moved.");
                return false;
            }
            log("INFO", "The rook has not moved.");

            // Ensure every space between the king and the rook is empty
            let mut file = ChessBoard::get_file(String::from("b"));
            while file < start_file {
                if !chess_board.borrow_space_contents(start_rank, file).is_empty() {
                    log(
                        "INFO",
                        format!(
                            "Bad move: The position {}{}, between the king and the rook, is not empty.",
                            ChessBoard::get_file_label(file),
                            ChessBoard::get_rank_label(start_rank)
                        )
                    );
                    return false;
                }
                file += 1;
            }
            log("INFO", "All positions between the king and the rook are empty.");

            // TODO: add checks for king safety
            log("NOTE", "Not currently checking for king safety in kingside castling.");

        // For a kingside castle:
        } else {
            log("INFO", "Detected a kingside castle attempt.");

            let rook_spot = chess_board.borrow_space_contents(start_rank, ChessBoard::get_file(String::from("h")));

            // Ensure there is a rook in file h
            if !rook_spot.is_rook() {
                log("INFO", "Bad move: File h in this rank does not contain a rook.");
                return false;
            }
            log("INFO", "File h in this rank contains a rook.");

            // Ensure the rook is friendly
            if rook_spot.get_color() != chess_board.borrow_space_contents(start_rank, start_file).get_color() {
                log("INFO", "Bad move: The rook in file h is not friendly.");
                return false;
            }
            log("INFO", "The rook in file h is friendly.");

            // Ensure the rook has not moved
            if rook_spot.has_moved() {
                log("INFO", "Bad move: The rook has already moved.");
                return false;
            }
            log("INFO", "The rook has not moved.");

            // Ensure every space between the king and the rook is empty
            let mut file = ChessBoard::get_file(String::from("g"));
            while file < start_file {
                if !chess_board.borrow_space_contents(start_rank, file).is_empty() {
                    log(
                        "INFO",
                        format!(
                            "Bad move: The position {}{}, between the king and the rook, is not empty.",
                            ChessBoard::get_file_label(file),
                            ChessBoard::get_rank_label(start_rank)
                        )
                    );
                    return false;
                }
                file -= 1;
            }
            log("INFO", "All positions between the king and the rook are empty.");

            // TODO: add checks for king safety
            log("NOTE", "Not currently checking for king safety in queenside castling.");
        }
    
    }

    // Passed all checks
    log("INFO", "The move has passed all checks.");
    return true;
    
}
