# Testing

## Unittests

* Integriertes Testkonzept
* `cargo test` führt die Tests aus
* Tests müssen nur mit den Makro `#[test]` markiert werden
* Framework erweitern die Testfunktionen bspw. um asynchrone Funktionen, sequentielle Ausführung etc
* Tests werden gerne in getrennte Testmodule verlagert
* Feature `test` wird bei der Ausführung aktiviert, mit dem Makro `#[cfg(test)]` wird dann nur für den Test der entsprechende Code kompiliert

```rust
# fn main() { }
pub fn add_signed_int16(a: i16, b: i16) -> i16 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_signed_add() {
        assert_eq!(3, add_signed_int16(1, 2));
        assert_eq!(-1, add_signed_int16(2, -3));
    }
}
```

* Auch testbar über den [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8ebf8119c94caa11dde4e5971152b173)

## Benchmarks

* Cargo intergriert ebenfalls eine Benchmark Suite
* Wird über `cargo bench` aufgerufen

