mod p_001;
mod p_002;
mod p_003;
mod p_004;
mod p_005;
mod p_006;

fn p_001() {
    run(p_001::main);
}

fn p_002() {
    run(p_002::main);
}

fn p_003() {
    run(p_003::main);
}

fn p_004() {
    run(p_004::main);
}

fn p_005() {
    run(p_005::main);
}

fn p_006() {
    run(p_006::main);
}

pub fn solve(n: u32) {
    // I know this is awful. I'm a complete novice in Rust and I'll make it better later.
    // Enums? HashMap? Something else I'm not aware of? I'll figure it out later.
    if n == 1 {
        p_001();
    } else if n == 2 {
        p_002();
    } else if n == 3 {
        p_003();
    } else if n == 4 {
        p_004();
    } else if n == 5 {
        p_005();
    } else if n == 6 {
        p_006();
    } else {
        println!("That problem is not yet solved.")
    }
}

fn run<F>(f: F)
where
    F: Fn() -> (u32),
{
    let result = f();
    println!("{}", result);
}