use project_euler_rs::prime::Abundants;

fn main() {
	let mut representable = vec![false; 30000];
	let max = (representable.len() - 1) as u64;

	for i in Abundants::new().take_while(|&i| i < max) {
		for j in Abundants::new()
			.skip_while(|&j| j < i)
			.take_while(|&j| i + j <= max)
		{
			representable[(i + j) as usize] = true;
		}
	}

	let ans = representable
		.iter()
		.enumerate()
		.filter_map(|(i, b)| if !b { Some(i) } else { None })
		.sum::<usize>();

	println!("{}", ans);
}
