use std::io::Write as _;

fn generate(
	global_time: u32,
	influences_pitch_somehow: u32,
	this_melody_time: u32,
	pitch_lowering: u32,
) -> u8 {
	let slow_time = global_time >> 16;
	let data = if slow_time % 4 > 0 {
		[117, 140, 176, 105, 140, 117, 105, 87]
	} else {
		[132, 157, 176, 105, 157, 132, 105, 88]
	};
	let current_data = data[(this_melody_time % 8) as usize];
	let something = global_time * current_data;
	let ret = (0b11 & influences_pitch_somehow & (something >> pitch_lowering)) << 4;
	ret as u8
}

fn main() {
	let mut fast_time = 0;
	let mut stdout = std::io::stdout().lock();
	loop {
		let medium_time = fast_time / 16384;
		let slow_time = fast_time / 131072;

		let first_melody = generate(fast_time, 1, medium_time, 12);
		let second_melody = generate(fast_time, slow_time, medium_time ^ (fast_time / 8192), 10);
		let third_melody = generate(
			fast_time,
			slow_time / 3,
			medium_time + ((fast_time / 2048) % 3),
			10,
		);
		let fourth_melody = generate(
			fast_time,
			slow_time / 5,
			8 + medium_time - ((fast_time / 1024) % 3),
			9,
		);

		let all_together = first_melody + second_melody + third_melody + fourth_melody;

		let buf = [all_together];
		stdout.write_all(&buf).unwrap();

		fast_time += 1;
	}
}
