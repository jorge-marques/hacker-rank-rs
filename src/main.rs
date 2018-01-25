mod hr {
    pub mod thirty_days {
        use std::io;

        pub fn _hello_world() {
            let mut input_string = String::new();

            io::stdin().read_line(&mut input_string)
                .expect("Could not read from stdin");

            println!("Hello World");
            println!("{}", input_string);
        }

        pub fn _operators() {
            let mut cost = String::new();
            let mut tip = String::new();
            let mut tax = String::new();

            io::stdin().read_line(&mut cost).expect("Could not read from stdin");
            io::stdin().read_line(&mut tip).expect("Could not read from stdin");
            io::stdin().read_line(&mut tax).expect("Could not read from stdin");

            let cost: f32 = cost.trim().parse().expect("Could not parse");
            let tip: f32 = tip.trim().parse().expect("Could not parse");
            let tax: f32 = tax.trim().parse().expect("Could not parse");

            let tip = cost * (tip / 100.0);
            let tax = cost * (tax / 100.0);

            let total = (cost + tip + tax).round();

            println!("The total meal cost is {} dollars.", total);
        }

        pub fn _intro_to_conditional_statements() {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Could not read from stdin");

            let n: u32 = input.trim().parse().expect("Could not parse");

            if n % 2 != 0 {
                println!("Weird");
            } else if n >= 2 && n <= 5 {
                println!("Not Weird");
            } else if n >= 6 && n <= 20 {
                println!("Weird");
            } else {
                println!("Not Weird");
            }
        }

        pub fn _class_vs_instance() {
            struct Person {
                age: i32
            };

            impl Person {
                fn new(initial_age: i32) -> Person {
                    let mut age = initial_age;

                    if age < 0 {
                        println!("Age is not valid, setting age to 0.");
                        age = 0;
                    };

                    return Person { age };
                }

                fn am_i_old(&self) {
                    if self.age < 13 {
                        println!("You are young.")
                    } else if self.age >= 13 && self.age < 18 {
                        println!("You are a teenager.")
                    } else {
                        println!("You are old.")
                    }
                }

                fn year_passes(&mut self) {
                    self.age += 1;
                }
            }

            let t: i32 = read_line().trim().parse().unwrap();

            for _ in 0..t {
                let age = read_line().trim().parse().unwrap();
                let mut p = Person::new(age);

                p.am_i_old();

                for _ in 0..3 {
                    p.year_passes();
                }

                p.am_i_old();
                println!();
            }

            fn read_line() -> String {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Could not read from stdin");
                return input;
            }
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
    hr::thirty_days::_class_vs_instance();
}
