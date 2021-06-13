fn main() {
    let mut n = 0;
    let mut frac = vec![];
    while frac.len() < 1000000 {
        n += 1;
        let mut tmp = n;
        let mut stack = vec![];
        while tmp > 0 {
            stack.push(tmp % 10);
            tmp /= 10;
        }

        while stack.len() > 0 {
            frac.push(stack.pop().unwrap())
        }
    }

    println!("{}", frac[0] * frac[10-1] * frac[100-1] * frac[1000-1] * frac[10000-1] * frac[100000-1] * frac[1000000-1])
}