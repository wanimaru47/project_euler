use std::collections::HashSet;

fn divide(mut n: i64) -> Vec<i64> {
    let mut d = vec![];
    while n > 0 {
        d.push(n % 10);
        n /= 10;
    }

    return d;
}

fn main() {
    let mut n = 1;
    loop {
        let x: HashSet<Vec<i64>> = (1..=6).map(|p| n * p)
            .map(|x| divide(x))
            .map(|x| {
                let mut y = x.clone();
                y.sort();
                return y;
            }).collect();

        if x.len() == 1 {
            println!("{}", n);
            return ;
        }

        n += 1;
    }
}
