use piston_window::*;
use rand::{thread_rng, Rng};

use snake::{Snake, Direction};
use draw::{draw_block, draw_rect};
use colors::{FOOD_COLOR, BORDER_COLOR, GAMEOVER_COLOR};

const MOVE_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
  snake: Snake,
  food_exists: bool,
  food_x: i32,
  food_y: i32,
  width: i32,
  height: i32,
  game_over: bool,
  waiting_time: f64,
}

// static
impl Game {
  pub fn new(width: i32, height: i32) -> Game {
    Game {
      snake: Snake::new(2, 2),
      waiting_time: 0.0,
      food_exists: true,
      food_x: 6,
      food_y: 10,
      width,
      height,
      game_over: false,
    }
  }
}

// methods
impl Game {
  pub fn draw(&self, ctx: &Context, g: &mut G2d) {
    self.snake.draw(ctx, g);

    if self.food_exists {
      draw_block(FOOD_COLOR, self.food_x, self.food_y, ctx, g);
    }

    draw_rect(BORDER_COLOR, 0, 0, self.width, 1, ctx, g);
    draw_rect(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
    draw_rect(BORDER_COLOR, 0, 0, 1, self.height, ctx, g);
    draw_rect(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);

    if self.game_over {
      draw_rect(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g);
    }
  }

  pub fn update(&mut self, delta_time: f64) {
    self.waiting_time += delta_time;

    if self.game_over {
      if self.waiting_time > RESTART_TIME {
        self.restart();
      }
      return;
    }

    if !self.food_exists {
      self.add_food();
    }

    if self.waiting_time > MOVE_PERIOD {
      self.update_snake(None);
    }
  }

  pub fn key_pressed(&mut self, key: Key) {
    if self.game_over {
      return;
    }

    let dir = match key {
      Key::Up => Some(Direction::Up),
      Key::Down => Some(Direction::Down),
      Key::Left => Some(Direction::Left),
      Key::Right => Some(Direction::Right),
      _ => None,
    };

    if dir.unwrap() == self.snake.head_dir().opposite() {
      return;
    }

    self.update_snake(dir);
  }

  fn check_eating(&mut self) {
    let (head_x, head_y): (i32, i32) = self.snake.head_pos();
    if self.food_exists && self.food_x == head_x && self.food_y == head_y {
      self.food_exists = false;
      self.snake.tail_restore();
    }
  }

  fn add_food(&mut self) {
    let mut rng = thread_rng();

    let mut new_x = rng.gen_range(1, self.width - 1);
    let mut new_y = rng.gen_range(1, self.height - 1);
    while self.snake.tail_overlaps(new_x, new_y) {
      new_x = rng.gen_range(1, self.width - 1);
      new_y = rng.gen_range(1, self.height - 1);
    }

    self.food_x = new_x;
    self.food_y = new_y;
    self.food_exists = true;
  }

  fn update_snake(&mut self, dir: Option<Direction>) {
    if self.snake.check_if_alive(dir, self.width, self.height) {
      self.snake.move_forward(dir);
      self.check_eating();
    } else {
      self.game_over = true;
    }
    self.waiting_time = 0.0;
  }

  fn restart(&mut self) {
    self.snake = Snake::new(2, 2);
    self.waiting_time = 0.0;
    self.food_exists = true;
    self.food_x = 6;
    self.food_y = 4;
    self.game_over = false;
  }
}
