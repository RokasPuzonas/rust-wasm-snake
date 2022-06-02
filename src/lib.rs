mod snake;

use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use snake::Direction;
use snake::SnakeGame;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement, HtmlDivElement, KeyboardEvent};

thread_local! {
	static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(20, 15)));
	static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
		let game = GAME.with(|game| game.clone());
		move || {
			game.borrow_mut().tick();
			render();
		}
	}) as Box<dyn FnMut()>);

	static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new({
		let game = GAME.with(|game| game.clone());
		move |e: KeyboardEvent| {
			let direction = match &e.key()[..] {
				"ArrowUp" => Direction::Up,
				"ArrowDown" => Direction::Down,
				"ArrowRight" => Direction::Right,
				"ArrowLeft" => Direction::Left,
				_ => return
			};

			game.borrow_mut().change_direction(direction);
		}
	}))
}

#[wasm_bindgen(start)]
pub fn main() {
	console::log_1(&"Staring...".into());

	TICK_CLOSURE.with(|tick_closure| {
		window()
			.unwrap_throw()
			.set_interval_with_callback_and_timeout_and_arguments_0(
				tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
				350
			)
			.unwrap_throw()
	});

	HANDLE_KEYDOWN.with(|handle_keydown| {
		window()
			.unwrap_throw()
			.add_event_listener_with_callback(
				"keydown",
				handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
			)
			.unwrap_throw();
	});

	render();
}

pub fn render() {
	GAME.with(|game| {
		let game = game.borrow();
		let document = window()
			.unwrap_throw()
			.document()
			.unwrap_throw();

		let root = document
			.get_element_by_id("root")
			.unwrap_throw()
			.dyn_into::<HtmlElement>()
			.unwrap_throw();

		root.set_inner_html("");
		root.style()
			.set_property("display", "inline-grid")
			.unwrap_throw();
		root.style()
			.set_property("grid-template", &format!("repeat({}, auto) / repeat({}, auto)", game.height, game.width))
			.unwrap_throw();

		for y in 0..game.height {
			for x in 0..game.width {
				let pos = (x, y);
				let elem = document.create_element("div").unwrap_throw().dyn_into::<HtmlDivElement>().unwrap_throw();
				elem.set_class_name("field");

				let text = if pos == game.food {
					"🍎"
				} else if Some(&pos) == game.snake.front() {
					"❇️"
				} else if game.snake.contains(&pos) {
					"🟩"
				} else {
					" "
				};

				elem.set_inner_text(text);

				root.append_child(&elem).unwrap_throw();
			}
		}
	})
}
