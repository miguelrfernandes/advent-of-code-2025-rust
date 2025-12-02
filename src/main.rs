use std::env;
use std::fs;

fn shift_dial(dial: i32, shift_distance: i32, dial_range: i32) -> i32 {
	((dial + shift_distance) % dial_range + dial_range) % dial_range
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let file_path = &args[1];
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
	
	let lines = contents.lines();
	
	fn get_first_char(s: &str) -> Option<char> {
		s.chars().next()
	}

	let dial_range = 100;
	let mut dial: i32 = 50;
	let target_dial = 0;
	let mut counter: i32 = 0;
	for line in lines {
		if let Some(direction) = get_first_char(line) {
			let shift_text: String = line
				.chars()
				.skip(1)
				.collect();
			let shift: i32 = shift_text.parse().expect("Failed to parse shift value");
			if direction == 'L' {
				dial = shift_dial(dial, -shift, dial_range);	
			} else if direction == 'R' {
				dial = shift_dial(dial, shift, dial_range);
			}
			if dial == target_dial {
				counter = counter + 1;
			}
		}
	}
	println!("The answer is {counter}!")
}
