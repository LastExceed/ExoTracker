use std::sync::{Arc, Mutex};
use std::time::Instant;
use nannou::prelude::*;
use crate::model::{Model, Status};

const SCALE_X: f32 = 10f32;
const SCALE_Y: f32 = 2f32;

pub fn view(app: &App, model_arc: &Arc<Mutex<Model>>, frame: Frame) {
	frame.clear(BLACK);
	let draw = app.draw();
	let rect = frame.rect();

	let model = model_arc.lock().unwrap();

	if model.status != Status::Active {
		let msg = format!("{:?}", model.status);
		draw.text(msg.as_str())
			.color(WHITE)
			.x_y(0f32, 0f32)
			.finish();
	} else {
		gridlines(&draw, &rect);

		let now = Instant::now();

		let data = model.snapshots.iter().map(|it| {
			(
				rect.right() - ((now - it.timestamp).as_millis() as f32 / SCALE_X),
				it.velocity.y / SCALE_Y,
				it.velocity.xz().magnitude() / SCALE_Y,
				it.velocity.magnitude() / SCALE_Y,
			)
		});

		draw.text("vertical speed (positive)").color(GREEN).y(rect.top() - 10f32);
		draw.text("vertical speed (negative)").color(RED).y(rect.top() - 20f32);
		draw.polyline().color(GREEN).points(data.clone().map(|it| { pt2(it.0, rect.bottom() + it.1) }));
		draw.polyline().color(RED).points(data.clone().map(|it| { pt2(it.0, rect.bottom() + -it.1) }));

		draw.text("horizontal speed").color(DODGERBLUE).y(rect.top() - 30f32);
		draw.polyline().color(DODGERBLUE).points(data.clone().map(|it| { pt2(it.0, rect.bottom() + it.2) }));

		draw.text("total speed").color(WHITE).y(rect.top() - 40f32);
		draw.polyline().color(WHITE).points(data.clone().map(|it| { pt2(it.0, rect.bottom() + it.3) }));
	}

	draw.to_frame(app, &frame).unwrap();
}

fn gridlines(draw: &Draw, rect: &Rect) {
	let mut line_index = 1;

	loop {
		let value = line_index as f32 * 100f32;
		let y = rect.bottom() + value / SCALE_Y;
		if y > rect.top() { break; }

		draw.line()
			.color(DIMGRAY)
			.start(pt2(rect.x.start, y))
			.end(pt2(rect.x.end, y))
			.finish();

		for x in [rect.x.start + 15f32, rect.x.end - 15f32] {
			draw.text(value.to_string().as_str())
				.color(DIMGRAY)
				.x_y(x, y + 10f32)
				.finish();
		}

		line_index += 1;
	}
}