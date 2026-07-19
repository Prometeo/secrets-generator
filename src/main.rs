use std::env;

fn main() {
    // TODO: use clap
    let base_password_string: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}[]@#!$%^&*()<>~-/+=.,;";
    let url_safe_chars: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";

    let input = env::args().skip(1).take(2).collect::<Vec<String>>();
    let string_type = input[0].as_str();
    let size = input[1].parse::<usize>().unwrap();
    match string_type {
        "password" => println!("{}", generate_secure_string(base_password_string, size)),
        "string" => println!("{}", generate_secure_string(url_safe_chars, size)),
        _ => println!("Invalid argument"),
    }
}

fn generate_secure_string(base_string: &str, string_length: usize) -> String {
    let mut secure_string: String = String::new();
    while secure_string.len() < string_length {
        let index = rand::random_range(0..base_string.len());
        let random_char = base_string.chars().nth(index).unwrap();
        if random_char != secure_string.chars().last().unwrap_or_default() {
            secure_string.push(random_char);
        }
    }
    secure_string
}
