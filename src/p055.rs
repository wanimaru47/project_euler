fn reverse(mut n: i128) -> i128 {
    let mut x = 0;
    while n > 0 {
        x = x * 10 + n % 10;
        n /= 10;
    }
    return x;
}
#[test]
fn test_reverse() {
    assert_eq!(4321, reverse(1234));
}

fn is_palindrome(n: i128) -> bool {
    n == reverse(n)
}

fn is_lychrel_number(mut n: i128) -> bool {
    n = n + reverse(n);
    for _ in 1..50 {
        if is_palindrome(n) {
            return false;
        }

        n = n + reverse(n);
    }

    return true;
}

fn main() {
    let mut cnt = 0;
    for i in 1..=10000 {
        if is_lychrel_number(i) {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
