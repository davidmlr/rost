use std::io::Write;
use std::str::FromStr;


fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                                .expect("error parsing argument"));
    }
    
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: findm NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let result = findm(numbers[0], numbers[1]);

    println!("Die Mitte von von {:?} ist {}", numbers, result);
}

fn findm(n: u64, m: u64) -> u64 {
    assert!(m != 0);
    let result = (m-n)/2;
    result
}

#[test]
fn test_gcd() {
    assert_eq!(findm(0, 4), 2);
}