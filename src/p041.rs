use std::cmp::max;

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    let mut x = 2;
    while x * x <= n {
        if n % x == 0 {
            return false;
        }
        x += 1;
    }

    return true;
}
#[test]
fn test_is_prime() {
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(47));
    assert!(!is_prime(48));
}

fn is_pandigital(selected: &Vec<bool>) -> bool {
    let mut flag = true;
    for i in 1..10 {
        if !flag && selected[i]{
            return false;
        }

        if !selected[i] {
            flag = false;
        }
    }

    return true;
}
#[test]
fn test_is_pandigital() {
    assert!(is_pandigital(&[false,true,true,true,false,false,false,false,false,false].to_vec()));
    assert!(!is_pandigital(&[false,false,true,true,false,false,false,false,false,false].to_vec()));
    assert!(!is_pandigital(&[false,true,true,true,false,false,false,false,false,true].to_vec()));
}

fn solve(num: i64, selected: &Vec<bool>) -> Option<i64> {
    let mut ans = if is_prime(num) && is_pandigital(selected) {
        Some(num)
    } else {
        None
    };

    for i in 1..10 {
        if !selected[i as usize] {
            let mut selected = selected.clone();
            selected[i as usize] = true;
            let res = solve(num * 10 + i, &selected);

            if ans.is_none() && res.is_some() {
                ans = res;
            } else if ans.is_some() && res.is_some() {
                ans = Some(max(ans.unwrap(), res.unwrap()));
            }
        }
    }

    return ans;
}

fn main() {
    println!("{}", solve(0, &vec![false; 10]).unwrap())
}