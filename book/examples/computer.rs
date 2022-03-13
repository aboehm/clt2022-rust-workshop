/// Gibt das Ergebnis einer vorzeichenbehafteten Addition mit einem 16-Bit Integer zurück
///
/// # Argumente
///
/// * `a` - 1. Summand
/// * `b` - 2. Summand
///
/// # Beispiele
///
/// ```rust
/// # use workshop::examples::computer::*;
/// let _res: i16 = add_signed_int16(1, -2);
/// ```
pub fn add_signed_int16(a: i16, b: i16) -> i16 {
    a + b
}

/// Gibt das Ergebnis einer vorzeichenlose Addition mit einem 16-Bit Integer zurück
///
/// # Argumente
///
/// * `a` - 1. Summand
/// * `b` - 2. Summand
///
/// # Beispiele
///
/// ```rust
/// # use workshop::examples::computer::*;
/// let _res: u16 = add_unsigned_int16(1, 2);
/// ```
pub fn add_unsigned_int16(a: u16, b: u16) -> u16 {
    a + b
}

/// Gibt das Ergebnis einer 16-Bit-Fließkomma Addition zurück
///
/// # Argumente
///
/// * `a` - 1. Summand
/// * `b` - 2. Summand
///
/// # Beispiele
///
/// ```rust
/// # use workshop::examples::computer::*;
/// let _res: f32 = add_float32(1.1, 2.2);
/// ```
pub fn add_float32(a: f32, b: f32) -> f32 {
    a + b
}

/// Hauptmethode
fn main() {
    let a = 16;
    let b = -3;
    let result = add_signed_int16(a, b);
    println!("Adding signed values {} and {}. Result is {}", a, b, result);

    let a = 3;
    let b = 4;
    let mut result = add_unsigned_int16(a, b);
    println!(
        "Adding unsigned values {} and {}. Result is {}",
        a, b, result
    );

    result = add_unsigned_int16(0, 1);
    println!(
        "Adding unsigned values {} and {}. Result is {}",
        0, 1, result
    );

    let a: u16 = 3;
    let b: f32 = 4.123;
    let result = add_float32(a.try_into().unwrap(), b);
    println!("Adding float values {} and {}. Result is {}", a, b, result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_signed_add() {
        assert_eq!(3, add_signed_int16(1, 2));
        assert_eq!(-1, add_signed_int16(2, -3));
    }

    #[test]
    fn test_unsigned_add() {
        assert_eq!(3, add_unsigned_int16(1, 2));
    }

    #[test]
    fn test_float_add() {
        assert_eq!(4.764, add_float32(1.223, 3.541));
    }
}
