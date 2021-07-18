use project_euler_rs::number_misc::{is_palindromic, is_palindromic_bin};

fn main() {
	let ans = (1..1_000_000)
		.filter(|i| is_palindromic(*i) && is_palindromic_bin(*i))
		.sum::<u64>();

	println!("{}", ans);
}
