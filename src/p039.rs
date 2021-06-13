fn main() {
    let mut count = vec![0; 4001];
    for a in 1..1001 {
        for b in 1..1001 {
            let c = ((a * a) as f64 + (b * b) as f64).sqrt() as i32;
            if a * a + b * b == c * c && a + b >= c {
                count[(a + b + c) as usize] += 1;
            }
        }
    }

    let mut ans = 0;
    let mut x = 0;
    for i in 0..1001 {
        if ans < count[i] {
            ans = count[i];
            x = i;
        }
    }

    println!("{}", x);
}
