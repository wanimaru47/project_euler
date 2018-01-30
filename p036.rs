fn encode(mut n: i32, base: i32) -> String {
    let mut res = "".to_string();
    while n > 0 {
        res += &(n % base).to_string();
        n = n / base;
    }

    res
}

fn check(s: String) -> bool {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() != s.chars().nth(s.len() - i - 1).unwrap() {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut sum: i64 = 0;
    for i in 1..1000001 {
        let s = encode(i.clone(), 2);
        let t = encode(i.clone(), 10);
        if check(s) && check(t) {
            sum += i as i64;
        }
    }

    println!("{}", sum);
}
