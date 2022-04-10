use super::*;

#[test]
fn test_is_valid() {
    let policy = PasswordPolicy {
        range: 1..=3,
        char: b'a',
        positions: (1, 3)
    };
    assert_eq!(policy.is_valid("zeus"), false, "no 'a's");
    assert_eq!(policy.is_valid("hades"), true, "single 'a'");
    assert_eq!(policy.is_valid("banana"), true, "three 'a's");
    assert_eq!(policy.is_valid("aaaah"), false, "too many 'a's");    
}

#[test]
fn test_is_valid_in_position() {
    let policy = PasswordPolicy {
        range: 1..=3,
        char: b'a',
        positions: (1, 3)
    };
    assert_eq!(policy.is_valid_in_position("abada"), false, "'a' in both positions");
    assert_eq!(policy.is_valid_in_position("keanu"), true, "'a' in second position");
    assert_eq!(policy.is_valid_in_position("apple"), true, "'a' in first position");
    assert_eq!(policy.is_valid_in_position("password"), false, "no 'a' at all");
}