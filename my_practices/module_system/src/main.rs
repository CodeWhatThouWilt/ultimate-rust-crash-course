use module_system::greet; // module system is the current project name
use rand::random; // imports thread_rng from the rand crate

fn main() {
    greet();
    let x: i32 = random();
    println!("{}", x);
}
