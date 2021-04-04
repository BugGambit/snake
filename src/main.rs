extern crate piston_window;
mod direction;
mod game;
mod graphics;
mod point;
mod snake;
mod tiles;

use character::CharacterCache;
use game::{GameState, Stage};
use graphics::{
  draw_snake, get_asset_path, BACKGROUND_COLOR, BLOCK_SIZE_IN_PX, BOARD_SIZE,
  GAME_OVER_OVERLAY_COLOR, SECONDS_PER_FRAME, TEXT_COLOR, WINDOW_SIZE,
};
use piston_window::*;
use tiles::Tiles;

fn main() {
  let mut game_state = GameState::new(BOARD_SIZE);

  let mut window: PistonWindow = WindowSettings::new("BugGambit - Snake", WINDOW_SIZE)
    .resizable(false)
    .build()
    .unwrap();
  let mut glyphs = window
    .load_font(get_asset_path().join("zorque.ttf"))
    .unwrap();
  let tiles = Tiles::new(
    get_asset_path().join("snake-graphics.png"),
    [5, 4],
    &mut window.create_texture_context(),
  );

  let mut delta_time = 0.0;

  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      game_state.handle_key_down(key);
    }

    window.draw_2d(&event, |c, g, device| {
      clear(BACKGROUND_COLOR, g);
      text::Text::new_color(TEXT_COLOR, 20)
        .draw(
          &game_state.get_score().to_string().as_ref(),
          &mut glyphs,
          &c.draw_state,
          c.transform.trans(0., 20.),
          g,
        )
        .unwrap();
      draw_snake(game_state.get_snake(), c.transform, g);
      let fruit_pos = game_state.get_fruit_position();
      tiles.draw_tile(
        [0, 3],
        [
          fruit_pos.x * BLOCK_SIZE_IN_PX as i32,
          fruit_pos.y * BLOCK_SIZE_IN_PX as i32,
          BLOCK_SIZE_IN_PX as i32,
          BLOCK_SIZE_IN_PX as i32,
        ],
        c,
        g,
      );
      if *game_state.get_stage() == Stage::GameOver {
        rectangle(
          GAME_OVER_OVERLAY_COLOR,
          [0., 0., WINDOW_SIZE[0] as f64, WINDOW_SIZE[1] as f64],
          c.transform,
          g,
        );

        let text = "Game Over - ENTER to restart";
        let font_size = 20;
        let width = glyphs.width(font_size, text).unwrap();
        text::Text::new_color(TEXT_COLOR, font_size)
          .draw(
            text,
            &mut glyphs,
            &c.draw_state,
            c.transform.trans(
              WINDOW_SIZE[0] as f64 / 2. - width / 2.,
              WINDOW_SIZE[0] as f64 / 2. + font_size as f64 / 2.,
            ),
            g,
          )
          .unwrap();
      }
      // Update glyphs before rendering.
      glyphs.factory.encoder.flush(device);
    });

    event.update(|arg| {
      delta_time += arg.dt;
      if delta_time >= SECONDS_PER_FRAME {
        delta_time = 0.;
        game_state.iterate();
      }
    });
  }
}
