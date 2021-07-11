use std::cmp::max;

const INPUT: &'static str = r#"
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
"#;

fn main() {
	let input: Vec<_> = INPUT
		.strip_prefix('\n')
		.unwrap()
		.strip_suffix('\n')
		.unwrap()
		.split('\n')
		.map(|row| {
			row.split(' ')
				.map(|num| num.parse::<u64>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect();

	let len = input.len();
	let mut dp = vec![vec![0; len]; len];
	dp[0][0] = input[0][0];

	for i in 1..len {
		for j in 0..i {
			dp[i][j] = max(dp[i][j], dp[i - 1][j] + input[i][j]);
			dp[i][j + 1] = max(dp[i][j + 1], dp[i - 1][j] + input[i][j + 1]);
		}
	}

	let ans = dp[len - 1].iter().max().unwrap();
	println!("{}", ans);
}
