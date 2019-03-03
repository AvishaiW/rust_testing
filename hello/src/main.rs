

fn main() {
    println!("{}", add(14, 18));
}

fn add(a: u64, b:u64) -> u64 {
    a + b
}


#[test]
fn test_add() {
    let a: u64 = 6565;
    let b: u64 = 6523;
    assert_eq!(add(a, b), a+b);
}