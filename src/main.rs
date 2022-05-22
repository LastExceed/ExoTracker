mod utils;
mod data_provider;
mod model;
mod view;

use utils::*;
use crate::model::create_model;

fn main() {
	nannou::app(create_model).run();
}