// crate::rules::pawn

use crate::components::chess_board::ChessBoard;
use crate::components::chess_piece::ChessPiece;

pub fn is_valid_move(start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, chess_board: &ChessBoard) -> bool {

    println!(
        "VALIDATING PAWN MOVE {}{} -> {}{}:",
        ChessBoard::get_file_label(start_file),
        ChessBoard::get_rank_label(start_rank),
        ChessBoard::get_file_label(target_file),
        ChessBoard::get_rank_label(target_rank)
    );
    println!(
        "({}-{} -> {}-{})",
        start_file, start_rank, target_file, target_rank
    );

    // TODO: CHECK FOR AND ALLOW EN PASSANT (have to figure out a way to capture the other pawn too)
    // TODO: figure out a way to implement pawn promotion

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

    // Ensure target is within 1 file either direction of start
    if (start_file as i8 - target_file as i8).abs() > 1 {

        println!("\tOH NO! Target is not within 1 file either direction of start.");

        return false;
    }

    println!("\tTarget is within 1 file either direction of start.");

    // For a white pawn:
    if start_space.is_white() {
        
        println!("\tMOVE PIECE IS A WHITE PAWN.");

        // For a space in the same file:
        if start_file == target_file {

            println!("\tTARGET IS IN THE SAME FILE AS START.");
            
            // Ensure target doesn't contain a piece
            if !chess_board.borrow_space_contents(target_rank, target_file).is_empty() {

                println!("\tOH NO! Target contains a piece.");

                return false;
            }

            println!("\tTarget does not contain a piece.");

            // For a pawn still in home row (rank 2):
            if start_rank == ChessBoard::get_rank(String::from("2")) {

                println!("\tSTART IS IN HOME ROW.");

                // Ensure target is within 2 spaces ahead of start
                if target_rank >= start_rank || target_rank < start_rank - 2 {

                    println!("\tOH NO! Target is not within 2 spaces ahead of start.");

                    return false;
                }

                println!("\tTarget is within 2 spaces ahead of start.");

            // For a pawn not still in home row:
            } else {

                println!("\tSTART IS NOT IN HOME ROW.");

                // Ensure target is the space ahead of start
                if target_rank != start_rank - 1 {

                    println!("\tOH NO! Target is not the space ahead of start.");

                    return false;
                }

                println!("\tTarget is the space ahead of start")

            }
            
        // For a space in an adjacent file:
        } else {

            println!("\tTARGET IS IN AN ADJACENT FILE TO START.");

            // Ensure target contains a piece
            if chess_board.borrow_space_contents(target_rank, target_file).is_empty() {

                println!("\tOH NO! Target does not contain a piece.");

                return false;
            }

            println!("\tTarget contains a piece.");

            // Ensure target is in the rank ahead of start
            if target_rank != start_rank - 1 {

                println!("\tOH NO! Target is not in the rank ahead of start.");

                return false;
            }

            println!("\tTarget is in the row ahead of start.");

        }

    // For a black pawn:
    } else {

        println!("\tMOVE PIECE IS A BLACK PAWN.");

        // For a space in the same file:
        if start_file == target_file {

            println!("\tTARGET IS IN THE SAME FILE AS START.");
            
            // Ensure target doesn't contain a piece
            if !chess_board.borrow_space_contents(target_rank, target_file).is_empty() {

                println!("\tOH NO! Target contains a piece.");

                return false;
            }

            println!("\tTarget does not contain a piece.");

            // For a pawn still in home row (rank 7):
            if start_rank == ChessBoard::get_rank(String::from("7")) {

                println!("\tSTART IS IN HOME ROW");

                // Ensure target is within 2 spaces ahead of start
                if target_rank <= start_rank || target_rank > start_rank + 2 {

                    println!("\tOH NO! Target is not within 2 spaces ahead of start.");

                    return false;
                }

                println!("\tTarget is within 2 spaces ahead of start.");

            // For a pawn not still in home row:
            } else {

                println!("\tSTART IS NOT IN HOME ROW.");

                // Ensure target is the space ahead of start
                if target_rank != start_rank + 1 {

                    println!("\tOH NO! Target is not the space ahead of start.");
                    println!(
                        "\t(target_rank ({}) != start_rank ({}) - 1",
                        target_rank,
                        start_rank
                    );

                    return false;
                }

                println!("\tTarget is the space ahead of start.");

            }
            
        // For a space in an adjacent file:
        } else {

            println!("\tTARGET IS IN AN ADJACENT FILE TO START.");

            // Ensure target contains a piece
            if chess_board.borrow_space_contents(target_rank, target_file).is_empty() {

                println!("\tOH NO! Target does not contain a piece.");

                return false;
            }

            println!("Target contains a piece.");

            // Ensure target is in the rank ahead of start
            if target_rank != start_rank + 1 {

                println!("\tOH NO! Target is not in the rank ahead of start.");

                return false;
            }

            println!("\tTarget is in the rank ahead of start.");

        }

    }

    println!("PAWN MOVE PASSED ALL CHECKS.");

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
