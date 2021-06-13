fn main() {
    let triangle = |n| n * (n + 1) / 2;
    let pentagonal = |n| n * (3 * n - 1) / 2;
    let hexagonal = |n| n * (2 * n - 1);

    let (mut i, mut j, mut k): (i64, i64, i64) = (286, 166, 144);
    while !(triangle(i) == pentagonal(j) && pentagonal(j) == hexagonal(k)) {
        let t = triangle(i);
        let p = pentagonal(j);
        let h = hexagonal(k);

        if t <= p && t <= h {
            i += 1;
        } else if p <= t && p <= h {
            j += 1;
        } else {
            k += 1;
        }
    }

    println!("{}", triangle(i));
}
