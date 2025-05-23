use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    // sikip1はプログラム名なのでスキップ
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    //引数がない場合はusageを表示して終了
    if numbers.is_empty() {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d)
}

// ユークリッドの互除法を使った最大公約数（Greatest Common Divisor: GCD）を計算するアルゴリズム
fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        let t = m;
        m = n % m;
        n = t;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
