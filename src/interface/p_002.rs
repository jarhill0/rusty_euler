pub fn main() -> u32 {
    let mut odd_1: u32;
    let mut odd_2: u32 = 1;
    let mut even: u32 = 2;
    let mut sum: u32 = 0;

    while even < 4_000_000 {
        sum += even;

        odd_1 = odd_2 + even;
        odd_2 = even + odd_1;
        even = odd_1 + odd_2
    }

    sum
}