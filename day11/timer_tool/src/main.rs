use std::{io,thread,time::Duration};
use std::io::{Write};

fn main(){
    println!("Basic Timer Tool");
    println!("Enter the timer duration (format: hours minutes seconds)");

    let duration = match get_timer_input() {
        Some(dur) =>dur,
        None => {
            println!("Invalid input");
            return;
        }
    };

    println!("Timer Set for: {} hours, {} minutes, {} seconds",duration.0,duration.1,duration.2);

    start_timer(duration.0,duration.1,duration.2);

    println!("Timer is Up")
}

fn get_timer_input() ->Option<(u64, u64,u64)> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    Some((hours,minutes,seconds))
}

fn start_timer(hours: u64,minutes: u64,seconds: u64) {
    let total_sec = hours*3600 + minutes*60 + seconds;
    for i in (1..=total_sec).rev(){
        let hrs = i / 3600;
        let mins = (i%3600)/60;
        let sec = i % 60;

        print!("\rtime Remaining: {:02}:{:02}:{:02}",hrs,mins,sec);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!()
}