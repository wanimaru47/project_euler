fn convert(n: i64, k: i64) -> String {
    let mut s = "".to_string();
    for i in 1..(k + 1) {
        s += &(n * i).to_string();
    }

    s
}

fn check(s: String) -> bool {
    if s.len() != 9 {
        return false;
    }

    for x in vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
        let mut flag = false;
        for i in 0..9 {
            if s.chars().nth(i).unwrap() == x {
                flag = true;
            }
        }

        if !flag {
            return false;
        }
    }

    true
}

fn main() {
    let mut ans = "".to_string();

    for i in 2..10 {
        let mut g = 1;
        loop {
            let s = convert(g.clone(), i.clone());
            g += 1;
            if s.len() > 9 {
                break;
            }

            if s.len() < 9 {
                continue;
            }

            if check(s.clone()) {
                if ans < s {
                    ans = s;
                }
            }
        }
    }

    println!("ans: {}", ans);
}
