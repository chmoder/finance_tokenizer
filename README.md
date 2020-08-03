# Tokenizer

Tokenizer is a library for tokenizing credit card data.

#### add tokenizer as a dependency to Cargo.toml 
```toml
[dependencies]
tokenizer = "0.1.0"
```

```rust,norun
use credit_card::CreditCard;
use tokenizer::Tokenizer;

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
};
```

# Current Features
- Tokenize [Credit Cards](https://github.com/chmoder/credit_card)

- blake3 hash support

# Future Features
- Swappable hash functions
- Tokenize other data types.  (String, Vec, bytes, ...)
- Tokenize debit cards, bank accounts, and other financial data

### Notice:
This is under development right now, so interfaces
and apis will be changing.  If you are interested
in using this please create an issue or reach out
with your feature request so I can help add it.