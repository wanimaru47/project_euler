fn main() {
    const LIMIT: usize = 1000000;
    let mut is_prime = [true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..is_prime.len() {
        if !is_prime[i] {
            continue;
        }

        let mut j = 2;
        while i * j < is_prime.len() {
            is_prime[i * j] = false;
            j += 1;
        }
    }

    let primes: Vec<usize> = is_prime.iter().enumerate().filter(|x| *x.1).map(|x| x.0).collect();

    let mut ans = 0;
    let mut max_con = 0;

    for i in 0..primes.len() {
        let mut sum = 0;
        for j in i..primes.len() {
            if sum + primes[j] <= LIMIT{
                sum += primes[j];
            } else {
                break;
            }

            if max_con <= j - i + 1 && is_prime[sum] {
                max_con = j - i + 1;
                ans = sum;
            }
        }
    }

    println!("{}", ans);
}
