pub fn is_alphabetical(input: &str) -> bool {
    return input
        .chars()
        .all(|c| c.is_alphabetic())
}

pub fn is_valid_currency_short(input: &str) -> bool {
    return !input.is_empty()
        && input.len() == 3 
        && input.is_ascii()
        && is_alphabetical(input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_currency_short() {
        let input_valid = "JPY";
        let input_short = "US";
        let input_long = "EURO";
        let input_non_ascii = "💲US";
        let input_non_alphabetical = "$US";

        assert_eq!(true, is_valid_currency_short(&input_valid));
        assert_eq!(false, is_valid_currency_short(&input_short));
        assert_eq!(false, is_valid_currency_short(&input_long));
        assert_eq!(false, is_valid_currency_short(&input_non_ascii));
        assert_eq!(false, is_valid_currency_short(&input_non_alphabetical));
    }
}
