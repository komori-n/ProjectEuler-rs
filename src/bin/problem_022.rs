use anyhow::Result;
use std::fs;

const INPUT_FILENAME: &'static str = "resources/p022_names.txt";

fn main() -> Result<()> {
	let mut input = parse_input()?;
	input.sort();
	let ans = input
		.iter()
		.enumerate()
		.map(|(i, s)| ((i + 1) as u32) * score(s))
		.sum::<u32>();

	println!("{}", ans);

	Ok(())
}

fn parse_input() -> Result<Vec<String>> {
	let input = fs::read_to_string(INPUT_FILENAME)?;
	let vec = input
		.split(',')
		.map(|name| {
			name.strip_prefix('"')
				.unwrap()
				.strip_suffix('"')
				.unwrap()
				.to_owned()
		})
		.collect();

	Ok(vec)
}

fn score(s: &str) -> u32 {
	s.chars()
		.map(|c| (c as u32) - ('A' as u32) + 1)
		.sum::<u32>()
}
