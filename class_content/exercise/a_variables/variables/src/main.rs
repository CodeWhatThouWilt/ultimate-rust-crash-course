const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles: i32 = STARTING_MISSILES;
    let mut ready: i32 = READY_AMOUNT;

    // let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{:p}, {:p}", &missiles, &ready);

    println!("{} missiles left", missiles - ready);
}
