pub fn main() -> u32 {
    factors_up_to(20)
}

fn factors_up_to(num: u32) -> u32 {
    let mut product: u32 = 1;
    for mut factor in 2..(num + 1) {
        factor = factors(factor, &product);
        product *= factor;
    }
    product
}

fn factors(mut f: u32, &num: &u32) -> u32 {
    let mut n = num;
    for x in 2..(f + 1) {
        while n % x == 0 && f % x == 0 {
            // while they have a common factor,
            n /= x; // divide it out
            f /= x;
        }
    }
    // f is mutated to be only the factors that are not already in n.
    f
}