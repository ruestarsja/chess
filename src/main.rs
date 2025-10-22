mod components;
mod rules;
mod utils;


use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use ggez::conf;
use ggez::event;
use ggez::graphics;
// use text_io::read;

use crate::components::chess_board::ChessBoard;
use crate::utils::logs::init_logs;
use crate::utils::logs::log;


fn main() {
    // create context and event loop
    let ctx_and_event_loop_result = ContextBuilder::new("chess", "Rue Starsja")
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true)
        )
        .build();
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
    screen_coords: graphics::Rect,
}


impl Game {

    pub fn construct(_ctx: &mut Context) -> Self {
        init_logs();

        // load images, music, etc.

        Self {
            chess_board: Default::default(),
            is_black_turn: false,
            screen_coords: graphics::Rect::new(
                0., 0., _ctx.gfx.drawable_size().0, _ctx.gfx.drawable_size().1
            ),
        }
    }

}


impl event::EventHandler for Game {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // println!();
        // println!("{}", self.chess_board);
        // println!();
        // print!(" > ");
        
        // let input: String = read!("{}\n");

        // if input == "exit" {
        //     println!();
        //     println!("Goodbye!");
        //     println!();
        //     _ctx.request_quit();

        // } else if input == "move" {
        //     println!();
        //     print!(" Starting File Label: ");
        //     let start_file_label: String = read!("{}\n");
        //     print!(" Starting Rank Label: ");
        //     let start_rank_label: String = read!("{}\n");
        //     print!(" Target File Label: ");
        //     let target_file_label: String = read!("{}\n");
        //     print!(" Target Rank Label: ");
        //     let target_rank_label: String = read!("{}\n");
        //     let success = self.chess_board.move_piece(
        //         self.is_black_turn,
        //         ChessBoard::get_rank(start_rank_label),
        //         ChessBoard::get_file(start_file_label),
        //         ChessBoard::get_rank(target_rank_label),
        //         ChessBoard::get_file(target_file_label)
        //     );
        //     if success {
        //         self.is_black_turn = !self.is_black_turn;
        //     }
        // }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // log("INFO", "Starting new draw.");
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(50, 45, 40));
        canvas.set_screen_coordinates(self.screen_coords);

        let board_side_len = if self.screen_coords.w < self.screen_coords.h { self.screen_coords.w } else { self.screen_coords.h };
        let margin = (self.screen_coords.w - self.screen_coords.h).abs() / 2.0;
        let board_x = if self.screen_coords.w < self.screen_coords.h { 0.0 } else { margin };
        let board_y = if self.screen_coords.w < self.screen_coords.h { margin } else { 0.0 };

        let mut meshes: Vec<graphics::Mesh> = vec![];
        
        meshes.push(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    board_x,
                    board_y,
                    board_side_len,
                    board_side_len
                ),
                graphics::Color::from_rgb(80, 60, 20)
            )?
        );

        meshes.push(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    board_x + 20.0,
                    board_y + 20.0,
                    board_side_len - 40.0,
                    board_side_len - 40.0
                ),
                graphics::Color::from_rgb(140, 110, 65)
            )?
        );

        let mut row = 0;
        while row < 8 {
            let mut col = 0;
            while col < 8 {
                if (row + col) % 2 == 0 {
                    meshes.push(
                        graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(
                                (board_x + 20.0) + (col as f32 * ((board_side_len - 40.0) / 8.0)),
                                (board_y + 20.0) + (row as f32 * ((board_side_len - 40.0) / 8.0)),
                                (board_side_len - 40.0) / 8.0,
                                (board_side_len - 40.0) / 8.0
                            ),
                            graphics::Color::from_rgb(200, 180, 160)
                        )?
                    );
                    // log(
                    //     "INFO",
                    //     format!(
                    //         "Added light square at row {}, col {}: ( x:{}, y:{}, w:{}, h:{} ).",
                    //         row,
                    //         col,
                    //         (board_x + 20.0) + (col as f32 * ((board_side_len - 40.0) / 8.0)),
                    //         (board_y + 20.0) + (row as f32 * ((board_side_len - 40.0) / 8.0)),
                    //         (board_side_len - 40.0) / 8.0,
                    //         (board_side_len - 40.0) / 8.0
                    //     )
                    // );
                }
                col += 1;
            }
            row += 1;
            // log("INFO", format!("Finished row, is now on row {}", row));
        }

        for mesh in meshes {
            canvas.draw(&mesh, graphics::DrawParam::default());
        }

        canvas.finish(ctx)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> GameResult {
        self.screen_coords = graphics::Rect::new(
            0., 0., _width, _height
        );
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, ggez::GameError> {
        log("INFO", "The program will exit now.");
        // Ok(true) = keep running, Ok(false) = exit
        Ok(false)
    }

}

