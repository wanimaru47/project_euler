fn divide(mut n: i64) -> Vec<i64> {
    let mut d = vec![];
    while n > 0 {
        d.push(n % 10);
        n /= 10;
    }

    return d;
}

fn check(values: (i64, i64, i64)) -> bool {
    let mut x = divide(values.0);
    let mut y = divide(values.1);
    let mut z = divide(values.2);
    x.sort();
    y.sort();
    z.sort();

    x == y && y == z
}

fn main() {
    const LIMIT: usize = 10000;
    let mut is_prime = [true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..is_prime.len() {
        if !is_prime[i] {
            continue;
        }

        let mut j = 2;
        while i * j < is_prime.len() {
            is_prime[i * j] = false;
            j += 1;
        }
    }

    let mut ans = vec![];
    for i in 1000..LIMIT {
        let mut j = 1;
        while i + 2 * j <= LIMIT {
            if is_prime[i]
                && is_prime[i + j]
                && is_prime[i + 2 * j]
                && check((i as i64, (i + j) as i64, (i + 2 * j) as i64))
            {
                ans.push((i, i + j, i + 2 * j));
            }
            j += 1;
        }
    }

    for x in ans {
        println!("{} {} {}", x.0, x.1, x.2);
    }
}
