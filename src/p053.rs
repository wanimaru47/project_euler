use std::collections::HashMap;

struct Combination {
    memo: HashMap<(i64, i64), i64>,
}

impl Combination {
    fn new() -> Combination {
        return Combination {
            memo: HashMap::new(),
        };
    }

    fn combination(&mut self, n: i64, r: i64, m: i64) -> i64 {
        if let Some(res) = self.memo.get(&(n, r)) {
            return res.clone();
        }

        let x = if r == 1 {
            n % m
        } else if n == r {
            1
        } else {
            self.combination(n - 1, r, m) + self.combination(n - 1, r - 1, m) % m
        };

        self.memo.insert((n, r), x);

        return x;
    }
}

fn main() {
    let mut cnt = 0;
    let mut comb = Combination::new();
    for i in 1..=100 {
        for j in 1..=i {
            if comb.combination(i, j, 10000000) > 1000000 {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
