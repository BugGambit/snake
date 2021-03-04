use std::collections::{linked_list::Iter, LinkedList};

use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    head_to_tail_blocks: LinkedList<Point>,
    blocks_to_grow: u32,
    board_size: [i32; 2],
}

impl Snake {
    pub fn new(x: i32, y: i32, board_size: [i32; 2]) -> Snake {
        let mut head_to_tail_blocks = LinkedList::new();
        let head = Point { x, y };
        head_to_tail_blocks.push_back(head);
        Snake {
            direction: Direction::rand(),
            head_to_tail_blocks,
            blocks_to_grow: 0,
            board_size,
        }
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn len(&self) -> usize {
        self.head_to_tail_blocks.len()
    }

    pub fn get_snake_body_iter(&self) -> Iter<Point> {
        self.head_to_tail_blocks.iter()
    }

    pub fn move_snake(&mut self) {
        let tail_tip = self.get_tail_tip_position();
        let new_head_position =
            self.clamp_point_in_board(self.get_head_position() + self.direction.to_delta_point());
        self.head_to_tail_blocks.push_front(new_head_position);
        self.head_to_tail_blocks.pop_back();
        if self.blocks_to_grow > 0 {
            self.head_to_tail_blocks.push_back(tail_tip);
            self.blocks_to_grow -= 1;
        }
    }

    pub fn increase_blocks_to_grow(&mut self, count: u32) {
        self.blocks_to_grow += count;
    }

    pub fn has_head_collided_into_body(&self) -> bool {
        let mut body_iterator = self.head_to_tail_blocks.iter();
        let head = body_iterator.next().unwrap();
        for body_block in body_iterator {
            if *head == *body_block {
                return true;
            }
        }
        false
    }

    fn clamp_point_in_board(&self, point: Point) -> Point {
        fn loop_number(num: i32, max_number: i32) -> i32 {
            (num + max_number) % max_number
        }
        Point {
            x: loop_number(point.x, self.board_size[0]),
            y: loop_number(point.y, self.board_size[1]),
        }
    }

    pub fn get_head_position(&self) -> Point {
        *self.head_to_tail_blocks.front().unwrap()
    }

    fn get_tail_tip_position(&self) -> Point {
        *self.head_to_tail_blocks.back().unwrap()
    }
}
