use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};

// Servo configuration. The datasheet was wrong for mine so these are manually calibrated.
// Your configuration may be different.
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 640;
const PULSE_NEUTRAL_US: u64 = 1540;
const PULSE_MAX_US: u64 = 2540;

fn wiggle_servo() -> Result<(), Box<dyn Error>> {
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) with the specified period,
    // and rotate the servo by setting the pulse width to its maximum value.
    println!("Rotating servo to the right side...");
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;

    // Sleep for 500 ms while the servo moves into position.
    thread::sleep(Duration::from_millis(500));

    // Rotate the servo to the opposite side.
    println!("Rotating servo to the left side...");
    pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;

    thread::sleep(Duration::from_millis(500));

    // Rotate the servo to its neutral (center) position in small steps.
    println!("Rotating servo to the center...");
    // pwm.set_pulse_width(Duration::from_micros(PULSE_NEUTRAL_US))?;
    // thread::sleep(Duration::from_millis(500));
    for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
        pwm.set_pulse_width(Duration::from_micros(pulse))?;
        thread::sleep(Duration::from_millis(20));
    }

    Ok(())
    // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
    // You can manually disable the channel by calling the Pwm::disable() method.
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn control_servo() -> Result<(), Box<dyn Error>> {
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) with the specified period,
    // and rotate the servo by setting the pulse width to its maximum value.
    println!("Rotating servo to the right side...");
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;
    loop {
        println!("Input a number between 0 and 100 to set the servo position, 'q' to quit");
        let input = get_user_input();
        match input.trim() {
            "q" => {
                println!("Quitting");
                break;
            }
            _ => {
                let input = input.trim().parse::<u64>().unwrap();
                let pulse = PULSE_MIN_US + (input * (PULSE_MAX_US - PULSE_MIN_US) / 100);
                println!("Setting pulse width to {}", pulse);
                pwm.set_pulse_width(Duration::from_micros(pulse))?;
            }
        }
    }

    Ok(())
    // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
    // You can manually disable the channel by calling the Pwm::disable() method.
}

fn main() -> () {
    println!("Input 'w' to wiggle the servo, 'c' to control it via user input");
    let input = get_user_input();
    match input.trim() {
        "w" => {
            println!("Wiggling servo");
            wiggle_servo().unwrap();
        }
        "c" => {
            println!("Control servo");
            control_servo().unwrap();
        }
        _ => {
            println!("Invalid input");
        }
    }
}
