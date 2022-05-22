use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use nalgebra::Vector3;
use win_mem::process::Process;
use win_mem::utils::WinResult;
use crate::resolve_multilevel_pointer;
use crate::model::{Model, Snapshot, Status};

pub fn begin(model_arc: Arc<Mutex<Model>>) {
	loop {
		{ model_arc.lock().unwrap().status = Status::NoGame; }

		let process = loop {
			if let Ok(process) = Process::find("EXO ONE.exe") {
				break process
			}
			thread::sleep(Duration::from_millis(1000));
		};

		{ model_arc.lock().unwrap().status = Status::Menu; }//desktop->menu

		let mut in_menu = true;
		while let Ok(unity_player) = process.find_module("UnityPlayer.dll") {

			while read_and_log(&model_arc, &process, unity_player.address()).is_ok() {
				in_menu = false;
				thread::sleep(Duration::from_millis(100));
			};

			if !in_menu {
				{ model_arc.lock().unwrap().status = Status::Menu; }//ingame -> menu
				in_menu = true;
			}
			thread::sleep(Duration::from_millis(100));
		}
	}
}

fn read_and_log(model_arc: &Arc<Mutex<Model>>, process: &Process, unity_player: usize) -> WinResult<()> {
	let address = resolve_multilevel_pointer(
		process,
		unity_player + 0x01A03D00,//0x0156C900
		&[0x58, 0x158, 0x28, 0xA0]//0x3F8, 0x1A8, 0x28, 0xA0
	)?;

	//let position = process.read_mem::<Vector3<f32>>(address + 0x00)?;
	let velocity = process.read_mem::<Vector3<f32>>(address + 0x30)?;

	let now = Instant::now();
	let mut model = model_arc.lock().unwrap();
	model.status = Status::Active;
	while let Some(front) = model.snapshots.front() {
		if (now - front.timestamp).as_millis() < 60000 {
			break
		}
		model.snapshots.pop_front().expect("impossible");
	}
	model.snapshots.push_back(Snapshot {
		velocity,
		timestamp: now
	});

	Ok(())
}