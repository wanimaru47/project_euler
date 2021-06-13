fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let mut m = x % y;
        x = y;
        y = m.clone();
    }

    return x;
}

fn main() {
    let mut d = 1;
    let mut h = 1;

    for i in 10..100 {
        for j in 10..100 {
            if (i >= j) {
                continue;
            }
            let x = i % 10;
            let y = (i / 10) % 10;
            let s = j % 10;
            let t = (j / 10) % 10;

            if (x == 0 || s == 0) {
                continue;
            }
            if (x == y || s == t) {
                continue;
            }

            let mut ans = false;
            if ((i as f64) / (j as f64) == (x as f64) / (s as f64) && y == t) {
                ans = true;
            }
            if ((i as f64) / (j as f64) == (x as f64) / (t as f64) && y == s) {
                ans = true;
            }
            if ((i as f64) / (j as f64) == (y as f64) / (s as f64) && x == t) {
                ans = true;
            }
            if ((i as f64) / (j as f64) == (y as f64) / (t as f64) && x == s) {
                ans = true;
            }

            if (ans) {
                h *= i;
                d *= j;
            }
        }
    }


    println!("{}", d / gcd(h, d));
}
