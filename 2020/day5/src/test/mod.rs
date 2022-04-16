use super::*;

#[test]
fn test_parse() {
    assert_eq!(Seat::parse("BFFFBBFRRR"), Seat(567));
    assert_eq!(Seat::parse("FFFBBBFRRR"), Seat(119));
    assert_eq!(Seat::parse("BBFFBBFRLL"), Seat(820));
}