use std::{thread, time::Duration};

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly....");

    thread::sleep(Duration::from_secs(2));

    intensity
}
