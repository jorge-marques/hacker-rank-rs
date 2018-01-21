mod hr {
    pub mod thirty_days {
        use std::io;

        pub fn _day0() {
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

        pub fn _compare_the_triplets() {
            let stdin = io::stdin();

            let mut a = String::new();
            let mut b = String::new();

            stdin.read_line(&mut a).expect("Could not read from stdin");
            stdin.read_line(&mut b).expect("Could not read from stdin");

            struct Triplets { a: [i32; 3], b: [i32; 3] }

            let triplets = Triplets {
                a: process_argument(&a),
                b: process_argument(&b),
            };

            struct Points { a: i32, b: i32 }

            let mut points = Points { a: 0, b: 0 };

            for n in 0..3 {
                let a = triplets.a[n];
                let b = triplets.b[n];

                if a > b {
                    points.a += 1;
                } else if a < b {
                    points.b += 1;
                }
            }

            println!("{} {}", points.a, points.b);

            fn process_argument(arg: &String) -> [i32; 3] {
                let mut ret = [0, 0, 0];

                for (i, e) in arg.trim().split(' ').enumerate() {
                    ret[i] = e.parse::<i32>().expect("Could not parse")
                }

                ret
            }
        }
    }
}

fn main() {
    hr::algorithms::_compare_the_triplets();
}
