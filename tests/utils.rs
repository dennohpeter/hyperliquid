use hyperliquid::utils::{parse_price, parse_size};

#[test]
fn test_parse_price() {
    assert_eq!(parse_price(1234.5), "1234.5");
    assert_eq!(parse_price(1234.56), "1234.5");
    assert_eq!(parse_price(0.001234), "0.001234");
    assert_eq!(parse_price(0.0012345), "0.001234");
    assert_eq!(parse_price(1.2345678), "1.2345");
}

#[test]
fn test_parse_size() {
    assert_eq!(parse_size(1.001, 3), "1.001");
    assert_eq!(parse_size(1.001, 2), "1");
    assert_eq!(parse_size(1.0001, 3), "1");

    assert_eq!(parse_size(1.001, 0), "1");

    assert_eq!(parse_size(1.001, 5), "1.001");
}
