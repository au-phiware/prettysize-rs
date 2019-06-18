use crate::Size;
use std::str::FromStr;

#[test]
fn unit_tests() {
    assert_eq!("200 bytes", format!("{}", Size::Bytes(200)));
    assert_eq!("200 KiB", format!("{}", Size::Kibibytes(200)));
    assert_eq!("2.00 MiB", format!("{}", Size::Kibibytes(2048)));
}

#[test]
fn size_equality() {
    assert_eq!(
        Size::Bytes(200),
        Size::Bytes(200),
        "Testing equality of two identically-constructed sizes"
    );
    assert_eq!(
        Size::Mebibytes(2),
        Size::Kibibytes(2048),
        "Testing equality of two identical sizes expressed in different units"
    );
    assert_eq!(
        Size::Mebibytes(2u8),
        Size::Mebibytes(2f64),
        "Testing equality of two identical sizes expressed in different types"
    );
}

#[test]
fn size_addition() {
    // as a reference...
    let size = &Size::Mebibytes(20) + &Size::Mebibytes(22);
    assert_eq!(size, Size::Mebibytes(42));

    // and not as a reference
    let size = Size::Mebibytes(20) + Size::Mebibytes(22);
    assert_eq!(size, Size::Mebibytes(42));
}

#[test]
fn primitive_multiplication() {
    let size = &Size::Gigabytes(12) * 7;
    assert_eq!(size.bytes(), 84000000000);
    let size = Size::Gigabytes(12) * 7;
    assert_eq!(size.bytes(), 84000000000);

    // and the other way around
    let size = 7 * Size::Gigabytes(12);
    assert_eq!(size.bytes(), 84000000000);

    // and with other types
    let size = &Size::Gigabytes(12) * 7.0;
    assert_eq!(size.bytes(), 84000000000);
    let size = 7.0 * Size::Gigabytes(12);
    assert_eq!(size.bytes(), 84000000000);
}

#[test]
fn primitive_division() {
    let size = &Size::Gigabytes(12) / 13f64;
    assert_eq!(size.bytes(), 923076923);

    let size = Size::Gigabytes(12.0) / 13;
    assert_eq!(size.bytes(), 923076923);
}

macro_rules! size_bytes {
    ($name:ident, $num:expr, $unit:ident, $T:ty, $res:tt) => {
        #[test]
        fn $name() {
            assert_eq!(Size::$unit::<$T>($num).bytes(), $res);
        }
    }
}

size_bytes!(size_bytes_b_i64_1, 792633534438178879, Bytes, i64, 792633534438178879);
size_bytes!(size_bytes_mb_u64_1, 16172825064112138, Kilobytes, u64, 16172825064112138000);
size_bytes!(size_bytes_tib_f64_1, -1.333602886575971, Tebibytes, f64, 18446742607397670991);

#[test]
fn size_from_str() {
    let size = Size::<f64>::from_str("200");
    assert_eq!(size.unwrap(), Size::Bytes(200));

    let size = Size::<f64>::from_str("200 bytes");
    assert_eq!(size.unwrap(), Size::Bytes(200));

    let size = Size::<f64>::from_str("200 KiB");
    assert_eq!(size.unwrap(), Size::Kibibytes(200));

    let size = Size::<f64>::from_str("2.50 MiB");
    assert_eq!(size.unwrap(), Size::Kibibytes(2560));

    let size = Size::<f64>::from_str("2G");
    assert_eq!(size.unwrap(), Size::Gigabytes(2));

    let size = Size::<f64>::from_str("~");
    assert_eq!(format!("{:?}", size.err().unwrap()), "TokenError(Unmatch)");

    let size = Size::<f64>::from_str("4.06 EiB").unwrap();
    assert_eq!(size, Size::Bytes::<i64>(4680861308703798272));
}
