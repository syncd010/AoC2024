use std::{env, fs, process, time};

use aoc2024::AoCResult;

// Command line argument parsing
struct Config {
    day: u16,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut use_test_file = false;
        let mut day: Option<&String> = None;
        let mut file_path: Option<&String> = None;
    
        // Poor man's argument parsing...
        for arg in args.into_iter().skip(1) {
            if arg == "-t" {
                use_test_file = true;
            } else if day.is_none()  {
                day = Some(arg);
            } else {
                file_path = Some(arg);
            }
        }
    
        let day = match day {
            None => "".parse::<u16>(), // Force an error
            Some(s) => s.parse::<u16>(),
        };
    
        if day.is_err() {
            return Err("usage: aoc day [-t] [file]\n\
                \tday - Day to run\n\
                \t-t - Test switch, to use 'data/input[day]Test' file for the data if none specified\n\
                \tfile - data file, use 'data/input[day]' if none specified");
        }
        let day = day.unwrap();
    
        let file_path = match file_path {
            None => format!("data/input{}{}", day, if use_test_file {"Test"} else {""}),
            Some(s) => s.clone(),
        };

        Ok(Config { day, file_path })
    }
    
}

// Expands for each day
macro_rules! expand_days {
    ( $( $day_num:expr => $mod:ident ),* ) => {
        $(
            mod $mod;
        )*

        fn run_day_part(fn_part: fn(&str) -> AoCResult, input: &str, desc: &str) {
            let start = time::Instant::now();
            let res = fn_part(input);
            let elapsed = start.elapsed();
            print!("{desc} ({elapsed:?}): ");
            match res {
                AoCResult::None => { println!("No value"); },
                AoCResult::Str(val) => { println!("{val}"); },
                AoCResult::Int(val) => { println!("{val}"); },
            }
        }

        fn run_day(day: u16, input: &str) {
            match day {
                $(
                    $day_num => {
                        println!("Day {day}:");
                        run_day_part($mod::solve_part_one, &input, "Part one");
                        run_day_part($mod::solve_part_two, &input, "Part two");
                    },
                )*
                _ => {
                    println!("Day {day} not implemented.");
                },
            }
        }
    };
}

expand_days!(1 => day1, 
             2 => day2,
             3 => day3,
             4 => day4,
             5 => day5,
             6 => day6,
             7 => day7,
             8 => day8,
             9 => day9,
             10 => day10
            );

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    let input = fs::read_to_string(&config.file_path)
        .expect(&format!("Couldn't read file: {0}", config.file_path));

    run_day(config.day, &input);
}
