fn main() {
	let coins = [1, 2, 5, 10, 20, 50, 100, 200];
	let mut dp = vec![vec![0; 201]; coins.len() + 1];

	dp[0][0] = 1;
	for (i, c) in coins.iter().enumerate() {
		for j in 0..201 {
			if dp[i][j] > 0 {
				for k in 0.. {
					let l = j + k * c;

					if l > 200 {
						break;
					}

					dp[i + 1][l] += dp[i][j];
				}
			}
		}
	}

	println!("{}", dp[coins.len()][200]);
}
