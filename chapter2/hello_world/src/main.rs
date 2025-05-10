fn main() {
    println!("Hello, world!");
}

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
