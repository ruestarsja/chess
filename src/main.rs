mod components;
mod rules;
mod utils;


use std::collections::HashMap;

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
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        panic!("Couldn't get cargo manifest dir.");
    };

    // create context and event loop
    let ctx_and_event_loop_result = ContextBuilder::new("chess", "Rue Starsja")
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true)
        )
        .window_setup(
            conf::WindowSetup::default()
                .title("Chess")
        )
        .add_resource_path(resource_dir)
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
    sprites: HashMap<(String, String), graphics::Image>,
    selected_space: Option<(u8, u8)>,
}


impl Game {

    pub fn construct(ctx: &mut Context) -> Self {
        init_logs();

        // load images, music, etc.
        let mut sprites: HashMap<(String, String), graphics::Image> = HashMap::new();
        let colors = [
            String::from("black"),
            String::from("white")
        ];
        let types = [
            String::from("bishop"),
            String::from("king"),
            String::from("knight"),
            String::from("pawn"),
            String::from("queen"),
            String::from("rook")
        ];
        for color in &colors {
            for _type in &types {
                sprites.insert(
                    (color.clone(), _type.clone()),
                    graphics::Image::from_path(
                        ctx,
                        format!("/sprites/{}_{}.png", color, _type)
                    ).expect("Could not open sprite.")
                );
            }
        }
        drop(colors);
        drop(types);

        Self {
            chess_board: Default::default(),
            is_black_turn: false,
            screen_coords: graphics::Rect::new(
                0., 0., ctx.gfx.drawable_size().0, ctx.gfx.drawable_size().1
            ),
            sprites: sprites,
            selected_space: None,
        }
    }

}


impl event::EventHandler for Game {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // println!();
        // println!("{}", self.chess_board);
        // println!();
        // print!(" > ");
        
        // let input: String = read!("{}\n");

        // if input == "exit" {
        //     println!();
        //     println!("Goodbye!");
        //     println!();
        //     ctx.request_quit();

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

