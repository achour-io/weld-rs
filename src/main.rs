use std::io::{self, Write};
use std::f64::consts;
slint::include_modules!();

fn str_to_f64(input : String) -> f64 {
    input.trim().parse().unwrap()
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

fn main() -> Result<(), slint::PlatformError> {
    // Earth constants
    let radius_earth : f64 = 6371.0; // km
    let mu_earth: f64 = 398600.4418; // km^3/s^2

    let app = MainWindow::new()?;
    let weak_app = app.as_weak();

    app.on_button_pressed(move |op, sma_str, ecc_str, tan_str| {
        let app = weak_app.unwrap();

        let sma = str_to_f64(sma_str.to_string());
        let ecc = str_to_f64(ecc_str.to_string());
        let tan = str_to_f64(tan_str.to_string());

        app.set_label_text(match op.as_str() {
            "rad" => format!("Current Altitude (ASL): {:.2} km", calculate_current_radius(sma, ecc, tan)-radius_earth).into(),
            "prd" => format!("Orbital Period: {}", format_time(calculate_period(sma, mu_earth))).into(),
            "vel" => format!("Velocity: {:.2} km/s", calculate_velocity(sma, calculate_current_radius(sma, ecc, tan), mu_earth)).into(),
            "pe" => format!("Periapsis (ASL): {:.2} km", calculate_periapsis(sma, ecc)-radius_earth).into(),
            "apo" => format!("Apoapsis (ASL): {:.2} km", calculate_apoapsis(sma, ecc)-radius_earth).into(),
            "soe" => format!("Specific Orbital Energy: {:.2} km²/s²", calculate_sp_orbital_energy(sma, mu_earth)).into(),
            "sam" => format!("Specific Angular Momentum: {:.2} km²/s", calculate_sp_angular_momentum(sma, ecc, mu_earth)).into(),
            _ => "Invalid operation".into(),
        });
    });

    app.run()
}