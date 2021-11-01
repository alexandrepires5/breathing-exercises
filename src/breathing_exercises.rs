use std::time;
use std::thread::sleep;

// enum BreathingChoice {
//     BoxBreathing(String),
//     WimHofBreathing(String)
// }

/// Implementation of box breathing technique
pub fn box_breathing() {
        println!("Starting box breathing!");
        println!("First, put yourself comfortable!");
        println!("Make yourself comfortable! We can wait 10 seconds!");
        sleep(time::Duration::new(10, 0));
        println!("Exhale all your air now. We can wait 6 seconds for this");
        sleep(time::Duration::new(6, 0));
        for i in 0..4 {
            println!("Round {}", i + 1);
            println!("Now, breathe in for 4 seconds!");
            sleep(time::Duration::new(4, 0));
            println!("Now, hold for 4 seconds!");
            sleep(time::Duration::new(4, 0));
            println!("Now, breathe out for 4 seconds!");
            sleep(time::Duration::new(4, 0));
            println!("Now, hold for 4 seconds!");
            sleep(time::Duration::new(4, 0));
        }
        println!("All finished! Better now? :)");
}

pub fn wim_hof_breathing() {
    let holdout_time_in_seconds = [60, 90, 90];
    println!("Starting Wim Hof breathing! 3 rounds");
    println!("First, put yourself comfortable!");
    println!("Make yourself comfortable! We can wait 10 seconds!");
    sleep(time::Duration::new(10, 0));
    println!("Ok, are you ready? Here we go!");
    sleep(time::Duration::new(6, 0));
    for i in 0..3 {
        println!("Round {}", i + 1);
        for j in 0..30 {
            println!("Breath in!");
            sleep(time::Duration::new(2, 0));
            if j == 29 {
                println!("Fully out!");
                sleep(time::Duration::new(3, 0));
            }
            else {
                println!("Breath out!");
                sleep(time::Duration::new(2, 0));
            }
            
        }
        println!("Now, hold for {} seconds!", holdout_time_in_seconds[i]);
        sleep(time::Duration::new(holdout_time_in_seconds[i], 0));
        println!("Breathe in and hold for 15 seconds!");
        sleep(time::Duration::new(15, 0));
        println!("Breathe out...");
        sleep(time::Duration::new(3, 0));
    }
    println!("All finished! Better now? :)");
}
