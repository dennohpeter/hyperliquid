use serde::Serializer;
use uuid::Uuid;

/// Parse price to the accepted number of decimals
/// Prices can have up to 5 significant figures, but no more than 6 decimals places
///
/// # Examples
/// ```
/// use hyperliquid::utils::parse_price;
///
/// assert_eq!(parse_price(1234.5), "1234.5");
/// assert_eq!(parse_price(1234.56), "1234.5");
/// assert_eq!(parse_price(0.001234), "0.001234");
/// assert_eq!(parse_price(0.0012345), "0.001234");
/// assert_eq!(parse_price(1.2345678), "1.2345");
/// ```
pub fn parse_price(px: f64) -> String {
    let px = format!("{px:.6}");

    let px = if px.starts_with("0.") {
        px
    } else {
        let px: Vec<&str> = px.split('.').collect();
        let whole = px[0];
        let decimals = px[1];

        let diff = 5 - whole.len(); // 0
        let sep = if diff > 0 { "." } else { "" };

        format!("{whole}{sep}{decimals:.0$}", diff)
    };

    let px = remove_trailing_zeros(&px);

    positive(px)
}

/// Parse size to the accepted number of decimals.
/// Sizes are rounded to the szDecimals of that asset.
/// For example, if szDecimals = 3 then 1.001 is a valid size but 1.0001 is not
/// You can find the szDecimals for an asset by making a `meta` request to the `info` endpoint
///
/// # Examples
/// ```
/// use hyperliquid::utils::parse_size;
///
/// assert_eq!(parse_size(1.001, 3), "1.001");
/// assert_eq!(parse_size(1.001, 2), "1");
/// assert_eq!(parse_size(1.0001, 3), "1");
/// assert_eq!(parse_size(1000.0, 0), "1000");
/// ```

pub fn parse_size(sz: f64, sz_decimals: u32) -> String {
    let sz = format!("{sz:.0$}", sz_decimals as usize);

    let px = remove_trailing_zeros(&sz);

    positive(px)
}

fn remove_trailing_zeros(s: &str) -> String {
    let mut s = s.to_string();
    while s.ends_with('0') && s.contains('.') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
}

fn positive(value: String) -> String {
    if value.starts_with('-') {
        "0".to_string()
    } else {
        value
    }
}

pub fn as_hex_option<S>(cloid: &Option<Uuid>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(cloid) = cloid {
        s.serialize_str(&format!("0x{}", cloid.simple()))
    } else {
        s.serialize_none()
    }
}

pub fn as_hex<S>(cloid: &Uuid, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("0x{}", cloid.simple()))
}
