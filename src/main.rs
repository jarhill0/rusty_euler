mod interface;
use std::io;

fn main() {
    println!("Project Euler!");
    println!("Enter problem number");
    let mut problem_num = String::new();

    io::stdin().read_line(&mut problem_num).expect(
        "Failed to read line",
    );

    let problem_num: u32 = match problem_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number.");
            return;
        }
    };

    interface::solve(problem_num);
}
