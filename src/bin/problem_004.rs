use itertools::iproduct;
use project_euler_rs::number_misc::is_palindromic;

fn main() {
	let ans = iproduct!(100u64..1000u64, 100u64..1000u64)
		.filter(|(x, y)| x <= y)
		.map(|(x, y)| x * y)
		.filter(|z| is_palindromic(*z))
		.max()
		.unwrap();
	println!("{}", ans);
}
