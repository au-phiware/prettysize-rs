#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate size;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

use size::{Base, Size, Style};
use std::str::FromStr;

use bincode::deserialize;

#[derive(Deserialize)]
struct Data {
    size: Size<u64>,
    base: Base,
    style: Style,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = deserialize::<Data>(data) {
        let s = data.size.to_string(data.base, data.style);
        let s = Size::<u64>::from_str(&s).unwrap();
        assert_eq!(data.size, s);
    }
});
