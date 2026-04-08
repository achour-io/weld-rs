use std::io::{self, Write};
use std::f64::consts;

fn str_to_f64(input : String) -> f64 {
    input.trim().parse().unwrap()
}

fn get_input(prompt : &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn calculate_current_radius(sma : f64, ecc : f64, tan : f64) -> f64 {
    sma*(1.0-ecc.powf(2.0))/(1.0+ecc*tan.to_radians().cos())
}

fn calculate_periapsis(sma : f64, ecc : f64) -> f64 {
    sma*(1.0-ecc)
}

fn calculate_apoapsis(sma : f64, ecc : f64) -> f64 {
    sma*(1.0+ecc)
}

fn calculate_velocity(sma : f64, distance : f64, mu : f64) -> f64 {
    (mu*(2.0/distance - 1.0/sma)).sqrt()
}

fn calculate_period(sma : f64, mu : f64) -> f64 {
    2.0*consts::PI*(sma.powf(3.0)/mu).sqrt()
}

fn calculate_sp_orbital_energy(sma : f64, mu : f64) -> f64 {
    -mu/(2.0*sma)
}

fn calculate_sp_angular_momentum(sma : f64, ecc : f64, mu : f64) -> f64 {
    (mu*sma*(1.0-ecc.powf(2.0))).sqrt()
}

fn format_time(time : f64) -> String {
    if time < 60.0 {
        format!("{time} seconds")
    } else if time < 3600.0 {
        format!("{:.2} minutes", time/60.0)
    } else {
        format!("{:.2} hours", time/3600.0)
    }
}

fn main() {
    println!("- The Orbital 1000 -");
    println!("--------------------");

    // Earth constants
    let radius_earth : f64 = 6371.0; // km
    let mu_earth: f64 = 398600.4418; // km^3/s^2

    // User inputs
    let sma : f64 = str_to_f64(get_input("Semi-major axis (km): "));
    let ecc : f64 = str_to_f64(get_input("Eccentricity: "));
    let tan : f64 = str_to_f64(get_input("True Anomaly (degrees): "));
    println!("--------------------");

    // Calculations
    let radius : f64 = calculate_current_radius(sma, ecc, tan);
    let velocity : f64 = calculate_velocity(sma, radius, mu_earth);
    let period : f64 = calculate_period(sma, mu_earth);
    let periapsis : f64 = calculate_periapsis(sma, ecc);
    let apoapsis : f64 = calculate_apoapsis(sma, ecc);
    let sp_orbital_energy : f64 = calculate_sp_orbital_energy(sma, mu_earth);
    let sp_angular_momentum : f64 = calculate_sp_angular_momentum(sma, ecc, mu_earth);

    // Outputs
    println!("Current Altitude (ASL): {} km", radius-radius_earth);
    println!("Apoapsis (ASL): {} km", apoapsis-radius_earth);
    println!("Periapsis (ASL): {} km", periapsis-radius_earth);
    println!("Current Velocity: {velocity} km/s");
    println!("Orbital Period: {}", format_time(period));
    println!("Specific Orbital Energy: {}", sp_orbital_energy);
    println!("Specific Angular Momentum: {}", sp_angular_momentum);
}