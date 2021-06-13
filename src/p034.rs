fn fac(n: i32) -> i32 {
    if n <= 1 { 1 } else { n * fac(n - 1) }
}

fn sum_fact(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += fac(n % 10);
        n /= 10;
    }

    return sum;
}

fn main() {
    let mut sum = 0;
    for x in 3..3000000 {
        let mut s = 0;

        if x == sum_fact(x) {
            sum += x;
        }
    }

    println!("{}", sum);
}
