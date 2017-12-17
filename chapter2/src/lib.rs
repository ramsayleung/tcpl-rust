use std::i64;
use std::str;
// exercise 2-3
pub fn hex_to_decimal(hex: &str) -> i64 {
    if is_hex(&hex) {
        i64::from_str_radix(&hex[2..], 16).unwrap()
    } else {
        -1
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
// exercise 2-4
pub fn squeeze(s1: &str, s2: &str) -> String {
    s1.chars().filter(|&ch| !s2.contains(ch)).collect()
}
// exercise 2-5
pub fn any(s1: &str, s2: &str) -> i32 {
    match s1.chars().position(|ch| s2.contains(ch)) {
        Some(index) => index as i32,
        None => -1,
    }
}
pub fn getbits(x: u32, p: i32, n: i32) -> u32 {
    (x >> (p + 1 - n)) & !(!0 << n)
}
// exercise 2-6
pub fn setbits(x: u32, p: i32, n: i32, y: u32) -> u32 {
    let left_shift_n = 0xffffffff << n;
    let n_bits_1 = !(!0 << n) << (p + 1 - n);
    let n_bits_y = ((y | left_shift_n) << (p + 1 - n)) | !(!0 << (p + 1 - n));
    (x | n_bits_1) & n_bits_y
}
// exercise 2-7
pub fn invert(x: u32, p: i32, n: i32) -> u32 {
    setbits(x, p, n, !getbits(x, p, n))
}
// exercise 2-8
pub fn rightrot(x: u32, mut n: i32) -> u32 {
    if n > 32 {
        n = n % 32;
    }
    x >> n | x << (32 - n)
}
// exercise 2-9
pub fn bitcount(mut x: u32) -> i32 {
    let mut counter = 0;
    while x != 0 {
        counter = counter + 1;
        x &= x - 1;
    }
    counter
}
// exercise 2-10
pub fn lower(str: &str) -> String {
    str.to_lowercase()
}
#[cfg(test)]
mod tests {
    use super::{hex_to_decimal, squeeze, any, setbits, invert, rightrot, bitcount, lower};
    #[test]
    fn test_hex_to_decimal() {
        let hex1 = "0xa1";
        let hex2 = "0x00";
        let hex3 = "0xff";
        let not_hex = "xxx";
        assert_eq!(161, hex_to_decimal(hex1));
        assert_eq!(0, hex_to_decimal(hex2));
        assert_eq!(255, hex_to_decimal(hex3));
        assert_eq!(-1, hex_to_decimal(not_hex));
    }
    #[test]
    fn test_squeeze() {
        let str1 = "hellowolrd";
        let str2 = "ol";
        assert_eq!("hewrd", squeeze(str1, str2));
        assert_eq!("", squeeze(str2, str1));
    }
    #[test]
    fn test_any() {
        let str1 = "hellowolrd";
        let str2 = "ol";
        let str3 = "ty";
        assert_eq!(2, any(str1, str2));
        assert_eq!(-1, any(str1, str3));
    }
    #[test]
    fn test_setbits() {
        let p = 4;
        let n = 3;
        let x: u32 = 77; // binary 01001101
        let y: u32 = 6; // binary 00000110 -> set 110 to x
        /* result should be 01011001 = 89*/
        assert_eq!(89, setbits(x, p, n, y));
    }
    #[test]
    fn test_invert() {

        let p: i32 = 4;
        let n: i32 = 3;
        let x: u32 = 77; // binary 01001101
        /* result should be 01010001 = 81*/
        assert_eq!(81, invert(x, p, n));
    }
    #[test]
    fn test_rightrot() {
        let n: i32 = 3;
        let x: u32 = 77; // 00000000 00000000 00000000 01001101
        // result should be  10100000 00000000 00000000 00001001
        assert_eq!(2684354569, rightrot(x, n));
    }
    #[test]
    fn test_bitcounts() {
        let x: u32 = 77; // 01001101
        let y: u32 = 0xffffffff; // 11111111111111111111111111111111
        assert_eq!(4, bitcount(x));
        assert_eq!(32, bitcount(y));
    }
    #[test]
    fn test_lower() {
        let x = "HelloWorld";
        assert_eq!("helloworld", lower(x));
    }
}
