mod components;
mod rules;
mod utils;


use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use ggez::event;
use ggez::graphics;
use text_io::read;

use crate::components::chess_board::ChessBoard;
use crate::utils::logs::init_logs;
use crate::utils::logs::log;


fn main() {
    // create context and event loop
    let ctx_and_event_loop_result = ContextBuilder::new("chess", "Rue Starsja").build();
    let (mut ctx, event_loop) = match ctx_and_event_loop_result {
        Ok(_) => {
            ctx_and_event_loop_result.unwrap()
        },
        Err(error) => {
            log(
                "ERROR",
                format!(
                    "ggez::ContextBuilder::build returned Err(...): {}",
                    error
                )
            );
            panic!("Failed to build Context.");
        }
    };
    // create game
    let game = Game::construct(&mut ctx);
    // run game: note that system will exit from inside this function call
    event::run(ctx, event_loop, game);
    // code from here on would be unreachable
}


struct Game {
    chess_board: ChessBoard,
    is_black_turn: bool,
}


impl Game {

    pub fn construct(_ctx: &mut Context) -> Self {
        init_logs();

        // load images, music, etc.

        Self {
            chess_board: Default::default(),
            is_black_turn: false,
        }
    }

}


impl event::EventHandler for Game {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        println!();
        println!("{}", self.chess_board);
        println!();
        print!(" > ");
        
        let input: String = read!("{}\n");

        if input == "exit" {
            println!();
            println!("Goodbye!");
            println!();
            _ctx.request_quit();

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
            let success = self.chess_board.move_piece(
                self.is_black_turn,
                ChessBoard::get_rank(start_rank_label),
                ChessBoard::get_file(start_file_label),
                ChessBoard::get_rank(target_rank_label),
                ChessBoard::get_file(target_file_label)
            );
            if success {
                self.is_black_turn = !self.is_black_turn;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        // ...
        canvas.finish(ctx)
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, ggez::GameError> {
        log("INFO", "The program should exit now.");
        // Ok(true) = keep running, Ok(false) = exit
        Ok(false)
    }

}

