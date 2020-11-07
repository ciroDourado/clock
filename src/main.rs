use std::io::{ stdout, Write };
use std::{ thread, time };
use std::process::Command;

struct Clock {
    hours: u8,
    minutes: u8,
    seconds: u8
}

impl Clock {
    pub fn new() -> Self {
        Self { hours: 23, minutes: 59, seconds: 58 }
    }

    pub fn print(&self) {
        print!("\n\t");
        print!("{0:>0minimum_width$}:",
            self.hours,
            minimum_width = 2
        ); stdout().flush().expect("write!");
        print!("{0:>0minimum_width$}:",
            self.minutes,
            minimum_width = 2
        ); stdout().flush().expect("write!");
        print!("{0:>0minimum_width$}\n",
            self.seconds,
            minimum_width = 2
        );
    }

    pub fn pass_one_second(&mut self) {

        if self.seconds < 59 { 
            self.seconds += 1;
        } else {
            self.seconds = 0;
            
            if self.minutes < 59 {
                self.minutes += 1;
            } else {
                self.minutes = 0;

                if self.hours < 23 {
                    self.hours += 1;
                } else {
                    self.hours = 0;
                }
            }
        }
    }

    pub fn run(&mut self) {
        let one_sec = time::Duration::from_millis(1000);
        loop {
            clear_terminal();
            self.print();
            self.pass_one_second();
            thread::sleep(one_sec);
        }
    }
}

fn main() {
    let mut my_clock: Clock = Clock::new();
    my_clock.run();
}

fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
}
