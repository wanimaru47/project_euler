fn is_prime(n: i64) -> bool {
    if n == 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}
#[test]
fn test_is_prime() {
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(47));
    assert!(!is_prime(48));
}

fn prime_factors(n: i64) -> Vec<(i64, i64)> {
    if n == 1 {
        return vec![(1, 1)];
    }

    let mut x = n;
    let mut i = 2;
    let mut res = vec![];
    while i * i <= n {
        let mut cnt = 0;
        while x % i == 0 {
            cnt += 1;
            x /= i;
        }

        if cnt > 0 {
            res.push((i, cnt));
        }
        i += 1;
    }

    if x != 1 {
        res.push((x, 1));
    }

    return res;
}
#[test]
fn test_prime_factors() {
    assert_eq!(vec![(2, 4)], prime_factors(16));
    assert_eq!(vec![(47, 1)], prime_factors(47));
    assert_eq!(vec![(3, 1), (5, 1), (43, 1)], prime_factors(645));
}

fn main() {
    let mut ans = 1;
    while prime_factors(ans).len() != 4
        || prime_factors(ans + 1).len() != 4
        || prime_factors(ans + 2).len() != 4
        || prime_factors(ans + 3).len() != 4
    {
        ans += 1;
    }

    println!("{}", ans);
}
