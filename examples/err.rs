use std::fs::File;
use std::io::prelude::*;
use std::io;


fn dummy_foo(is_succesful: bool) -> Result<u32, u8> {
    match is_succesful {
        true => Ok(1 as u32),
        false => Err(1 as u8),
    }
}

fn calling_dummy_foo() -> Result<u32, u8> {
    Ok(dummy_foo(true)?)
}

fn main() {
    let a = calling_dummy_foo().unwrap();
    println!("Obtained {a}");
}

