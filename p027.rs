use std::cmp::max;

fn is_prime(n: i32) -> bool {
    if n < 0 || n == 0 || n == 1 {
        return false;
    }
    let mut div = 2;
    while div * div <= n {
        if n % div == 0 {
            return false;
        }
        div += 1;
    }

    return true;
}
#[test]
fn prime_test() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(47));
    assert!(!is_prime(48));
}

fn calc(a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
        let f = (n * n) + (a * n) + b;
        if is_prime(f) {
            n += 1;
        } else {
            return n;
        }
    }
}

#[test]
fn calc_test() {
    assert_eq!(80, calc(-79, 1601));
    assert_eq!(40, calc(1, 41));
}

fn main() {
    let mut cnt = -1;
    let mut ans = -1;
    for a in -999..1000 {
        for b in -1000..1001 {
            let res = calc(a, b);
            if cnt == res {
                cnt = res;
                ans = max(ans, a * b);
            } else if cnt < res {
                cnt = res;
                ans = a * b;
            }
        }
    }

    println!("ans = {}", ans);
}