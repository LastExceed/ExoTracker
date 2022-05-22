use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use nalgebra::Vector3;
use nannou::prelude::*;
use crate::data_provider;
use crate::view::view;

pub struct Snapshot {
	pub velocity: Vector3<f32>,
	pub timestamp: Instant
}

#[derive(Debug, PartialEq)]
pub enum Status {
	NoGame,
	Menu,
	Active
}

pub struct Model {
	pub status: Status,
	pub snapshots: VecDeque<Snapshot>
}

pub fn create_model(app: &App) -> Arc<Mutex<Model>> {
	app.new_window()
		.title("ExoTracker v2.00")
		.view(view)
		.key_pressed(key_pressed)
		.build().unwrap();

	let model = Model {
		status: Status::NoGame,
		snapshots: VecDeque::new()
	};

	let model_arc = Arc::new(Mutex::new(model));
	let model_arc_clone = Arc::clone(&model_arc);

	thread::spawn(move || {
		data_provider::begin(model_arc_clone);
	});

	model_arc
}

fn key_pressed(app: &App, model: &mut Arc<Mutex<Model>>, key: Key) {
	if key == Key::F11 {
		app.main_window().set_fullscreen(!app.main_window().is_fullscreen());
	}
}