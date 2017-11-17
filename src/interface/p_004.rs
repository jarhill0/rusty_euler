pub fn main() -> u32 {
    let mut a: u32 = 999;
    let mut b: u32;
    let mut largest_solution: u32 = 0;
    while a >= 666 {
        // making the random assumtion that the factors are both larger than 666
        b = 999;
        while b >= 666 {
            if a * b > largest_solution && is_palindrome(a * b) {
                largest_solution = a * b;
            }
            b -= 1;
        }
        a -= 1;
    }
    largest_solution
}

fn is_palindrome(mut num: u32) -> bool {
    let mut reversed = 0;
    let num_original = num;

    while num != 0 {
        reversed *= 10;
        reversed += num % 10;
        num /= 10;
    }

    num_original == reversed
}