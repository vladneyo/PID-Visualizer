use crate::console_input::ConsoleInput;
use std::io;

#[derive(Debug)]
pub struct Input {
    pub target_value: f64,
    pub acceptable_time: f64, // in seconds
}
impl Input {
    pub fn new() -> Self {
        Self {
            target_value: 0.0,
            acceptable_time: 0.0,
        }
    }
    pub fn set(value: f64, time: f64) -> Input {
        Self {
            target_value: value,
            acceptable_time: time,
        }
    }
}

impl ConsoleInput for Input {
    fn type_in() -> Input {
        println!("Enter your Target Value:");

        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read value.");
        let target_value = value
            .trim()
            .parse()
            .expect("Please enter a valid float number.");

        println!("Enter your Acceptable Time:");

        let mut time = String::new();
        io::stdin()
            .read_line(&mut time)
            .expect("Failed to read time.");
        let acceptable_time = time
            .trim()
            .parse()
            .expect("Please enter a valid number of seconds.");

        Self {
            target_value,
            acceptable_time,
        }
    }
}
