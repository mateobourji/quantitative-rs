use chrono::Utc;
use crate::cashflows::CashFlow;
use crate::instruments::Value;
use crate::processes::Simulate;

pub fn monte_carlo_price<T: Value, U: Simulate>(instrument: &T, price_process: &U, annual_discount_rate: f64, number_of_paths: usize, number_of_steps: usize) -> CashFlow
{
    ((0..number_of_paths)
        .map(|_| price_process.generate_price_path(number_of_steps))
        .map(|price_path| instrument.calculate_payoff(&price_path))
        .sum::<CashFlow>() / (number_of_paths as f64))
        .value_at_date(Utc::now(), annual_discount_rate)
}


#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use statrs::assert_almost_eq;
    use crate::cashflows::currency::Currency;
    use crate::instruments::barrier_option::{Barrier, BarrierOption, BarrierType};

    use crate::instruments::OptionType;
    use crate::instruments::vanilla_option::VanillaOption;
    use crate::processes::black_scholes_process::BlackScholesProcess;
    use crate::processes::heston_process::HestonProcess;

    use super::*;

    #[test]
    fn test_monte_carlo_black_scholes_vanilla_option() {
        let option = VanillaOption {
            strike: 100.0,
            exercise_datetime: Utc::now() + Duration::days(365),
            settlement_datetime: Utc::now() + Duration::days(365 + 2),
            option_type: OptionType::Call,
            underlying_currency: Currency::USD,
        };

        let bs_process = BlackScholesProcess::new(100.0, 0.05, 0.2, 1.0);
        let price = monte_carlo_price(&option, &bs_process, 0.05, 1000, 365);
        assert!(price.amount > 0.0, "The calculated option price should be positive.");
    }

    #[test]
    fn test_monte_carlo_heston() {
        let option = VanillaOption {
            strike: 100.0,
            exercise_datetime: Utc::now() + Duration::days(365),
            settlement_datetime: Utc::now() + Duration::days(365 + 2),
            option_type: OptionType::Call,
            underlying_currency: Currency::USD,
        };

        let bs_process = HestonProcess::new(100.0, 0.05, 0.05, 0.8, 0.1, 0.2, 0.2, 1.0);
        let price = monte_carlo_price(&option, &bs_process, 0.05, 1000, 365);
        
        assert!(price.amount > 0.0, "The calculated option price should be positive.");
    }

    #[test]
    fn test_barrier_option_up_and_in_triggered() {
        let barrier_option = BarrierOption {
            strike: 100.0,
            exercise_datetime: Utc::now() + Duration::days(365),
            settlement_datetime: Utc::now() + Duration::days(365 + 2),
            option_type: OptionType::Call,
            barrier: Barrier {
                level: 105.0,
                barrier_type: BarrierType::UpAndIn,
            },
            underlying_currency: Currency::USD,
        };

        let bs_process = BlackScholesProcess::new(100.0, 0.05, 0.2, 1.0);
        let payoff = monte_carlo_price(&barrier_option, &bs_process, 0.05, 1000, 365);
        assert!(payoff.amount > 0.0, "Payoff should be positive when barrier is triggered.");
    }

    #[test]
    fn test_barrier_option_up_and_in_not_triggered() {
        let barrier_option = BarrierOption {
            strike: 100.0,
            exercise_datetime: Utc::now() + Duration::days(365),
            settlement_datetime: Utc::now() + Duration::days(365 + 2),
            option_type: OptionType::Call,
            barrier: Barrier {
                level: 1000.0,
                barrier_type: BarrierType::UpAndIn,
            },
            underlying_currency: Currency::USD,
        };

        let bs_process = BlackScholesProcess::new(100.0, 0.05, 0.2, 1.0);
        let payoff = monte_carlo_price(&barrier_option, &bs_process, 0.05, 1000, 365);
        assert_almost_eq!(payoff.amount, 0.0, 0.01);
    }
}