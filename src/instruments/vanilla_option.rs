extern crate chrono;

use chrono::{DateTime, Utc};

use crate::cashflows::CashFlow;
use crate::cashflows::Currency;
use crate::instruments::{OptionType, Value};

pub struct VanillaOption {
    pub strike: f64,
    pub exercise_datetime: DateTime<Utc>,
    pub settlement_datetime: DateTime<Utc>,
    pub option_type: OptionType,
    pub underlying_currency: Currency,
}

impl VanillaOption {
    pub fn new(strike: f64, exercise_datetime: DateTime<Utc>, settlement_datetime: DateTime<Utc>, option_type: OptionType, underlying_currency: Currency) -> Self {
        VanillaOption {
            strike,
            exercise_datetime,
            settlement_datetime,
            option_type,
            underlying_currency,
        }
    }
}

impl Value for VanillaOption
{
    fn calculate_payoff(&self, price_path: Vec<f64>) -> CashFlow {
        match self.option_type {
            OptionType::Call => CashFlow::new((price_path.last().unwrap() - self.strike).max(0.0), self.underlying_currency, self.settlement_datetime),
            OptionType::Put => CashFlow::new((self.strike - price_path.last().unwrap()).max(0.0), self.underlying_currency, self.settlement_datetime)
        }
    }

    fn settlement_datetime(&self) -> DateTime<Utc> {
        self.settlement_datetime
    }

    fn underlying_currency(&self) -> Currency { self.underlying_currency }
}
