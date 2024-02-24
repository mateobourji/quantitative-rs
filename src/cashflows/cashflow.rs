extern crate chrono;

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use chrono::{DateTime, Utc};
use crate::cashflows::Currency;

#[derive(Debug)]
pub struct CashFlow {
    pub amount: f64,
    pub currency: Currency,
    pub settlement_datetime: DateTime<Utc>,
}

impl CashFlow {
    pub fn new(amount: f64, currency: Currency, settlement_datetime: DateTime<Utc>) -> Self {
        CashFlow { amount, currency, settlement_datetime }
    }

    pub fn convert_to(&self, other_currency: Currency, conversion_rate: f64) -> CashFlow {
        CashFlow::new(self.amount * conversion_rate, other_currency, self.settlement_datetime)
    }
    
    pub fn value_at_date(&self, valuation_datetime: DateTime<Utc>, annual_discount_rate: f64) -> CashFlow {
        let duration = self.settlement_datetime - valuation_datetime;
        let years_to_settlement = duration.num_seconds() as f64 / (365.25 * 24.0 * 3600.0);

        CashFlow::new(self.amount / (1.0 + annual_discount_rate).powf(years_to_settlement), self.currency, valuation_datetime)
    }
    
    fn validate_operation_with(&self, other: &CashFlow) -> bool {
        if self.settlement_datetime != other.settlement_datetime {
            panic!("Cannot operate on cashflows with different settlement dates.");
        }
        else if self.currency != other.currency {
            panic!("Cannot operate on cashflows with different currencies.");
        }
        else{
            true
        }
    }
}

impl fmt::Display for CashFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:.6}", self.currency, self.amount)
    }
}


impl PartialEq for CashFlow {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.settlement_datetime == other.settlement_datetime
    }
}
impl Add<&CashFlow> for &CashFlow {
    type Output = CashFlow;

    fn add(self, other: &CashFlow) -> Self::Output {
        self.validate_operation_with(other);
        CashFlow {
                amount: self.amount + other.amount,
                currency: self.currency,
                settlement_datetime: self.settlement_datetime,
            }
    }
}

impl AddAssign<&CashFlow> for CashFlow {
    fn add_assign(&mut self, other: &CashFlow) {
        self.validate_operation_with(other);
        self.amount += other.amount;
    }
}

impl Sub<&CashFlow> for &CashFlow {
    type Output = CashFlow;

    fn sub(self, other: &CashFlow) -> Self::Output {
        self.validate_operation_with(other);
        CashFlow {
            amount: self.amount - other.amount,
            currency: self.currency,
            settlement_datetime: self.settlement_datetime,
        }
    }
}

impl SubAssign<&CashFlow> for CashFlow {
    fn sub_assign(&mut self, other: &CashFlow) {
        self.validate_operation_with(other);
        self.amount -= other.amount;
    }
}

impl Mul<f64> for CashFlow {
    type Output = Self;

