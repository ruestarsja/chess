mod components;


use components::chess_board::ChessBoard;


fn main() {
    
    let chess_board = ChessBoard::default();

    let mut running: bool = true;
    while running {

        println!("{}", chess_board);
        println!();
        println!(" > ");
        running = false;

    }

}
