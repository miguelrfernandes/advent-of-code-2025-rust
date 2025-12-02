use std::env;
use std::fs;

fn shift_dial(dial: i32, shift_distance: i32, dial_range: i32) -> i32 {
	(dial + shift_distance).rem_euclid(dial_range)
}

fn get_first_char(s: &str) -> Option<char> {
	s.chars().next()
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let file_path = &args[1];
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
	
	let dial_range = 100;
	let target_dial = 0;
	let mut dial: i32 = 50;
	let mut counter: i32 = 0;
	
	for line in contents.lines() {
		if let Some(direction) = get_first_char(line) {
			let shift_text: String = line
				.chars()
				.skip(1)
				.collect();
			let shift: i32 = shift_text.parse().expect("Failed to parse shift value");
			dial = match direction {
				'L' => shift_dial(dial, -shift, dial_range),	
				'R' => shift_dial(dial, shift, dial_range),
				_ => {
					eprintln!("Unknown direction: {direction}");
					continue;
				}
			};
			if dial == target_dial {
				counter += 1;
			}
		}
	}
	println!("The answer is {counter}!")
}
