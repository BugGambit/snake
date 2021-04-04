use std::{fs::canonicalize, path::PathBuf};

use piston_window::{types::Color, *};

use crate::{point::Point, snake::Snake};

pub const BLOCK_SIZE_IN_PX: u32 = 25;
pub const BOARD_SIZE: [i32; 2] = [25, 25];

pub const WINDOW_SIZE: [u32; 2] = [
  BOARD_SIZE[0] as u32 * BLOCK_SIZE_IN_PX,
  BOARD_SIZE[1] as u32 * BLOCK_SIZE_IN_PX,
];

pub const FPS: i32 = 10;
pub const SECONDS_PER_FRAME: f64 = 1.0 / FPS as f64;

pub const BACKGROUND_COLOR: Color = [0., 0., 0., 1.0];
pub const TEXT_COLOR: Color = [1., 1., 1., 1.];
pub const GAME_OVER_OVERLAY_COLOR: Color = [1., 0., 0., 0.25];

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
