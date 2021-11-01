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
    println!("Still implementing!!");
}