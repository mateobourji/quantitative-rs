use chrono::Utc;
use rand_distr::num_traits::Pow;
use crate::instruments::OptionType;
use crate::instruments::vanilla_option::VanillaOption;

pub fn binomial_price(instrument: &VanillaOption, s0: f64, r: f64, sigma: f64, n: usize) -> f64 {
    let time_to_maturity = instrument.exercise_datetime.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
    let dt = time_to_maturity  / n as f64;
    let up = (sigma * dt.sqrt()).exp();
    let down = 1.0 / up;
    let p = (r.exp() * dt - down) / (up - down);

    let mut price_tree = vec![0.0; n + 1];

    for j in 0..=n {
        let stock_price = s0 * up.pow(j as i32) * down.pow(n as i32 - j as i32);
        price_tree[j] = if let OptionType::Call = instrument.option_type {
            (stock_price - instrument.strike).max(0.0)
        } else {
            (instrument.strike - stock_price).max(0.0)
        };
    }

    // Calculate the option price at each node
    for i in (0..n).rev() {
        for j in 0..=i {
            price_tree[j] = ((p * price_tree[j + 1] + (1.0 - p) * price_tree[j]) / r.exp().pow(dt)).max(0.0);
        }
    }

    price_tree[0]
}