        let mut rank = 0;
        while rank < 8 {
            let mut file = 0;
            while file < 8 {
                if (rank + file) % 2 == 0 {
                    meshes.push(
                        graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(
                                (board_x + 20.0) + (file as f32 * ((board_side_len - 40.0) / 8.0)),
                                (board_y + 20.0) + (rank as f32 * ((board_side_len - 40.0) / 8.0)),
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
                if self.selected_space == Some((file, rank)) {
                    
                }
                file += 1;
            }
            rank += 1;
            // log("INFO", format!("Finished row, is now on row {}", row));
        }

        for mesh in meshes {
            canvas.draw(&mesh, graphics::DrawParam::default());
        }

        match self.selected_space {
            None => (),
            Some(_) => {
                let selected_space_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(5.0),
                    graphics::Rect::new(
                        (board_x + 20.0) + (self.selected_space.unwrap().0 as f32 * ((board_side_len - 40.0) / 8.0)),
                        (board_y + 20.0) + (self.selected_space.unwrap().1 as f32 * ((board_side_len - 40.0) / 8.0)),
                        (board_side_len - 40.0) / 8.0,
                        (board_side_len - 40.0) / 8.0
                    ),
                    graphics::Color::from_rgb(100, 200, 100)
                )?;
                canvas.draw(
                    &selected_space_mesh,
                    graphics::DrawParam::default()
                );
            }
        }

        rank = 0;
        while rank < 8 {
            let mut file = 0;
            while file < 8 {
                let piece = self.chess_board.borrow_space_contents(rank, file);
                if !piece.is_empty() {
                    let sprite_option = self.sprites.get(&(piece.get_color(), piece.get_type()));
                    let sprite = match sprite_option {
                        Some(_) => sprite_option.unwrap(),
                        None => {
                            log(
                                "ERROR",
                                format!(
                                    "Tried to look up sprite for a {} {}, but got a None response.",
                                    piece.get_color(),
                                    piece.get_type()
                                )
                            );
                            panic!("Couldn't get sprite.");
                        }
                    };
                    canvas.draw(
                        sprite,
                        graphics::DrawParam::default()
                            .dest_rect(graphics::Rect::new(
                                board_x + 20.0 + (file as f32 * ((board_side_len - 40.0) / 8.0)),
                                board_y + 20.0 + (rank as f32 * ((board_side_len - 40.0) / 8.0)),
                                ((board_side_len - 40.0) / 8.0) / sprite.width() as f32,
                                ((board_side_len - 40.0) / 8.0) / sprite.height() as f32
                            ))
                    );
                    // log(
                    //     "INFO",
                    //     format!(
                    //         "Drew sprite for {} {} at rect: x {}, y {}, w {}, h {}",
                    //         piece.get_color(),
                    //         piece.get_type(),
                    //         board_x + 20.0 + (file as f32 * ((board_side_len - 40.0) / 8.0)),
                    //         board_y + 20.0 + (rank as f32 * ((board_side_len - 40.0) / 8.0)),
                    //         ((board_side_len - 40.0) / 8.0) / sprite.width() as f32,
                    //         ((board_side_len - 40.0) / 8.0) / sprite.height() as f32
                    //     )
                    // );
                }
                file += 1;
            }
            rank += 1;
        }

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
            &mut self,
            ctx: &mut Context,
            button: event::MouseButton,
            x: f32,
            y: f32,
        ) -> GameResult {
        log(
            "INFO",
            format!(
                "User clicked at coordinates (x {}, y {}).",
                x, y
            )
        );

        let board_side_len = if self.screen_coords.w < self.screen_coords.h { self.screen_coords.w } else { self.screen_coords.h };
        let margin = (self.screen_coords.w - self.screen_coords.h).abs() / 2.0;
        let board_x = if self.screen_coords.w < self.screen_coords.h { 0.0 } else { margin };
        let board_y = if self.screen_coords.w < self.screen_coords.h { margin } else { 0.0 };

        if x < (board_x + 20.0)
        || x > (board_x + board_side_len - 20.0)
        || y < (board_y + 20.0)
        || y > (board_y + board_side_len - 20.0) {
            log(
                "INFO",
                "User did not click on a space."
            );
            return Ok(());
        } else {
            let file = ((x - (board_x + 20.0)) / ((board_side_len - 40.0) / 8.0)).floor() as u8;
            let rank = ((y - (board_y + 20.0)) / ((board_side_len - 40.0) / 8.0)).floor() as u8;
            log(
                "INFO",
                format!(
                    "User clicked on the space {}{}.",
                    ChessBoard::get_file_label(file),
                    ChessBoard::get_rank_label(rank)
                )
            );
            match self.selected_space {
                None => {
                    log("INFO", "Didn't have a space selected previously.");
                    let piece = self.chess_board.borrow_space_contents(rank, file);
                    log(
                        "INFO",
                        format!(
                            "\n\tpiece is black: {}\n\tpiece is white: {}\n\tis black turn: {}\n\t! is black turn: {}",
                            piece.is_black(), piece.is_white(), self.is_black_turn, !self.is_black_turn
                        )
                    );
                    match (piece.is_black() && self.is_black_turn)
                       || (piece.is_white() && !self.is_black_turn) {
                        true => {
                            log("INFO", "Selected the clicked space.");
                            self.selected_space = Some((file, rank));
                            return Ok(());
                        }
                        false => {
                            log("INFO", "Did not select the clicked space; is not the correct turn.");
                            return Ok(());
                        }
                    }
                },
                Some(space) => {
                    log(
                        "INFO",
                        format!(
                            "Previously had {}{} selected.",
                            ChessBoard::get_file_label(self.selected_space.unwrap().0),
                            ChessBoard::get_rank_label(self.selected_space.unwrap().1)
                        )
                    );
                    if space == (file, rank) {
                        log("INFO", "User clicked the previously selected space.");
                        self.selected_space = None;
                        log("INFO", "Unselected the previously selected space.");
                        return Ok(());
                    } else {
                        log("INFO", "User clicked a space other than the previously selected space.");
                        log("INFO", "Trying to move piece from the previously selected space to the newly clicked space.");
                        if self.chess_board.move_piece(
                            self.is_black_turn,
                            self.selected_space.unwrap().1,
                            self.selected_space.unwrap().0,
                            rank,
                            file
                        ) {
                            log("INFO", "Moved the piece from the previously selected space to the newly clicked space.");
                            self.is_black_turn = !self.is_black_turn;
                            self.selected_space = None;
                            return Ok(());
                        } else {
                            log("INFO", "Could not move the piece from the previously selected space to the newly clicked space.");
                            let piece = self.chess_board.borrow_space_contents(rank, file);
                            match (piece.is_black() && self.is_black_turn)
                               || (piece.is_white() && !self.is_black_turn) {
                                    true => {
                                        log("INFO", "Selected the clicked space.");
                                        self.selected_space = Some((file, rank));
                                        return Ok(());
                                    }
                                    false => {
                                        log("INFO", "Did not select the clicked space; is not the correct turn.");
                                        self.selected_space = None;
                                        return Ok(());
                                    }
                                }
                            log("INFO", "Updated to select the newly clicked space.");
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) -> GameResult {
        self.screen_coords = graphics::Rect::new(
            0., 0., width, height
        );
        Ok(())
    }

    fn quit_event(&mut self, ctx: &mut Context) -> Result<bool, ggez::GameError> {
        log("INFO", "The program will exit now.");
        // Ok(true) = keep running, Ok(false) = exit
        Ok(false)
    }

}

