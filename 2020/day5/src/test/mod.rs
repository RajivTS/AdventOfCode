use super::*;

#[test]
fn test_parse() {
    let input = "FBFBBFFRLR";
    let seat = Seat::parse(input);
    assert_eq!(seat, Seat { row: 44, col: 5 });
}