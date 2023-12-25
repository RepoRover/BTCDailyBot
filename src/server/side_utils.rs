use chrono::Utc;
use std::fmt::Display;

pub fn print_error<E: Display>(error_label: &str, error_message: E) {
    eprintln!("{} --- {}: {}", Utc::now(), error_label, error_message);
}
