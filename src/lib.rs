mod tokenizer;

pub use tokenizer::Tokenizer;

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;
    use credit_card::CreditCard;

    #[test]
    fn generate_token() {
        let cc = CreditCard {
            number: "4111111111111111".to_string(),
            cardholder_name: "Graydon Hoare".to_string(),
            expiration_month: "01".to_string(),
            expiration_year: "2023".to_string(),
            brand: Option::from("visa".to_string()),
            security_code: Option::from("123".to_string())
        };
        let token = Tokenizer::generate(&cc);
        assert_eq!(token.len(), 64);
        let token2 = Tokenizer::generate(&cc);
        assert_ne!(token, token2)
    }
}
