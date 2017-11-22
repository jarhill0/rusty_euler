pub fn main() -> u32 {
    square_of_sum(100) - sum_of_squares(100)
}

fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for x in 1..n + 1 {
        sum += x.pow(2);
    }
    sum
}

fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for x in 1..n + 1 {
        sum += x;
    }
    sum.pow(2)
}