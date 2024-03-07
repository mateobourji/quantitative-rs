use chrono::{DateTime, Utc};

use crate::cashflows::cashflow::CashFlow;
use crate::cashflows::currency::Currency;

pub mod vanilla_option;
pub mod barrier_option;

pub enum OptionType {
    Call,
    Put,
}

pub trait Value {
    fn calculate_payoff(&self, price_path: &Vec<f64>) -> CashFlow;

    fn settlement_datetime(&self) -> DateTime<Utc>;
    
    fn underlying_currency(&self) -> Currency;
}