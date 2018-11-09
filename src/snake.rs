use std::collections::LinkedList;
use piston_window::{Context, G2d};

use draw::draw_block;
use colors::SNAKE_COLOR;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn opposite(&self) -> Direction {
    match *self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

#[derive(Debug, Clone)]
struct Block {
  x: i32,
  y: i32,
}

pub struct Snake {
  direction: Direction,
  body: LinkedList<Block>,
  tail: Option<Block>,
}

// static
impl Snake {
  pub fn new(x: i32, y: i32) -> Snake {
    let mut body: LinkedList<Block> = LinkedList::new();

    body.push_back(Block { x: x + 2, y });
    body.push_back(Block { x: x + 1, y });
    body.push_back(Block { x, y });

    Snake {
      direction: Direction::Right,
      body,
      tail: None,
    }
  }
}

// methods
impl Snake {
  pub fn draw(&self, ctx: &Context, g: &mut G2d) {
    for block in &self.body {
      draw_block(SNAKE_COLOR, block.x, block.y, ctx, g)
    }
  }

  pub fn head_pos(&self) -> (i32, i32) {
    let head_block = self.body.front().unwrap();
    (head_block.x, head_block.y)
  }

  pub fn head_dir(&self) -> Direction {
    self.direction
  }

  pub fn head_next(&self, dir: Option<Direction>) -> (i32, i32) {
    let (head_x, head_y): (i32, i32) = self.head_pos();
    let move_dir = match dir {
      Some(d) => d,
      None => self.direction,
    };

    match move_dir {
      Direction::Up => (head_x, head_y - 1),
      Direction::Down => (head_x, head_y + 1),
      Direction::Left => (head_x - 1, head_y),
      Direction::Right => (head_x + 1, head_y),
    }
  }

  pub fn move_forward(&mut self, dir: Option<Direction>) {
    match dir {
      None => (),
      Some(d) => self.direction = d,
    }

    let (last_x, last_y): (i32, i32) = self.head_pos();
    let removed_block = self.body.pop_back().unwrap();
    let new_block = match self.direction {
      Direction::Up => Block {
        x: last_x,
        y: last_y - 1,
      },
      Direction::Down => Block {
        x: last_x,
        y: last_y + 1,
      },
      Direction::Left => Block {
        x: last_x - 1,
        y: last_y,
      },
      Direction::Right => Block {
        x: last_x + 1,
        y: last_y,
      },
    };
    self.body.push_front(new_block);
    self.tail = Some(removed_block);
  }

  pub fn tail_restore(&mut self) {
    let tail_block = self.tail.clone().unwrap();
    self.body.push_back(tail_block);
  }

  pub fn tail_overlaps(&self, x: i32, y: i32) -> bool {
    let mut ch = 0;
    for block in &self.body {
      if block.x == x && block.y == y {
        return true;
      }
      ch += 1;
      if ch == self.body.len() - 1 {
        break;
      }
    }
    false
  }

  pub fn check_if_alive(
    &self,
    dir: Option<Direction>,
    game_width: i32,
    game_height: i32,
  ) -> bool {
    let (next_x, next_y) = self.head_next(dir);

    if self.tail_overlaps(next_x, next_y) {
      return false;
    }

    next_x > 0
      && next_y > 0
      && next_x < game_width - 1
      && next_y < game_height - 1
  }
}
