fn main() {
    fn make_adder(num: i32) -> impl FnMut(i32) -> i32 {
        let mut new_num = num;

        move |second_num: i32| {
            new_num += second_num;
            return new_num;
        }
    }

    let mut add_five = make_adder(5);
    let num = add_five(5);
    let second_num = add_five(10);

    println!("{num}, {second_num}, {0}", add_five(5));
}
