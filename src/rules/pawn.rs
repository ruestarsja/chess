// crate::rules::pawn

use crate::components::chess_board::ChessBoard;
use crate::components::chess_piece::ChessPiece;
use crate::utils::logs::log;

pub fn is_valid_move(start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, chess_board: &ChessBoard, last_move: &Option<((u8, u8), (u8, u8))>) -> bool {

    log("INFO", "Validating a potential pawn move...");
    log("NOTE", "crate::rules::pawn::is_valid_move does not currently check for and allow en passant.");
    log("NOTE", "crate::rules::king::is_valid_move does not currently check for and allow pawn promotion.");

    // TODO: CHECK FOR AND ALLOW EN PASSANT (have to figure out a way to capture the other pawn too)
    // TODO: figure out a way to implement pawn promotion

    // Ensure start is on the board
    if start_rank >= 8 || start_file >= 8 {
        log(
            "ERROR",
            format!("crate::rules::pawn::is_valid_move received an invalid starting position: file #{}, rank #{}", start_file, start_rank)
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

    // Ensure target is within 1 file either direction of start
    if (start_file as i8 - target_file as i8).abs() > 1 {
        log("INFO", "Bad move: The target position is not within one file either direction of the starting position.");
        return false;
    }
    log("INFO", "The target position is within one file either direction of the starting position.");

    // For a white pawn:
    if start_space.is_white() {
        log("INFO", "The piece to move is a white pawn.");

        // For a space in the same file:
        if start_file == target_file {
            log("INFO", "The target position is in the same file as the starting position.");
            
            // Ensure target doesn't contain a piece
            if !chess_board.borrow_space_contents(target_rank, target_file).is_empty() {
                log("INFO", "Bad move: The target position contains a piece.");
                return false;
            }
            log("INFO", "The target position does not contain a piece.");

            // For a pawn still in home row (rank 2):
            if start_rank == ChessBoard::get_rank(String::from("2")) {
                log("INFO", "The start position is in the pawn's home row.");

                // Ensure target is within 2 spaces ahead of start
                if target_rank >= start_rank || target_rank < start_rank - 2 {
                    log("INFO", "Bad move: The target position is not within two spaces ahead of the starting position.");
                    return false;
                }
                log("INFO", "The target position is within two spaces ahead of the starting position.");

            // For a pawn not still in home row:
            } else {
                log("INFO", "The start position is not in the pawn's home row.");

                // Ensure target is the space ahead of start
                if target_rank != start_rank - 1 {
                    log("INFO", "Bad move: The target position is not the space immediately ahead of the starting position.");
                    return false;
                }
                log("INFO", "The target position is the space immediately ahead of the starting position.");
            }
            
        // For a space in an adjacent file:
        } else {
            log("INFO", "The target position is in an adjacent file to the starting position.");

            // Ensure target is in the rank ahead of start
            if target_rank != start_rank - 1 {
                log("INFO", "Bad move: The target position is not in the rank immediately ahead of the starting position.");
                return false;
            }
            log("INFO", "The target position is in the rank immediately ahead of the starting position.");

            // For target being empty:
            if chess_board.borrow_space_contents(target_rank, target_file).is_empty() {
                log("INFO", "The target position does not contain a piece.");
                // Ensure the move is an en passant
                match last_move {
                    None => {
                        log("INFO", "Bad move: This is the first move, cannot be en passant.");
                        return false;
                    },
                    Some(_move) => {
                        log("INFO", "There was a previous move.");
                        // Ensure there is a pawn in the correct relative space
                        if !chess_board.borrow_space_contents(target_rank + 1, target_file).is_pawn() {
                            log(
                                "INFO", 
                                format!(
                                    "Bad move: The position {}{}, below the target position, is not a pawn.",
                                    ChessBoard::get_file_label(target_file),
                                    ChessBoard::get_rank_label(target_rank + 1)
                                )
                            );
                            return false
                        }

                        // Ensure pawn just moved there with a double move
                        if *_move != ((target_file, target_rank - 1), (target_file, target_rank + 1)) {
                            log("INFO", "Bad move: Pawn has not just moved with a double move.");
                            return false;
                        }
                        log("INFO", "This is a valid en passant move.");
                    }
                }
            }
        }

    // For a black pawn:
    } else {
        log("INFO", "The piece to move is a black pawn.");

        // For a space in the same file:
        if start_file == target_file {
            log("INFO", "The target position is in the same file as the starting position.");

            // Ensure target doesn't contain a piece
            if !chess_board.borrow_space_contents(target_rank, target_file).is_empty() {
                log("INFO", "Bad move: The target position contains a piece.");
                return false;
            }
            log("INFO", "The target position does not contain a piece.");

            // For a pawn still in home row (rank 7):
            if start_rank == ChessBoard::get_rank(String::from("7")) {
                log("INFO", "The starting position is in the pawn's home row.");

                // Ensure target is within 2 spaces ahead of start
                if target_rank <= start_rank || target_rank > start_rank + 2 {
                    log("INFO", "Bad move: The target position is not within two spaces ahead of the starting position.");
                    return false;
                }
                log("INFO", "The target position is within two spaces ahead of the starting position.");

            // For a pawn not still in home row:
            } else {
                log("INFO", "The starting position is not in the pawn's home row.");

                // Ensure target is the space ahead of start
                if target_rank != start_rank + 1 {
                    log("INFO", "Bad move: The target position is not the space immediately ahead of the starting position.");
                    return false;
                }
                log("INFO", "The target position is the space immediately ahead of the starting position.");
            }
            
        // For a space in an adjacent file:
        } else {
            log("INFO", "The target position is in an adjacent file to the starting position.");

            // Ensure target is in the rank ahead of start
            if target_rank != start_rank + 1 {
                log("INFO", "Bad move: The target position is not in the rank immediately ahead of the starting position.");
                return false;
            }
            log("INFO", "The target position is in the rank immediately ahead of the starting position.");

            // For target being empty:
            if chess_board.borrow_space_contents(target_rank, target_file).is_empty() {
                log("INFO", "The target position does not contain a piece.");
                // Ensure the move is an en passant
                match last_move {
                    None => {
                        log("INFO", "Bad move: This is the first move, cannot be en passant.");
                        return false;
                    },
                    Some(_move) => {
                        log("INFO", "There was a previous move.");
                        // Ensure there is a pawn in the correct relative space
                        if !chess_board.borrow_space_contents(target_rank - 1, target_file).is_pawn() {
                            log(
                                "INFO", 
                                format!(
                                    "Bad move: The position {}{}, below the target position, is not a pawn.",
                                    ChessBoard::get_file_label(target_file),
                                    ChessBoard::get_rank_label(target_rank - 1)
                                )
                            );
                            return false
                        }

                        // Ensure pawn just moved there with a double move
                        if *_move != ((target_file, target_rank + 1), (target_file, target_rank - 1)) {
                            log("INFO", "Bad move: Pawn has not just moved with a double move.");
                            return false;
                        }
                        log("INFO", "This is a valid en passant move.");
                    }
                }
            }
        }
    }

    // Passed all checks
    log("INFO", "The move has passed all checks.");
    return true;
    
}