    fn mul(self, multiplier: f64) -> Self::Output {
        CashFlow {
            amount: self.amount * multiplier,
            currency: self.currency,
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
            currency: self.currency,
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
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn cashflows_are_equal() {
        let cf1 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        assert_eq!(cf1, cf2, "Cashflows with the same amount and settlement date should be equal");
    }

    #[test]
    fn cashflows_are_not_equal_due_to_amount() {
        let cf1 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        assert_ne!(cf1, cf2, "Cashflows with different amounts should not be equal");
    }

    #[test]
    fn cashflows_are_not_equal_due_to_date() {
        let cf1 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        assert_ne!(cf1, cf2, "Cashflows with different settlement dates should not be equal");
    }

    #[test]
    fn test_cashflow_add() {
        let cf1 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let result = &cf1 + &cf2;
        assert_eq!(result.amount, 250.0);
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different settlement dates.")]
    fn test_cashflow_add_different_settlement_dates_panic() {
        let cf1 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let _ = &cf1 + &cf2;
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different currencies.")]
    fn test_cashflow_add_different_currencies_panic() {
        let cf1 = CashFlow::new(100.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::EUR, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let _ = &cf1 + &cf2;
    }

    #[test]
    fn test_cashflow_add_assign() {
        let mut cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        cf1 += &cf2;
        assert_eq!(cf1.amount, 350.0);
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different settlement dates.")]
    fn test_cashflow_add_assign_different_settlement_dates_panic() {
        let mut cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        cf1 += &cf2;
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different currencies.")]
    fn test_cashflow_add_assign_different_currencies_panic() {
        let mut cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::EUR, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        cf1 += &cf2;
    }


    #[test]
    fn test_cashflow_sub() {
        let cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let result = &cf1 - &cf2;
        assert_eq!(result.amount, 50.0);
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different settlement dates.")]
    fn test_cashflow_sub_different_settlement_dates_panic() {
        let cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let _ = &cf1 - &cf2;
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different currencies.")]
    fn test_cashflow_sub_different_currencies_panic() {
        let cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::EUR, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let _ = &cf1 - &cf2;
    }
    
    #[test]
    fn test_cashflow_sub_assign() {
        let mut cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        cf1 -= &cf2;
        assert_eq!(cf1.amount, 50.0);
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different settlement dates.")]
    fn test_cashflow_sub_assign_different_settlement_dates_panic() {
        let mut cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        cf1 -= &cf2;
    }

    #[test]
    #[should_panic(expected = "Cannot operate on cashflows with different currencies.")]
    fn test_cashflow_sub_assign_different_currencies_panic() {
        let mut cf1 = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        let cf2 = CashFlow::new(150.0, Currency::EUR, Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap());
        cf1 -= &cf2;
    }

    #[test]
    fn test_cashflow_mul() {
        let cf = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let result = cf * 2.0;
        assert_eq!(result.amount, 400.0);
    }

    #[test]
    fn test_cashflow_mul_assign() {
        let mut cf = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        cf *= 2.0;
        assert_eq!(cf.amount, 400.0);
    }

    #[test]
    fn test_cashflow_div() {
        let cf = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let result = cf / 2.0;
        assert_eq!(result.amount, 100.0);
    }

    #[test]
    #[should_panic(expected = "Attempt to divide by zero")]
    fn test_cashflow_div_by_zero() {
        let cf = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let _ = cf / 0.0; // This should panic
    }

    #[test]
    fn test_cashflow_div_assign() {
        let mut cf = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        cf /= 2.0;
        assert_eq!(cf.amount, 100.0);
    }

    #[test]
    #[should_panic(expected = "Attempt to divide by zero.")]
    fn test_cashflow_div_assign_panic() {
        let mut cf = CashFlow::new(200.0, Currency::USD, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        cf /= 0.0; // This should panic
    }

    #[test]
    fn test_conversion() {
        let initial_cash_flow = CashFlow::new(100.0, Currency::USD, Utc::now());
        let conversion_rate = 0.8;
        let converted_cash_flow = initial_cash_flow.convert_to(Currency::EUR, conversion_rate);

        assert_eq!(converted_cash_flow.amount, 80.0); 
        assert_eq!(converted_cash_flow.currency, Currency::EUR);
        assert_eq!(converted_cash_flow.settlement_datetime, initial_cash_flow.settlement_datetime);
    }

    #[test]
    fn test_identity_conversion() {
        let initial_cash_flow = CashFlow::new(100.0, Currency::USD, Utc::now());
        let conversion_rate = 1.0;
        let converted_cash_flow = initial_cash_flow.convert_to(Currency::USD, conversion_rate);

        assert_eq!(converted_cash_flow.amount, 100.0); 
        assert_eq!(converted_cash_flow.currency, Currency::USD);
        assert_eq!(converted_cash_flow.settlement_datetime, initial_cash_flow.settlement_datetime);
    }

    #[test]
    fn test_idempotent_conversion() {
        let initial_cash_flow = CashFlow::new(100.0, Currency::USD, Utc::now());
        let conversion_rate_to_eur = 0.8;
        let conversion_rate_back_to_usd = 1.25; 

        let converted_to_eur = initial_cash_flow.convert_to(Currency::EUR, conversion_rate_to_eur);
        let converted_back_to_usd = converted_to_eur.convert_to(Currency::USD, conversion_rate_back_to_usd);

        assert!((converted_back_to_usd.amount - 100.0).abs() < f64::EPSILON);
        assert_eq!(converted_back_to_usd.currency, initial_cash_flow.currency);
        assert_eq!(converted_back_to_usd.settlement_datetime, initial_cash_flow.settlement_datetime);
    }

    #[test]
    fn test_convert_zero_cashflow() {
        let initial_cash_flow = CashFlow::new(0.0, Currency::USD, Utc::now());
        let conversion_rate = 0.8;
        let converted_cash_flow = initial_cash_flow.convert_to(Currency::EUR, conversion_rate);
        
        assert_eq!(converted_cash_flow.amount, 0.0);
        assert_eq!(converted_cash_flow.currency, Currency::EUR);
        assert_eq!(converted_cash_flow.settlement_datetime, initial_cash_flow.settlement_datetime);
    }
    
    #[test]
    fn test_present_value_one_year() {
        let amount = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let discount_rate = 0.05;

        let cashflow = CashFlow::new(amount, Currency::USD, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(95.2412757719014, Currency::USD, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }

    #[test]
    fn test_present_value_six_months() {
        let amount = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2023, 7, 1, 0, 0, 0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let discount_rate = 0.05;

        let cashflow = CashFlow::new(amount, Currency::USD, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(97.61119324310415, Currency::USD, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }

    #[test]
    fn test_present_value_with_negative_discount_rate() {
        let amount: f64 = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let discount_rate = -0.01;

        let cashflow = CashFlow::new(amount, Currency::USD, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(101.00940615592717, Currency::USD, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }

    #[test]
    fn test_future_value_one_year() {
        let amount = 100.0;
        let settlement_datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let valuation_datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let discount_rate = 0.05f64;

        let cashflow = CashFlow::new(amount, Currency::USD, settlement_datetime);
        let present_value = cashflow.value_at_date(valuation_datetime, discount_rate);

        let expected_present_value = CashFlow::new(104.99649357857778, Currency::USD, valuation_datetime);
        assert_eq!(expected_present_value, present_value);
    }
}