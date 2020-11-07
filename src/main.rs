use std::io::{ stdin, stdout, Write };
use std::{ thread, time };
use std::process::Command;

struct Clock {
    hours: Option<u8>,
    minutes: Option<u8>,
    seconds: Option<u8>
}

impl Clock {
    pub fn new() -> Self {
        Self {
            hours: None,
            minutes: None,
            seconds: None
        }
    }

    pub fn set_a_valid_hour(&mut self) {
        let mut input = String::new();

        loop {
            print!("\n'Enter' for default (0 hours), or any number\n");
            print!(">>> "); stdout().flush().expect("write!");
            input.clear();
            stdin().read_line(&mut input).expect("read!");
            clear_terminal();

            match input.as_str() {
                "\n" => {self.hours = Some(0); break}
                _ => (),
            }

            self.hours = match input.trim().parse::<u8>() {
                Ok(hours) => {
                    if hours < 24 {
                        Some(hours)
                    } else {print!("Invalid hour"); continue}
                },
                Err(_) => {print!("Invalid hour"); continue}
            };
            break;
        }
    }

    pub fn set_a_valid_minute(&mut self) {
        let mut input = String::new();

        loop {
            print!("\n'Enter' for default (0 minutes), or any number\n");
            print!(">>> "); stdout().flush().expect("write!");
            input.clear();
            stdin().read_line(&mut input).expect("read!");
            clear_terminal();

            match input.as_str() {
                "\n" => {self.minutes = Some(0); break}
                _ => (),
            }

            self.minutes = match input.trim().parse::<u8>() {
                Ok(minutes) => {
                    if minutes < 60 {
                        Some(minutes)
                    } else {print!("Invalid minutes"); continue}
                },
                Err(_) => {print!("Invalid minutes"); continue}
            };
            break;
        }
    }

    pub fn set_a_valid_second(&mut self) {
        let mut input = String::new();

        loop {
            print!("\n'Enter' for default (0 seconds), or any number\n");
            print!(">>> "); stdout().flush().expect("write!");
            input.clear();
            stdin().read_line(&mut input).expect("read!");
            clear_terminal();

            match input.as_str() {
                "\n" => {self.seconds = Some(0); break}
                _ => (),
            }

            self.seconds = match input.trim().parse::<u8>() {
                Ok(seconds) => {
                    if seconds < 60 {
                        Some(seconds)
                    } else {print!("Invalid seconds"); continue}
                },
                Err(_) => {print!("Invalid seconds"); continue}
            };
            break;
        }
    }

    pub fn everything_is_ok(&self) -> bool {
        let mut result: bool = true;
        if self.hours == None {
            print!("Hours unitialized!\n");
            result = false;
        }
        if self.minutes == None {
            print!("Minutes unitialized!\n");
            result = false;
        }
        if self.seconds == None {
            print!("Seconds unitialized!\n");
            result = false;
        }
        result
    }

    pub fn print(&self) {
        if self.everything_is_ok() {
            print!("\n\t");

            print!("{0:>0minimum_width$}:",
                self.hours.unwrap(),
                minimum_width = 2
            ); stdout().flush().expect("write!");

            print!("{0:>0minimum_width$}:",
                self.minutes.unwrap(),
                minimum_width = 2
            ); stdout().flush().expect("write!");

            print!("{0:>0minimum_width$}\n",
                self.seconds.unwrap(),
                minimum_width = 2
            );
        } // end check for any unitialized number
    } // end print

    pub fn pass_one_second(&mut self) {
        if self.everything_is_ok() {
            if self.seconds.unwrap() < 59 {
                self.seconds = Some(self.seconds.unwrap() + 1);
            } else {
                self.seconds = Some(0);

                if self.minutes.unwrap() < 59 {
                    self.minutes = Some(self.minutes.unwrap() + 1);
                } else {
                    self.minutes = Some(0);

                    if self.hours.unwrap() < 23 {
                        self.hours = Some(self.hours.unwrap() + 1);
                    } else {
                        self.hours = Some(0);
            }}}
        } // end check for any unitialized number
    } // end pass one second

    pub fn run(&mut self) {
        if self.everything_is_ok() {
            let one_sec = time::Duration::from_millis(1000);
            loop {
                clear_terminal();
                self.print();
                self.pass_one_second();
                thread::sleep(one_sec);
            } // end clock loop
        } // end check for any unitialized number
    } // end run
}

fn main() {
    clear_terminal();

    let mut my_clock: Clock = Clock::new();
    my_clock.set_a_valid_hour();
    my_clock.set_a_valid_minute();
    my_clock.set_a_valid_second();
    my_clock.run();
}

fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
}
