extern crate chrono;
extern crate statrs;

use statrs::distribution::{ContinuousCDF, Normal};
use chrono::offset::Utc;
use crate::instruments::OptionType;
use crate::instruments::vanilla_option::VanillaOption;

fn normal_cdf(x: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(x)
}
pub fn black_scholes_price2<T>(instrument: &T, s0: f64, r: f64, sigma: f64) -> f64 {
    let time_to_maturity = instrument.maturity.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
    let d1 = (s0 / instrument.strike).ln() + (r + sigma.powi(2) / 2.0) * time_to_maturity / (sigma * time_to_maturity.sqrt());
    let d2 = d1 - sigma * time_to_maturity.sqrt();

    match instrument.option_type {
        OptionType::Call => s0 * normal_cdf(d1) - instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(d2),
        OptionType::Put => instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(-d2) - s0 * normal_cdf(-d1),
    }
}
macro_rules! create_black_scholes_pricing_function {
    ($func_name:ident, $option_type:ty) => {
        pub fn $func_name(instrument: &$option_type, s0: f64, r: f64, sigma: f64) -> f64 {
            let time_to_maturity = instrument.maturity.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
            let d1 = (s0 / instrument.strike).ln() + (r + sigma.powi(2) / 2.0) * time_to_maturity / (sigma * time_to_maturity.sqrt());
            let d2 = d1 - sigma * time_to_maturity.sqrt();

            match instrument.option_type {
                OptionType::Call => s0 * normal_cdf(d1) - instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(d2),
                OptionType::Put => instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(-d2) - s0 * normal_cdf(-d1),
            }
        }
    };
}

create_black_scholes_pricing_function!(black_scholes_price, VanillaOption);



mod tests {
    use chrono::Duration;
    use super::*;

    #[test]
    fn test_normal_cdf() {
        let value = normal_cdf(0.0);
        assert!((value - 0.5).abs() < 0.001);
    }

    fn test_black_scholes(option_type: OptionType, strike: f64, s0: f64, r: f64, sigma: f64, expected_price: f64) {

        let option = VanillaOption {
            strike,
            maturity: Utc::now() + Duration::days(365),
            option_type,
        };

        let price = black_scholes_price(&option, s0, r, sigma);

        assert!((price - expected_price).abs() <= 0.1, "Price {} not within expected range {}", price, expected_price);
    }

    #[test]
    fn test_black_scholes_variations() {
        let r = 0.05;
        let sigma = 0.2;

        test_black_scholes(OptionType::Call, 100.0, 100.0, r, sigma, 10.45);
        test_black_scholes(OptionType::Call, 120.0, 100.0, r, sigma, 3.25);
        test_black_scholes(OptionType::Call, 80.0, 100.0, r, sigma, 24.59);
        test_black_scholes(OptionType::Put, 100.0, 100.0, r, sigma, 5.57);
        test_black_scholes(OptionType::Put, 120.0, 100.0, r, sigma, 17.39);
        test_black_scholes(OptionType::Put, 80.0, 100.0, r, sigma, 0.69);

    }
}