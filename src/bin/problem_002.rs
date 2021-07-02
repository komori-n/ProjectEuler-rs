use project_euler_rs::fibonacci::Fibonacci;

fn main() {
	let ans = Fibonacci::new(0, 1)
		.take_while(|x| *x < 4_000_000)
		.filter(|x| *x % 2 == 0)
		.sum::<u64>();

	println!("{}", ans);
}