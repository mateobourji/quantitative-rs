extern crate chrono;

use crate::instruments::{OptionType, Value};
use crate::cashflows::cashflow;
use chrono::{DateTime, Utc};
use crate::cashflows::cashflow::CashFlow;

pub struct VanillaOption {
    pub strike: f64,
    pub exercise_datetime: DateTime<Utc>,
    pub settlement_datetime: DateTime<Utc>,
    pub option_type: OptionType,
}

impl VanillaOption {
    pub fn new(strike: f64, exercise_datetime: DateTime<Utc>, settlement_datetime: DateTime<Utc>, option_type: OptionType) -> Self {
        VanillaOption {
            strike,
            exercise_datetime,
            settlement_datetime,
            option_type,
        }
    }
}

impl Value for VanillaOption
{
    fn calculate_payoff(&self, price_path: Vec<f64>) -> CashFlow {
        match self.option_type {
            OptionType::Call => cashflow::CashFlow::new((price_path.last().unwrap() - self.strike).max(0.0), self.settlement_datetime),
            OptionType::Put => cashflow::CashFlow::new((self.strike - price_path.last().unwrap()).max(0.0), self.settlement_datetime)
        }
    }

    fn settlement_datetime(&self) -> DateTime<Utc> {
        self.settlement_datetime
    }
}
