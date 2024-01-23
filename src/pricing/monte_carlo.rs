use crate::instruments::Payoff;
use crate::instruments::vanilla_option::VanillaOption;
use crate::processes::heston_process::HestonProcess;

pub fn monte_carlo_price(instrument: &VanillaOption, heston_process: &HestonProcess, num_paths: usize) -> f64 {
    let mut total_payoff = 0.0;

    for _ in 0..num_paths {
        let price_path = heston_process.generate_path();
        total_payoff += instrument.payoff(price_path);
    }

    let average_payoff = total_payoff / num_paths as f64;
    let discounted_payoff = average_payoff * (-heston_process.model.r * heston_process.model.t).exp();

    discounted_payoff
}