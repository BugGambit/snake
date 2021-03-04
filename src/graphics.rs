use std::{fs::canonicalize, path::PathBuf};

use piston_window::{types::Color, *};

use crate::{point::Point, snake::Snake};

pub const BLOCK_SIZE_IN_PX: u32 = 25;
pub const BOARD_SIZE: [i32; 2] = [25, 25];

pub const WINDOW_SIZE: [u32; 2] = [
    BOARD_SIZE[0] as u32 * BLOCK_SIZE_IN_PX,
    BOARD_SIZE[1] as u32 * BLOCK_SIZE_IN_PX,
];

pub const FPS: i32 = 30;
pub const SECONDS_PER_FRAME: f64 = 1.0 / FPS as f64;

pub const BACKGROUND_COLOR: Color = [0., 0., 0., 1.0];
pub const TEXT_COLOR: Color = [1., 1., 1., 1.];
pub const GAME_OVER_OVERLAY_COLOR: Color = [1., 0., 0., 0.25];

// pub fn draw(game_state: &GameState, window: PistonWindow, event: &Event) {
//     window.draw_2d(event, |c, g, device| {
//         clear(BACKGROUND_COLOR, g);
//         text::Text::new_color(TEXT_COLOR, 20)
//             .draw(
//                 &game_state.get_score().to_string().as_ref(),
//                 &mut glyphs,
//                 &c.draw_state,
//                 c.transform.trans(0., 20.),
//                 g,
//             )
//             .unwrap();
//         draw_snake(game_state.get_snake(), c.transform, g);
//         let fruit_pos = game_state.get_fruit_position();
//         tiles.draw_tile(
//             [0, 3],
//             [
//                 fruit_pos.x * BLOCK_SIZE_IN_PX as i32,
//                 fruit_pos.y * BLOCK_SIZE_IN_PX as i32,
//                 BLOCK_SIZE_IN_PX as i32,
//                 BLOCK_SIZE_IN_PX as i32,
//             ],
//             c,
//             g,
//         );
//         if *game_state.get_stage() == Stage::GameOver {
//             rectangle(
//                 GAME_OVER_OVERLAY_COLOR,
//                 [0., 0., WINDOW_SIZE[0] as f64, WINDOW_SIZE[1] as f64],
//                 c.transform,
//                 g,
//             );

//             let text = "Game Over - ENTER to restart";
//             let font_size = 20;
//             let width = glyphs.width(font_size, text).unwrap();
//             text::Text::new_color(TEXT_COLOR, font_size)
//                 .draw(
//                     text,
//                     &mut glyphs,
//                     &c.draw_state,
//                     c.transform.trans(
//                         WINDOW_SIZE[0] as f64 / 2. - width / 2.,
//                         WINDOW_SIZE[0] as f64 / 2. + font_size as f64 / 2.,
//                     ),
//                     g,
//                 )
//                 .unwrap();
//         }
//         // Update glyphs before rendering.
//         glyphs.factory.encoder.flush(device);
//     });
// }

pub fn draw_cell(point: Point, color: Color, transform: math::Matrix2d, g: &mut G2d) {
    let x = point.x as f64;
    let y = point.y as f64;
    let rect = [
        x * BLOCK_SIZE_IN_PX as f64,
        y * BLOCK_SIZE_IN_PX as f64,
        BLOCK_SIZE_IN_PX as f64,
        BLOCK_SIZE_IN_PX as f64,
    ];
    rectangle(color, rect, transform, g);
}

pub fn draw_snake(snake: &Snake, transform: math::Matrix2d, g: &mut G2d) {
    for block in snake.get_snake_body_iter() {
        draw_cell(*block, [0., 1., 0., 1.], transform, g);
    }
}

pub fn get_asset_path() -> PathBuf {
    canonicalize("./assets/").unwrap()
}
