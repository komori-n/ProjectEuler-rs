use project_euler_rs::number_misc::to_letters;

fn main() {
	let ans = (1..1000)
		.map(to_letters)
		.map(|s| s.chars().map(|c| c.is_alphabetic() as u64).sum::<u64>())
		.sum::<u64>();

	println!("{}", ans + 11);
}
