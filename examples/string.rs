

fn main() {
    let s = "0xB0A0".to_lowercase();
    let s = if "0x" == &s[..2] { &s[2..] } else { &s };
    let val = u16::from_str_radix(s, 16).expect("Invalid value");

    println!("Parsed value: {val:X?}");
}

