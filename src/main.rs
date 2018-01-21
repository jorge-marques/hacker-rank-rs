mod hr {
    pub mod thirty_days {
        use std::io;

        pub fn _day1() {
            let mut input_string = String::new();

            io::stdin().read_line(&mut input_string)
                .expect("Could not read from stdin");

            println!("Hello World");
            println!("{}", input_string);
        }
    }

    pub mod algorithms {
        use std::io;

        pub fn _simple_array_sum() {
            let mut elements = String::new();

            io::stdin().read_line(&mut elements).expect("Could not read from stdin");

            let elements = elements
                .trim()
                .split(' ')
                .map(|x| x.parse::<isize>().expect("Could not parse"));

            let sum: isize = elements.sum();

            println!("{}", sum);
        }
    }
}

fn main() {
    hr::algorithms::_simple_array_sum();
}
