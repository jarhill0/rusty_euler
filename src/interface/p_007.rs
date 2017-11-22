pub fn main() -> u32 {
    let mut counter = 0;
    let mut num = 2;
    while counter < 10_001 {
        while !is_prime(num) {
            num += 1;
        }
        counter += 1;
        num += 1;
    }

    num - 1
}

fn is_prime(n: u32) -> bool {
    let sqrt = (n as f64).sqrt() as u32;
    for x in 2..sqrt + 1 {
        if n % x == 0 {
            return false;
        }
    }
    true
}