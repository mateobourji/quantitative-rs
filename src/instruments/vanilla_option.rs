extern crate chrono;

use std::cmp::max;
use crate::instruments::{OptionType, Payoff};
use chrono::{DateTime, Utc};

pub struct VanillaOption {
    pub strike: f64,
    pub maturity: DateTime<Utc>,
    pub option_type: OptionType,
}

impl Payoff for VanillaOption
{
    fn payoff(&self, price_path: Vec<f64>) -> f64 {
        match self.option_type {
            OptionType::Call => max(price_path.last() - self.strike, 0.0),
            OptionType::Put => max(self.strike - price_path.last(), 0.0)
        }
    }
}
