fn pentagonal(n: i64) -> i64 {
    return n * (n * 3 - 1) / 2;
}
#[test]
fn test_pentagonal() {
    assert_eq!(1, pentagonal(1));
    assert_eq!(5, pentagonal(2));
    assert_eq!(12, pentagonal(3));
}

fn is_pentagonal(n: i64) -> bool {
    let mut i = 1;
    while pentagonal(i) < n {
        i += 1;
    }
    return n == pentagonal(i);
}
#[test]
fn test_is_pentagonal() {
    assert!(is_pentagonal(1));
    assert!(is_pentagonal(51));
    assert!(!is_pentagonal(52));
}

fn main() {
    let mut i = 1;
    loop {
        for j in (1 .. i).rev() {
            let d = pentagonal(i) - pentagonal(j);
            let s = pentagonal(i) + pentagonal(j);

            if is_pentagonal(d) && is_pentagonal(s) {
                println!("{}", d);
                return ;
            }
        }

        i += 1;
    }
}
