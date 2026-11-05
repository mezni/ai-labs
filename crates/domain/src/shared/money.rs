use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    amount: i64,
    currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    NegativeAmount,
    InvalidCurrency,
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeAmount => write!(f, "Amount cannot be negative"),
            Self::InvalidCurrency => write!(f, "Currency must be a valid three-letter code"),
        }
    }
}

impl std::error::Error for MoneyError {}

impl Money {
    pub fn new(amount: i64, currency: &str) -> Result<Self, MoneyError> {
        if amount < 0 {
            return Err(MoneyError::NegativeAmount);
        }

        if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
            return Err(MoneyError::InvalidCurrency);
        }

        Ok(Self {
            amount,
            currency: currency.to_string(),
        })
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_money_with_valid_amount_and_currency() {
        let money = Money::new(1999, "CAD").unwrap();

        assert_eq!(money.amount(), 1999);
        assert_eq!(money.currency(), "CAD");
    }

    #[test]
    fn rejects_negative_amount() {
        let result = Money::new(-100, "CAD");

        assert_eq!(result, Err(MoneyError::NegativeAmount));
    }

    #[test]
    fn rejects_invalid_currency() {
        let result = Money::new(1000, "cad");

        assert_eq!(result, Err(MoneyError::InvalidCurrency));
    }
}
