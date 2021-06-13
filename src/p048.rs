fn pow_mod(b: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        1
    } else {
        (b * pow_mod(b, n - 1, m)) % m
    }
}

fn main() {
    let ans: i64 = (1..=1000)
        .map(|n| pow_mod(n, n, 10000000000))
        .fold(0, |s, p| (s + p) % 10000000000);
    println!("{}", ans);
}
