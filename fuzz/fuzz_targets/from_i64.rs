#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate size;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

use size::{Base, Size,Unit, Style};
use std::str::FromStr;

use bincode::deserialize;

#[derive(Deserialize)]
#[serde(remote = "Unit")]
pub enum UnitDef {
    Byte,
    Kibibyte,
    Kilobyte,
    Mebibyte,
    Megabyte,
    Gibibyte,
    Gigabyte,
    Tebibyte,
    Terabyte,
    Pebibyte,
    Petabyte,
    Exbibyte,
    Exabyte,
}

#[derive(Deserialize)]
#[serde(remote = "Base")]
pub enum BaseDef {
    Base2,
    Base10,
}

#[derive(Deserialize)]
#[serde(remote = "Style")]
pub enum StyleDef {
    Abbreviated,
    AbbreviatedLowerCase,
    Full,
    Smart,
    FullLowerCase,
}

#[derive(Deserialize, Debug)]
struct Data {
    number: i64,
    #[serde(with="UnitDef")]
    unit: Unit,
    #[serde(with="BaseDef")]
    base: Base,
    #[serde(with="StyleDef")]
    style: Style,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = deserialize::<Data>(data) {
        println!("{:?}", data);
        let Data{number, unit, base, style} = data;
        let size = match unit {
            Unit::Byte => Size::Bytes(number),
            Unit::Kibibyte => Size::Kibibytes(number),
            Unit::Kilobyte => Size::Kilobytes(number),
            Unit::Mebibyte => Size::Mebibytes(number),
            Unit::Megabyte => Size::Megabytes(number),
            Unit::Gibibyte => Size::Gibibytes(number),
            Unit::Gigabyte => Size::Gigabytes(number),
            Unit::Tebibyte => Size::Tebibytes(number),
            Unit::Terabyte => Size::Terabytes(number),
            Unit::Pebibyte => Size::Pebibytes(number),
            Unit::Petabyte => Size::Petabytes(number),
            Unit::Exbibyte => Size::Exbibytes(number),
            Unit::Exabyte => Size::Exabytes(number),
        };
        let abs = number.abs() as u64;
        println!("assert_eq!({:?}, Size::{:?}s::<i64>({:?}).bytes())", abs, unit, number);
        println!("{:?} == {:?}", abs, size.bytes());
        assert_eq!(abs, size.bytes());
        let s = size.to_string(base, style);
        println!("{:?} => {:?}", size, s);
        println!("assert_eq!(Size::<i64>::from_str({:?}).unwrap(), Size::{:?}s::<i64>({:?}))", s, unit, number);
        let s = Size::<i64>::from_str(&s).unwrap();
        println!("{:?} == {:?}", size, s);
        let size = format!("{:.2e}", size.bytes() as f64);
        let s = format!("{:.2e}", s.bytes() as f64);
        println!("{:?} == {:?}", size, s);
        assert_eq!(size, s);
    }
});
