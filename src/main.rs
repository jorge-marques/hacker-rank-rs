mod thirty_days {
    use std::io;

    pub fn day1() {
        let mut input_string = String::new();

        io::stdin().read_line(&mut input_string)
            .expect("Could not read from stdin.");

        println!("Hello World");
        println!("{}", input_string);
    }
}

fn main() {
    thirty_days::day1();
}
