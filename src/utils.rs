use console::Term;
use win_mem::process::Process;
use win_mem::utils::WinResult;

pub trait TermEx {
	fn clearprint(&self, text: &str) -> ();
}

impl TermEx for Term {
	fn clearprint(&self, text: &str) -> () {
		self.clear_screen().expect("screen clear failed");
		println!("{}", text);
	}
}

pub fn resolve_multilevel_pointer(process: &Process, base: usize, offsets: &[usize]) -> WinResult<usize> {
	let mut address = base;

	for offset in offsets {
		address = process.read_mem::<usize>(address)? + offset;
	}
	Ok(address)
}

// recursive approach for novelty

// pub fn resolve_multilevel_pointer2(process: &Process, base: usize, offsets: &[usize]) -> WinResult<usize> {
// 	if let [first_offset, remaining_offsets @ ..] = offsets {
// 		resolve_multilevel_pointer2(
// 			process,
// 			process.read_mem::<usize>(base)? + first_offset,
// 			remaining_offsets
// 		)
// 	} else { Ok(base) }
// }