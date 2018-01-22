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

        pub fn _a_very_big_sum() {
            let mut elements = String::new();

            io::stdin().read_line(&mut elements).expect("Could not read from stdin");

            let elements = elements
                .trim()
                .split(' ')
                .map(|x| x.parse::<u64>().expect("Could not parse"));

            let sum: u64 = elements.sum();

            println!("{}", sum);
        }

        pub fn _diagonal_difference() {
            let mut size = String::new();

            io::stdin().read_line(&mut size)
                .expect("Could not read from stdin");

            let size: usize = size.trim().parse().expect("Could not parse");

            let mut diff = 0;

            for i in 0..size {
                let row = read_row();

                diff += row[i] - row[size - 1 - i];
            }

            let diff = diff.abs();

            println!("{}", diff);

            fn read_row() -> Vec<i32> {
                let mut row = String::new();

                io::stdin().read_line(&mut row)
                    .expect("Could not read from stdin");

                row.trim().split_whitespace()
                    .map(|x| x.parse::<i32>().expect("Could not parse"))
                    .collect()
            }
        }

        pub fn _plus_minus() {
            let numbers = parse_input();
            let mut counter = [0, 0, 0];

            for n in &numbers {
                if *n > 0 {
                    counter[0] += 1;
                } else if *n < 0 {
                    counter[1] += 1;
                } else {
                    counter[2] += 1;
                }
            }

            counter.iter().for_each(|x| {
                println!("{}", *x as f32 / numbers.len() as f32);
            });

            fn parse_input() -> Vec<i32> {
                let mut input = String::new();

                io::stdin().read_line(&mut input)
                    .expect("Could not read from stdin");

                input.trim().split_whitespace()
                    .map(|x| x.parse::<i32>().expect("Could not parse"))
                    .collect()
            }
        }

        pub fn _staircase() {
            let mut size = String::new();

            io::stdin().read_line(&mut size)
                .expect("Could not read from stdin");

            let size: usize = size.trim().parse()
                .expect("Could not parse");

            for i in 0..size {
                println!("{:>1$}", (0..(i + 1)).map(|_| "#").collect::<String>(), size);
            }
        }
    }
}

fn main() {
    hr::algorithms::_staircase();
}
