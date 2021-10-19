mod utils;

use std::thread;
use std::time::Duration;
use console::Term;
use nalgebra::Vector3;
use win_mem::process::Process;
use win_mem::utils::WinResult;
use utils::*;

const CANNON: Vector3<f32> = Vector3::new(
	-66000f32,
	0f32,
	0f32
);

fn main() {
	let term = Term::stdout();
	term.hide_cursor().expect("");
	term.set_title("ExoTracker v1.00");

	loop {
		term.clearprint("searching for game process...");
		let process = loop {
			if let Ok(process) = Process::find("EXO ONE.exe") {
				break process
			}
			thread::sleep(Duration::from_millis(1000));
		};

		term.clearprint("in menu");//desktop->menu
		let mut in_menu = true;
		while let Ok(unity_player) = process.find_module("UnityPlayer.dll") {
			while let Ok(_) = read_and_print(&term, &process, unity_player.address()) {
				in_menu = false;
				thread::sleep(Duration::from_millis(20));
			};
			if in_menu {
				thread::sleep(Duration::from_millis(20));
			} else {
				term.clearprint("in menu");//ingame -> menu
				in_menu = true;
			}
		}
	}
}

fn read_and_print(term: &Term, process: &Process, unity_player: usize) -> WinResult<()> {
	let address = resolve_multilevel_pointer(
		&process,
		unity_player + 0x0156C900,
		&[0x3F8, 0x1A8, 0x28, 0xA0]
	)?;

	let position = process.read_mem::<Vector3<f32>>(address + 0x00)?;
	let velocity = process.read_mem::<Vector3<f32>>(address + 0x30)?;

	let distance = CANNON - position;

	term.move_cursor_to(0, 0).expect("");
	println!("speed vertical   {}     ", velocity.y                as i32);
	println!("speed horizontal {}     ", velocity.xz().magnitude() as i32);
	println!("speed total      {}     ", velocity.magnitude()      as i32);
	println!();
	println!("distance to go   {}     ", distance.magnitude()      as i32 - 7500);

	Ok(())
}