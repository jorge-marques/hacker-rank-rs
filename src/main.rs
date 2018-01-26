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

        pub fn _loops() {
            let n: i32 = read_line().trim().parse().unwrap();

            for i in 1..11 {
                println!("{} x {} = {}", n, i, n * i);
            }

            fn read_line() -> String {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Could not read from stdin");
                return input;
            }
        }
    }

    pub mod algorithms {
        pub mod warmup {
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

            pub fn _mini_max_sum() {
                let input: Vec<u64> = read_line().trim().split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                let sum: u64 = input.iter().sum();

                let mut min: u64 = sum - input.iter().next().unwrap();
                let mut max: u64 = min;

                for n in input.iter() {
                    let result = sum - n;

                    if result < min {
                        min = result;
                    }

                    if result > max {
                        max = result;
                    }
                }

                println!("{} {}", min, max);

                fn read_line() -> String {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Could not read from stdin");
                    return input;
                }
            }

            pub fn _birthday_cake_candles() {
                let heights: Vec<u32> = read_line().trim().split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                let result = heights.iter().fold((0, 0), |mut res, e| {
                    if *e > res.0 {
                        res.0 = *e;
                        res.1 = 1;
                    } else if *e == res.0 {
                        res.1 += 1;
                    }

                    return res;
                });

                println!("{:?}", result.1);

                fn read_line() -> String {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Could not read from stdin");
                    return input;
                }
            }

            pub fn _time_conversion() {
                let time = read_line().trim().to_string();

                let pm = &time[time.len() - 2..] == "PM";
                let am = !pm;

                let hours = &time[..2];
                let tail = &time[2..time.len() - 2];

                if hours == "12" && pm {
                    println!("12{}", tail);
                    return;
                }

                if hours == "12" && am {
                    println!("00{}", tail);
                    return;
                }

                if pm {
                    let to_24 = hours.parse::<u8>().unwrap() + 12;
                    println!("{:?}{}", to_24, tail);
                    return;
                }

                if am {
                    println!("{}", &time[..time.len() - 2]);
                }

                fn read_line() -> String {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Could not read from stdin");
                    return input;
                }
            }
        }

        pub mod implementation {
            use std::io;

            pub fn _grading_students() {
                let n: u32 = _read_line().trim().parse().unwrap();

                let mut grades: Vec<u32> = vec![];

                for _ in 0..n {
                    grades.push(_read_line().trim().parse().unwrap());
                }

                for grade in grades {
                    if grade < 38 {
                        println!("{}", grade);
                        continue;
                    }

                    let nearest = _next_multiple(5, grade);

                    if nearest - grade < 3 {
                        println!("{}", nearest);
                        continue;
                    }

                    println!("{}", grade);
                }
            }

            fn _next_multiple(of: u32, from: u32) -> u32 {
                return from + of - (from % of);
            }

            use std::cmp::max;

            pub fn _apple_and_orange() {
                let house: Vec<i32> = _parse_args(&_read_line());
                let trees: Vec<i32> = _parse_args(&_read_line());
                let apples: Vec<i32> = _parse_args(&_read_line());
                let oranges: Vec<i32> = _parse_args(&_read_line());

                struct Score { larry: i32, rob: i32 }
                struct Position { larry: i32, rob: i32 }
                struct HouseBoundaries { left: i32, right: i32 }

                let position = Position { larry: trees[0], rob: trees[1] };
                let boundaries = HouseBoundaries { left: house[0], right: house[1] };

                let mut score = Score { larry: 0, rob: 0 };

                for i in 0..max(apples.len(), oranges.len()) {
                    if i < apples.len() {
                        let position = position.larry + apples[i];

                        if position >= boundaries.left && position <= boundaries.right {
                            score.larry += 1;
                        }
                    }

                    if i < oranges.len() {
                        let position = position.rob + oranges[i];

                        if position >= boundaries.left && position <= boundaries.right {
                            score.rob += 1;
                        }
                    }
                }

                println!("{}", score.larry);
                println!("{}", score.rob);
            }

            use std::str::FromStr;
            use std::fmt::Debug;

            fn _parse_arg<T>(arg: &str) -> T
                where T: FromStr, <T as FromStr>::Err: Debug
            {
                arg.trim().parse().unwrap()
            }

            fn _parse_args<T>(args: &str) -> Vec<T>
                where T: FromStr, <T as FromStr>::Err: Debug
            {
                args.trim().split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            }

            fn _read_line() -> String {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Could not read from stdin");
                return input;
            }
        }
    }
}

fn main() {
    hr::algorithms::implementation::_apple_and_orange();
}
