fn main() {
    let mut sum = 0;
    let v = [
        2,
        3,
        5,
        7,
        23,
        37,
        53,
        73,
        313,
        317,
        373,
        797,
        3137,
        3797,
        739397,
    ];
    for n in v.iter() {
        sum += n;
    }
    println!{"{}", sum};
}
