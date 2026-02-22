use super::*;

#[test]
fn test_greeting_returns_correct_message() {
    assert_eq!(greeting(), "Hello, world!");
}

#[test]
fn test_greeting_is_not_empty() {
    assert!(!greeting().is_empty());
}

#[test]
fn test_greeting_contains_hello() {
    assert!(greeting().contains("Hello"));
}

#[test]
fn test_greeting_contains_world() {
    assert!(greeting().contains("world"));
}

#[test]
fn test_greeting_ends_with_exclamation() {
    assert!(greeting().ends_with('!'));
}