use project_euler_rs::number_misc::factorial;

fn main() {
	let ans = (10..10_000_000)
		.filter(|x| {
			x.to_string()
				.chars()
				.map(|c| factorial(c.to_digit(10).unwrap()))
				.sum::<u32>() == *x as u32
		})
		.sum::<i32>();

	println!("{}", ans);
}
