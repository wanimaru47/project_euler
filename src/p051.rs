use std::collections::HashMap;

fn divide(mut n: i64) -> Vec<i64> {
    let mut d = vec![];
    while n > 0 {
        d.push(n % 10);
        n /= 10;
    }

    d
}

fn decode(v: &Vec<i64>, target: i64) -> String {
    let mut s = "".to_string();
    for (i, n) in v.iter().enumerate() {
        if target & 1 << i > 0 {
            s += "*";
        } else {
            s += &*n.to_string();
        }
    }

    s
}
#[test]
fn test_decode() {
    assert_eq!(
        "111***333",
        decode(&vec![1, 1, 1, 2, 2, 2, 3, 3, 3], 0b000111000)
    );
}

fn main() {
    const LIMIT: usize = 1000000;
    let mut is_prime = vec![true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p_map: HashMap<String, Vec<i64>> = HashMap::new();

    for i in 2..LIMIT {
        if is_prime[i] {
            let mut j = 2;
            while i * j <= LIMIT {
                is_prime[i * j] = false;
                j += 1;
            }
        }
    }

    let primes: Vec<i64> = is_prime
        .iter()
        .enumerate()
        .filter(|x| *x.1)
        .map(|x| x.0 as i64)
        .collect();

    for p in &primes {
        let d = divide(*p);
        for i in 1..1 << d.len() {
            let mut tmp = -1;
            let mut f = true;
            for j in 0..d.len() {
                if i & 1 << j != 0 && (tmp == -1 || d[j] == tmp) {
                    tmp = d[j];
                } else if i & 1 << j != 0 {
                    f = false;
                }
            }

            if f {
                let s = decode(&d, i);
                let v = p_map.entry(s).or_insert(vec![]);
                v.push(*p);
            }
        }
    }

    let mut ans = LIMIT as i64;
    for (_, v) in p_map {
        if v.len() == 8 && v[0] < ans {
            ans = v[0];
        }
    }
    println!("{}", ans);
}
