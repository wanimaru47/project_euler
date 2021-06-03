fn dfs(seq: Vec<i64>, used: Vec<bool>) -> i64 {
    if !used.contains(&false) {
        let checker = |x, y, z, d| (x * 100 + y * 10 + z) % d == 0;
        let d = [2, 3, 5, 7, 11, 13, 17];
        let mut flag = true;
        for i in 1..8 {
            if !checker(seq[i], seq[i+1], seq[i+2], d[i - 1]) {
                flag = false;
            }
        }
        return if flag {
            let mut res = 0;
            for s in seq {
                res = res * 10 + s;
            }
            return res;
        } else {
            0
        };
    }

    let mut res = 0;
    for i in 0..10 {
        if (seq.len() == 0 && i == 0) || used[i] {
            continue;
        }

        let mut used = used.clone();
        used[i] = true;
        let mut seq = seq.clone();
        seq.push(i as i64);
        res += dfs(seq, used);
    }

    return res;
}

fn main() {
    println!("{}", dfs(vec![], vec![false; 10]));
}
