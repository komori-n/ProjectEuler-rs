use num::BigInt;
use project_euler_rs::fibonacci::Fibonacci;

fn main() {
	let fib = Fibonacci::new(BigInt::from(1), BigInt::from(1));

	let ans = fib
		.enumerate()
		.skip_while(|(i, x)| x.to_string().len() < 1000)
		.next()
		.unwrap();

	println!("{:?}", ans.0 + 2);
}
