use project_euler_rs::collatz::Collatz;

fn main() {
	// todo: memorize the process of the calculation
	let ans = (1..1_000_000)
		.max_by_key(|x| Collatz::new(*x).count())
		.unwrap();

	println!("{}", ans);
}
