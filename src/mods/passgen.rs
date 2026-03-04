use rand::prelude::IndexedRandom;

pub fn passgen(selected_options: [bool; 4], pass_len: usize) -> Result<String, String> {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()_-+=|[]{};:',.<>?`~";

    let mut charset = String::new();

    if selected_options[0] {
        charset.push_str(lowercase_letters);
    }
    if selected_options[1] {
        charset.push_str(uppercase_letters);
    }
    if selected_options[2] {
        charset.push_str(numbers);
    }
    if selected_options[3] {
        charset.push_str(symbols);
    }

    if pass_len == 0 {
        return Err("password length must be greater than 0".to_string());
    }

    if charset.is_empty() {
        return Err("at least one character type must be selected".to_string());
    }

    let charset_vec: Vec<char> = charset.chars().collect();
    let mut rng = rand::rng();

    let mut pass = String::with_capacity(pass_len);
    for _ in 0..pass_len {
        let ch = charset_vec.choose(&mut rng).expect("charset is non-empty");
        pass.push(*ch);
    }
    Ok(pass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passgen() {
        // Test case 1: Only lowercase letters
        let selected_options = [true, false, false, false];
        let pass_len = 10;
        let result = passgen(selected_options, pass_len).unwrap();
        assert_eq!(result.len(), pass_len);

        // Test case 2: Only uppercase letters
        let selected_options = [false, true, false, false];
        let pass_len = 8;
        let result = passgen(selected_options, pass_len).unwrap();
        assert_eq!(result.len(), pass_len);

        // Test case 3: Only numbers
        let selected_options = [false, false, true, false];
        let pass_len = 12;
        let result = passgen(selected_options, pass_len).unwrap();
        assert_eq!(result.len(), pass_len);

        // Test case 4: Only symbols
        let selected_options = [false, false, false, true];
        let pass_len = 15;
        let result = passgen(selected_options, pass_len).unwrap();
        assert_eq!(result.len(), pass_len);

        // Test case 5: Combination of options
        let selected_options = [true, true, true, true];
        let pass_len = 20;
        let result = passgen(selected_options, pass_len).unwrap();
        assert_eq!(result.len(), pass_len);
    }
}
