mod lib;
use lib::hex_to_decimal;
fn main() {
    let hex1 = "0xa1";
    let hex2 = "0x00";
    let hex3 = "0xff";
    println!("hex ({}) to decimal({})", hex1, hex_to_decimal(hex1));
    println!("hex ({}) to decimal({})", hex2, hex_to_decimal(hex2));
    println!("hex ({}) to decimal({})", hex3, hex_to_decimal(hex3));
}
