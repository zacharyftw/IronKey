use super::utils::savepass;
use rand::prelude::SliceRandom;
use std::io;

pub fn passgen(selected_options: [bool; 4], pass_len: usize) -> io::Result<String> {
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
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "password length must be greater than 0",
        ));
    }

    if charset.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "at least one character type must be selected",
        ));
    }

    let mut charset_vec = charset.chars().collect::<Vec<char>>();
    let mut rng = rand::thread_rng();
    charset_vec.shuffle(&mut rng);

    let mut pass = String::with_capacity(pass_len);

    for &char in charset_vec.iter().cycle().take(pass_len) {
        pass.push(char);
    }
    savepass("passwords.txt", &pass)?;

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

    #[test]
    fn no_consecutive_characters() {
        let selected_options = [true, true, true, true]; // Enable all character types
        let pass_len = 100; // Generate a relatively long password to test this thoroughly
        let password = passgen(selected_options, pass_len).unwrap();

        // Check for consecutive characters
        let mut last_char = '\0'; // Initialize with a character that won't be in the password
        let mut consecutive_found = false;
        for c in password.chars() {
            if c == last_char {
                consecutive_found = true;
                break;
            }
            last_char = c;
        }

        assert!(
            !consecutive_found,
            "Generated password contains consecutive characters."
        );
    }
}
