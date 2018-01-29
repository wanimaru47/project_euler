fn isPrime(n: i32) -> bool {
    let sn = ((n as f64).sqrt() as i32) + 1;

    for i in 2..sn {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn hasZero(n: i32) -> bool {
    let mut n = n;
    while n > 0 {
        if n % 10 == 0 {
            return true;
        }

        n = n / 10;
    }

    false
}

fn rotateNumber(n: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    v.push(n);

    let mut m = (n % 10) * ((10.0 as f64).powi((n as f64).log(10.0) as i32) as i32) + (n / 10);

    while n != m {
        v.push(m.clone());
        m = (m % 10) * ((10.0 as f64).powi((m as f64).log(10.0) as i32) as i32) + (m / 10);
    }

    v
}

fn main() {
    let mut count = 0;

    'out: for i in 2..1000001 {
        if hasZero(i) {
            continue;
        }

        let v = rotateNumber(i);
        for x in v {
            if !isPrime(x) {
                continue 'out;
            }
        }

        count += 1;
    }

    println!("{}", count);
}
