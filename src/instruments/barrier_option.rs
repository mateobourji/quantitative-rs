use chrono::{DateTime, Utc};
use crate::cashflows::{CashFlow, Currency};
use crate::instruments::{OptionType, Value};

pub struct BarrierOption {
    pub strike: f64,
    pub exercise_datetime: DateTime<Utc>,
    pub settlement_datetime: DateTime<Utc>,
    pub option_type: OptionType,
    pub barrier: Barrier,
    pub underlying_currency: Currency,
}

pub struct Barrier{
    pub barrier_type: BarrierType,
    pub level: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BarrierType {
    UpAndIn,
    UpAndOut,
    DownAndIn,
    DownAndOut
}

impl Value for BarrierOption {
    fn calculate_payoff(&self, price_path: &Vec<f64>) -> CashFlow {
        let barrier_crossed = match self.barrier.barrier_type {
            BarrierType::UpAndIn | BarrierType::UpAndOut => price_path.iter().any(|&p| p >= self.barrier.level),
            BarrierType::DownAndIn | BarrierType::DownAndOut => price_path.iter().any(|&p| p <= self.barrier.level),
        };

        let in_play = match self.barrier.barrier_type {
            BarrierType::UpAndIn | BarrierType::DownAndIn => barrier_crossed,
            BarrierType::UpAndOut | BarrierType::DownAndOut => !barrier_crossed,
        };

        if in_play {
            match self.option_type {
                OptionType::Call => CashFlow::new((price_path.last().unwrap() - self.strike).max(0.0), self.underlying_currency, self.settlement_datetime),
                OptionType::Put => CashFlow::new((self.strike - price_path.last().unwrap()).max(0.0), self.underlying_currency, self.settlement_datetime),
            }
        } else {
            CashFlow::new(0.0, self.underlying_currency, self.settlement_datetime)
        }
    }

    fn settlement_datetime(&self) -> DateTime<Utc> {
        self.settlement_datetime
    }

    fn underlying_currency(&self) -> Currency { self.underlying_currency }
}

