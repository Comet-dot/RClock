extern crate chrono;
use chrono::Local;
fn main() {
    let date = Local::now();
    println!("╭─────────────────────────────────╮");
    println!("{}", date.format("│     Year➔%Y Time➔%H:%M:%S     │"));
    println!("╰────┬───────────────────────┬────╯");
    println!("     │version 0.0.1 by Comet.│ ");
    println!("     ╰───────────────────────╯ ");
    }
