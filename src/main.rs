fn main() {
    let base_string: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}[]@#!$%^&*()<>~-/+=.,;";
    let password = generate_password(base_string, 8);
    println!("{password:?}");
}

fn generate_password(base_string: &str, password_length: usize) -> String {
    // TODO: do not allow consecutive duplicate characters
    let mut password: String = String::new();
    while password.len() < password_length {
        let index = rand::random_range(0..base_string.len());
        password.push(base_string.chars().nth(index).unwrap());
    }
    password
}
