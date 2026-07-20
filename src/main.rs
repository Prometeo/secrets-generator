use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Variant {
    Password,
    UrlSafe,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of secure string
    #[arg(short, long, value_enum)]
    variant: Variant,

    /// Secure string length
    #[arg(short, long, default_value_t = 8)]
    length: usize,
}

fn main() {
    let base_password_string: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}[]@#!$%^&*()<>~-/+=.,;";
    let url_safe_chars: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";

    let args = Args::parse();

    match args.variant {
        Variant::Password => println!(
            "{}",
            generate_secure_string(base_password_string, args.length)
        ),
        Variant::UrlSafe => println!("{}", generate_secure_string(url_safe_chars, args.length)),
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
