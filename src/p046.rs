use std::collections::HashMap;

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

fn is_twice_a_square(n: i64) -> bool {
    let x = n / 2;

    let mut i = 1;
    while i * i < x {
        i += 1;
    }

    if x != i * i {
        return false;
    }

    return 2 * x == n;
}
#[test]
fn test_is_twice_a_square() {
    assert!(is_twice_a_square(2));
    assert!(!is_twice_a_square(4));
    assert!(is_twice_a_square(8));
    assert!(is_twice_a_square(18));
    assert!(!is_twice_a_square(2222));
}

fn main() {
    let mut n = 2;

    let mut memo: HashMap<i64, bool> = HashMap::new();
    memo.insert(1, false);

    loop {
        let is_p = is_prime(n);
        memo.insert(n, is_p);

        if n % 2 == 0 || is_p {
            n += 1;
            continue;
        }

        let mut f = false;
        for p in 2..n {
            if memo.get(&p) != Some(&true) {
                continue;
            }

            if is_twice_a_square(n - p) {
                f = true;
                break;
            }
        }

        if !f {
            println!("{}", n);
            return ;
        }

        n += 1;
    }
}
