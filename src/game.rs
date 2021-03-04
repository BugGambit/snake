use piston_window::{keyboard, Key};
use rand::Rng;

use crate::{direction::Direction, point::Point, snake::Snake};

#[derive(PartialEq)]
pub enum Stage {
    Playing,
    GameOver,
}

const INITIAL_SNAKE_LENGTH: u32 = 3;
pub struct GameState {
    stage: Stage,
    snake: Snake,
    fruit: Point,
    board_size: [i32; 2],
}

impl GameState {
    pub fn new(board_size: [i32; 2]) -> Self {
        GameState::init_state(board_size)
    }

    pub fn get_stage(&self) -> &Stage {
        &self.stage
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_fruit_position(&self) -> Point {
        self.fruit
    }

    pub fn get_score(&self) -> i32 {
        (self.snake.len() as i32 - INITIAL_SNAKE_LENGTH as i32) * 10
    }

    pub fn handle_key_down(&mut self, key: keyboard::Key) {
        match key {
            Key::Return => {
                if self.stage != Stage::GameOver {
                    return;
                }
                let state = GameState::init_state(self.board_size);
                self.stage = state.stage;
                self.snake = state.snake;
                self.fruit = state.fruit;
            }
            Key::A | Key::Left => self.safely_change_snake_direction(Direction::WEST),
            Key::S | Key::Down => self.safely_change_snake_direction(Direction::SOUTH),
            Key::D | Key::Right => self.safely_change_snake_direction(Direction::EAST),
            Key::W | Key::Up => self.safely_change_snake_direction(Direction::NORTH),
            _ => (),
        }
    }

    pub fn iterate(&mut self) {
        if self.stage == Stage::GameOver {
            return;
        }

        self.snake.move_snake();

        if self.snake.has_head_collided_into_body() {
            self.stage = Stage::GameOver;
            return;
        }

        if self.snake.get_head_position() == self.fruit {
            self.snake.increase_blocks_to_grow(3);
            self.fruit = GameState::get_new_fruit_position(self.board_size);
        }
    }

    fn init_state(board_size: [i32; 2]) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, board_size[0]);
        let y = rng.gen_range(0, board_size[1]);
        let mut snake = Snake::new(x, y, board_size);
        snake.increase_blocks_to_grow(INITIAL_SNAKE_LENGTH - 1);
        for _ in 0..INITIAL_SNAKE_LENGTH - 1 {
            snake.move_snake();
        }
        let mut state = Self {
            snake,
            stage: Stage::Playing,
            fruit: Point { x: 0, y: 0 },
            board_size,
        };
        state.fruit = GameState::get_new_fruit_position(board_size);
        state
    }

    fn get_new_fruit_position(board_size: [i32; 2]) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0, board_size[0]),
            y: rng.gen_range(0, board_size[1]),
        }
    }

    pub fn safely_change_snake_direction(&mut self, new_direction: Direction) {
        if self.snake.get_direction().negate() == new_direction {
            return;
        }
        self.snake.set_direction(new_direction);
    }
}
