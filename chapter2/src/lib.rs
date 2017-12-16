// exercise 2-3
use std::i64;
pub fn hex_to_decimal(hex: &str) -> i64 {
    if is_hex(&hex) {
        i64::from_str_radix(&hex[2..], 16).unwrap()
    } else {
        -0
    }
}
fn is_hex(hex: &str) -> bool {
    let string = String::from(hex);
    let prefix = string.get(0..2);
    if prefix == Some("0x") || prefix == Some("0X") {
        true
    } else {
        false
    }
}
