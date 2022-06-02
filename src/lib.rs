use std::collections::VecDeque;

use rand::prelude::*;

pub type Position = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug)]
pub struct SnakeGame {
	width: usize,
	height: usize,
	snake: VecDeque<Position>,
	snake_direction: Direction,
	food: Position,
	game_over: bool
}

fn rand_position(width: usize, height: usize) -> Position {
	let mut rng = thread_rng();
	(rng.gen_range(0..width), rng.gen_range(0..height))
}

fn get_opposite_direction(direction: &Direction) -> Direction {
	match direction {
		Direction::Up => Direction::Down,
		Direction::Down => Direction::Up,
		Direction::Right => Direction::Left,
		Direction::Left => Direction::Right,
	}
}

impl SnakeGame {
	pub fn new(width: usize, height: usize) -> SnakeGame {
		Self {
			width,
			height,
			snake: VecDeque::from([(width / 2, height / 2)]),
			snake_direction: Direction::Left,
			food: rand_position(width, height),
			game_over: false
		}
	}

	pub fn change_direction(&mut self, direction: Direction) {
		// Because you can't start going in the opposite direction you are
		// currently going
		if get_opposite_direction(&self.snake_direction) == direction {
			return;
		}

		self.snake_direction = direction
	}

	fn is_valid_move(&self, (x, y): Position, dx: i32, dy: i32) -> bool {
		let new_x = x as i32 + dx;
		let new_y = y as i32 + dy;
		0 <= new_x && new_x < self.width as i32 && 0 <= new_y && new_y < self.height as i32
	}

	fn get_unoccupied_position(&self) -> Option<Position> {
		let mut rng = thread_rng();
		(0..self.height)
			.flat_map(|y| (0..self.width).map(move |x| (x, y)))
			.filter(|pos| !self.snake.contains(pos))
			.choose(&mut rng)
	}

	pub fn tick(&mut self) {
		if self.game_over {
			return;
		}

		let head = self.snake.front();
		if head.is_none() {
			return;
		}
		let head = *head.unwrap();

		let (dx, dy): (i32, i32) = match &self.snake_direction {
			Direction::Up => (-1, 0),
			Direction::Down => (1, 0),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		};

		if !self.is_valid_move(head, dx, dy) {
			self.game_over = true;
			return;
		}

		let new_head = (
			(head.0 as i32 + dx) as usize,
			(head.1 as i32 + dy) as usize
		);
		self.snake.push_front(new_head);

		if new_head == self.food {
			let new_food = self.get_unoccupied_position();
			if let Some(new_food) = new_food {
				self.food = new_food;
			} else {
				self.game_over = true;
			}
		} else {
			self.snake.pop_back();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let game = SnakeGame::new(10, 10);

		println!("{:?}", game);
	}
}
