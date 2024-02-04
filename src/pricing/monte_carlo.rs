use crate::cashflows::cashflow::CashFlow;
use crate::instruments::Value;
use crate::processes::Simulate;


fn monte_carlo_price<T: Value, U: Simulate>(instrument: &T, price_process: &U, annual_discount_rate: f64, number_of_paths: usize, number_of_steps: usize) -> f64 {
    let mut total_payoff = CashFlow::new(0.0, instrument.settlement_datetime());
    for _ in 0..number_of_paths {
        let price_path = price_process.generate_price_path(number_of_steps);
        total_payoff += instrument.calculate_payoff(price_path);
    }

    let average_payoff = total_payoff / number_of_paths as f64;
    let discount_average_payoff = average_payoff.value_at_date(instrument.settlement_datetime(), annual_discount_rate);

    discount_average_payoff.amount
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use crate::instruments::OptionType;
    use crate::instruments::vanilla_option::VanillaOption;
    use crate::processes::black_scholes_process::BlackScholesProcess;

    #[test]
    fn test_monte_carlo_black_scholes() {
        let option = VanillaOption {
            strike: 100.0,
            exercise_datetime: Utc::now() + Duration::days(365),
            settlement_datetime: Utc::now() + Duration::days(365 + 2),
            option_type: OptionType::Call,
        };

        let bs_process = BlackScholesProcess::new(100.0, 0.05, 0.2, 1.0); // s0, r, sigma, t
        let price = monte_carlo_price(&option, &bs_process, 0.05, 1000, 365); // number_of_paths, number_of_steps

        assert!(price > 0.0, "The calculated option price should be positive.");
    }
}