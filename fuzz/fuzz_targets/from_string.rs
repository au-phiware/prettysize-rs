#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate size;

use size::Size;
use std::str::FromStr;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Size::<f64>::from_str(s);
    }
});
