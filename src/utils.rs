pub fn float_to_int_for_hashing(num: f64) -> u64 {
    (num * 100_000_000.0).round() as u64
}

/// Parse price to the accepted number of decimals
/// Prices can have up to 5 significant figures, but no more than 6 decimals places
///
/// # Examples
/// ```
/// use hyperliquid::parse_price;
///
/// assert_eq!(parse_price(1234.5), "1234.5");
/// assert_eq!(parse_price(1234.56), "1234.5");
/// assert_eq!(parse_price(0.001234), "0.001234");
/// assert_eq!(parse_price(0.0012345), "0.001234");
/// assert_eq!(parse_price(1.2345678), "1.2345");
/// ```
pub fn parse_price(px: f64) -> String {
    let px = format!("{px:.6}");

    if px.starts_with("0.") {
        px
    } else {
        let px: Vec<&str> = px.split(".").collect();
        let whole = px[0];
        let decimals = px[1];

        let diff = 5 - whole.len(); // 0
        let sep = if diff > 0 { "." } else { "" };

        format!("{whole}{sep}{decimals:.0$}", diff)
    }
}

/// Parse size to the accepted number of decimals.
/// Sizes are rounded to the szDecimals of that asset.
/// For example, if szDecimals = 3 then 1.001 is a valid size but 1.0001 is not
/// You can find the szDecimals for an asset by making a `meta` request to the `info` endpoint
///
/// # Examples
/// ```
/// use hyperliquid::parse_size;
///
/// assert_eq!(parse_size(1.001, 3), "1.001");
/// assert_eq!(parse_size(1.001, 2), "1.00");
/// assert_eq!(parse_size(1.0001, 3), "1.000");
/// ```

pub fn parse_size(sz: f64, sz_decimals: u32) -> String {
    format!("{sz:.0$}", sz_decimals as usize)
}
