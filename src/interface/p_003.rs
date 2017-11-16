fn largest_p_factor(mut n: u64) -> u32 {
    let mut f: u64 = 2;
    while f * f < n {
        while n % f == 0 {
            n /= f
        }
        f += 1;
    }
    n as u32
}

pub fn main() -> u32 {
    largest_p_factor(600851475143)
}