extern crate chrono;

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CashFlow {
    pub amount: f64,
    pub settlement_datetime: DateTime<Utc>,
}

impl CashFlow {
    pub fn new(amount: f64, settlement_datetime: DateTime<Utc>) -> Self {
        CashFlow { amount, settlement_datetime }
    }

    pub fn value_at_date(&self, valuation_datetime: DateTime<Utc>, annual_discount_rate: f64) -> CashFlow {
        let duration = self.settlement_datetime - valuation_datetime;
        let years_to_settlement = duration.num_seconds() as f64 / (365.25 * 24.0 * 3600.0);

        CashFlow::new(self.amount / (1.0 + annual_discount_rate).powf(years_to_settlement), valuation_datetime)
    }
}

impl fmt::Display for CashFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6}", self.amount)
    }
}

impl PartialEq for CashFlow {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.settlement_datetime == other.settlement_datetime
    }
}

impl Add for CashFlow {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.settlement_datetime != other.settlement_datetime {
            panic!("Cannot add cashflows with different settlement dates.");
        }
        CashFlow {
            amount: self.amount + other.amount,
            settlement_datetime: self.settlement_datetime,
        }
    }
}

impl AddAssign for CashFlow {
    fn add_assign(&mut self, other: Self) {
        if self.settlement_datetime == other.settlement_datetime {
            self.amount += other.amount;
        } else {
            panic!("Cannot add cashflows with different settlement dates.");
        }
    }
}

impl Sub for CashFlow {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.settlement_datetime != other.settlement_datetime {
            panic!("Cannot subtract cashflows with different settlement dates.");
        }
        CashFlow {
            amount: self.amount - other.amount,
            settlement_datetime: self.settlement_datetime,
        }
    }
}

impl Mul<f64> for CashFlow {
    type Output = Self;

    fn mul(self, multiplier: f64) -> Self::Output {
        CashFlow {
            amount: self.amount * multiplier,
            settlement_datetime: self.settlement_datetime,
        }
    }
}

impl MulAssign<f64> for CashFlow {
    fn mul_assign(&mut self, multiplier: f64) {
        self.amount *= multiplier;
    }
}

impl Div<f64> for CashFlow {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Attempt to divide by zero.");
        }
        CashFlow {
            amount: self.amount / rhs,
            settlement_datetime: self.settlement_datetime,
        }
    }
}


impl DivAssign<f64> for CashFlow {
    fn div_assign(&mut self, divisor: f64) {
        if divisor == 0.0 {
            panic!("Attempt to divide by zero.");
        } else {
            self.amount /= divisor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone};

    #[test]
    fn cashflows_are_equal() {
        let cf1 = CashFlow::new(100.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(100.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        assert_eq!(cf1, cf2, "Cashflows with the same amount and settlement date should be equal");
    }

    #[test]
    fn cashflows_are_not_equal_due_to_amount() {
        let cf1 = CashFlow::new(100.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        assert_ne!(cf1, cf2, "Cashflows with different amounts should not be equal");
    }

    #[test]
    fn cashflows_are_not_equal_due_to_date() {
        let cf1 = CashFlow::new(100.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(100.0, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap());
        assert_ne!(cf1, cf2, "Cashflows with different settlement dates should not be equal");
    }

    #[test]
    fn test_cashflow_add() {
        let cf1 = CashFlow::new(100.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(150.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let result = cf1 + cf2;
        assert_eq!(result.amount, 250.0);
    }

    #[test]
    #[should_panic(expected = "Cannot add cashflows with different settlement dates.")]
    fn test_cashflow_add_panic() {
        let cf1 = CashFlow::new(100.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(150.0, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap());
        let _ = cf1 + cf2;
    }

    #[test]
    fn test_cashflow_add_assign() {
        let mut cf1 = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(150.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        cf1 += cf2;
        assert_eq!(cf1.amount, 350.0);
    }

    #[test]
    #[should_panic(expected = "Cannot add cashflows with different settlement dates.")]
    fn test_cashflow_add_assign_panic() {
        let mut cf1 = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(150.0, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap());
        cf1 += cf2;
    }
    #[test]
    fn test_cashflow_sub() {
        let cf1 = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(150.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let result = cf1 - cf2;
        assert_eq!(result.amount, 50.0);
    }

    #[test]
    #[should_panic(expected = "Cannot subtract cashflows with different settlement dates.")]
    fn test_cashflow_sub_panic() {
        let cf1 = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let cf2 = CashFlow::new(150.0, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap());
        let _ = cf1 - cf2;
    }

    #[test]
    fn test_cashflow_mul() {
        let cf = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let result = cf * 2.0;
        assert_eq!(result.amount, 400.0);
    }

    #[test]
    fn test_cashflow_mul_assign() {
        let mut cf = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        cf *= 2.0;
        assert_eq!(cf.amount, 400.0);
    }

    #[test]
    fn test_cashflow_div() {
        let cf = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let result = cf / 2.0;
        assert_eq!(result.amount, 100.0);
    }

    #[test]
    #[should_panic(expected = "Attempt to divide by zero")]
    fn test_cashflow_div_by_zero() {
        let cf = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        let _ = cf / 0.0; // This should panic
    }

    #[test]
    fn test_cashflow_div_assign() {
        let mut cf = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        cf /= 2.0;
        assert_eq!(cf.amount, 100.0);
    }

    #[test]
    #[should_panic(expected = "Attempt to divide by zero.")]
    fn test_cashflow_div_assign_panic() {
        let mut cf = CashFlow::new(200.0, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap());
        cf /= 0.0; // This should panic
    }

    #[test]
    fn test_present_value_one_year() {
        let amount = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap();
        let discount_rate = 0.05;

        let cashflow = CashFlow::new(amount, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(95.2412757719014, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }

    #[test]
    fn test_present_value_six_months() {
        let amount = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2023, 7, 1, 0, 0 ,0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap();
        let discount_rate = 0.05;

        let cashflow = CashFlow::new(amount, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(97.61119324310415, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }

    #[test]
    fn test_present_value_with_negative_discount_rate() {
        let amount: f64 = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap();
        let discount_rate = -0.01;

        let cashflow = CashFlow::new(amount, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(101.00940615592717, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }

    #[test]
    fn test_future_value_one_year() {
        let amount = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0 ,0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0 ,0).unwrap();
        let discount_rate = 0.05f64;

        let cashflow = CashFlow::new(amount, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(104.99649357857778, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }
}