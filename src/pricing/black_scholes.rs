extern crate chrono;
extern crate statrs;

use chrono::offset::Utc;
use statrs::distribution::{Continuous, ContinuousCDF, Normal};

use crate::instruments::OptionType;
use crate::instruments::vanilla_option::VanillaOption;

fn normal_cdf(x: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(x)
}

fn normal_pdf(x: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.pdf(x)
}

fn d1(strike: f64, s0: f64, r: f64, sigma: f64, time_to_maturity: f64) -> f64 {
    (s0 / strike).ln() + (r + sigma.powi(2) / 2.0) * time_to_maturity / (sigma * time_to_maturity.sqrt())
}

fn d2(d1: f64, sigma: f64, time_to_maturity: f64) -> f64 {
    d1 - sigma * time_to_maturity.sqrt()
}

macro_rules! create_black_scholes_functions {
    ($option_type:ty) => {
        pub fn black_scholes_price(instrument: &$option_type, s0: f64, r: f64, sigma: f64) -> f64 {
            let time_to_maturity = instrument.exercise_datetime.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
            let d1 = d1(instrument.strike, s0, r, sigma, time_to_maturity);
            let d2 = d2(d1, sigma, time_to_maturity);

            match instrument.option_type {
                OptionType::Call => s0 * normal_cdf(d1) - instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(d2),
                OptionType::Put => instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(-d2) - s0 * normal_cdf(-d1),
            }
        }

        pub fn delta(instrument: &$option_type, s0: f64, r: f64, sigma: f64) -> f64 {
            let time_to_maturity = instrument.exercise_datetime.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
            let d1 = d1(instrument.strike, s0, r, sigma, time_to_maturity);

            match instrument.option_type {
                OptionType::Call => normal_cdf(d1),
                OptionType::Put => normal_cdf(d1) - 1.0,
            }
        }

        pub fn gamma(instrument: &$option_type, s0: f64, r: f64, sigma: f64) -> f64 {
            let time_to_maturity = instrument.exercise_datetime.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
            let d1 = d1(instrument.strike, s0, r, sigma, time_to_maturity);
            normal_pdf(d1) / (s0 * sigma * time_to_maturity.sqrt())
        }

        pub fn vega(instrument: &$option_type, s0: f64, r: f64, sigma: f64) -> f64 {
            let time_to_maturity = instrument.exercise_datetime.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
            let d1 = d1(instrument.strike, s0, r, sigma, time_to_maturity);
            s0 * normal_pdf(d1) * time_to_maturity.sqrt()
        }

        pub fn theta(instrument: &$option_type, s0: f64, r: f64, sigma: f64) -> f64 {
            let time_to_maturity = instrument.exercise_datetime.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
            let d1 = d1(instrument.strike, s0, r, sigma, time_to_maturity);
            let d2 = d2(d1, sigma, time_to_maturity);

            match instrument.option_type {
                OptionType::Call => -s0 * normal_pdf(d1) * sigma / (2.0 * time_to_maturity.sqrt()) - r * instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(d2),
                OptionType::Put => -s0 * normal_pdf(d1) * sigma / (2.0 * time_to_maturity.sqrt()) + r * instrument.strike * (-r * time_to_maturity).exp() * normal_cdf(-d2),
            }
        }

        pub fn rho(instrument: &$option_type, s0: f64, r: f64, sigma: f64) -> f64 {
            let time_to_maturity = instrument.exercise_datetime.signed_duration_since(Utc::now()).num_days() as f64 / 365.25;
            let d1 = d1(instrument.strike, s0, r, sigma, time_to_maturity);
            let d2 = d2(d1, sigma, time_to_maturity);

            match instrument.option_type {
                OptionType::Call => instrument.strike * time_to_maturity * (-r * time_to_maturity).exp() * normal_cdf(d2),
                OptionType::Put => -instrument.strike * time_to_maturity * (-r * time_to_maturity).exp() * normal_cdf(-d2),
            }
        }
    };
}

create_black_scholes_functions!(VanillaOption);

mod tests {
    use chrono::Duration;

    use super::*;

    fn create_option(option_type: OptionType, strike: f64, days_to_maturity: i64) -> VanillaOption {
        VanillaOption {
            strike,
            exercise_datetime: Utc::now() + Duration::days(days_to_maturity),
            settlement_datetime: Utc::now() + Duration::days(days_to_maturity + 2),
            option_type,
        }
    }

    fn test_black_scholes(option_type: OptionType, strike: f64, s0: f64, r: f64, sigma: f64, expected_price: f64) {
        let option = create_option(option_type, strike, 365);

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

    #[test]
    fn test_delta() {
        let option = create_option(OptionType::Call, 100.0, 365);
        let delta = delta(&option, 100.0, 0.05, 0.2);
        // Compare delta with a known value or range
        assert!((delta - 0.63683).abs() < 0.1);
    }

    #[test]
    fn test_gamma() {
        let option = create_option(OptionType::Call, 100.0, 365);
        let gamma = gamma(&option, 100.0, 0.05, 0.2);
        // Compare gamma with a known value or range
        assert!((gamma - 0.01876).abs() < 0.1);
    }

    #[test]
    fn test_vega() {
        let option = create_option(OptionType::Call, 100.0, 365);
        let vega = vega(&option, 100.0, 0.05, 0.2);

        assert!((vega - 37.52403).abs() < 0.1);
    }

    #[test]
    fn test_theta() {
        let option = create_option(OptionType::Call, 100.0, 365);
        let theta = theta(&option, 100.0, 0.05, 0.2);

        assert!((theta - -6.41403).abs() < 0.1);
    }

    #[test]
    fn test_rho() {
        let option = create_option(OptionType::Call, 100.0, 365);
        let rho = rho(&option, 100.0, 0.05, 0.2);

        assert!((rho - 53.04977330181251).abs() < 0.1);
    }

    #[test]
    fn test_normal_cdf() {
        let value = normal_cdf(0.0);
        assert!((value - 0.5).abs() < 0.001);
    }
}