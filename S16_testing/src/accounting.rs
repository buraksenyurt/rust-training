fn get_discount_amount(unit_price: f64, rate: f64) -> Result<f64, AccountingError> {
    if unit_price < 0.0 {
        return Err(AccountingError::InvalidUnitPrice(
            "Unit price cannot be negative.",
        ));
    }
    if rate < 0.0 || rate > 1.0 {
        return Err(AccountingError::InvalidRate(
            "Discount rate must be between 0.0 and 1.0.",
        ));
    }
    Ok(unit_price * (1.0 - rate))
}

#[derive(Debug, PartialEq)]
enum AccountingError {
    InvalidUnitPrice(&'static str),
    InvalidRate(&'static str),
}

#[cfg(test)]
mod accounting_tests {
    use super::*;
    use crate::accounting::AccountingError::{InvalidRate, InvalidUnitPrice};

    #[test]
    fn test_no_discount() {
        let result = get_discount_amount(100.0, 0.0);
        assert_eq!(result.unwrap(), 100.0);
    }
    #[test]
    fn test_two_percent_discount() {
        let result = get_discount_amount(100.0, 0.2);
        assert_eq!(result.unwrap(), 80.0);
    }
    #[test]
    fn test_full_discount() {
        let result = get_discount_amount(100.0, 1.0);
        assert_eq!(result.unwrap(), 0.0);
    }
    #[test]
    fn test_negative_price() {
        let result = get_discount_amount(-50.0, 0.2);
        assert_eq!(
            result,
            Err(InvalidUnitPrice("Unit price cannot be negative."))
        );
    }
    #[test]
    fn test_invalid_discount_greater_than_one() {
        let result = get_discount_amount(100.0, 1.5);
        assert_eq!(
            result,
            Err(InvalidRate("Discount rate must be between 0.0 and 1.0."))
        );
    }
    #[test]
    fn test_invalid_discount_negative() {
        let result = get_discount_amount(100.0, -0.3);
        assert_eq!(
            result,
            Err(InvalidRate("Discount rate must be between 0.0 and 1.0."))
        );
    }
}
