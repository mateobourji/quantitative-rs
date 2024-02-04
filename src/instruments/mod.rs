use chrono::{DateTime, Utc};
use crate::cashflows::cashflow::CashFlow;

pub mod vanilla_option;

pub enum OptionType {
    Call,
    Put,
}

pub trait Value {
    fn calculate_payoff(&self, price_path: Vec<f64>) -> CashFlow;

    fn settlement_datetime(&self) -> DateTime<Utc>;
}