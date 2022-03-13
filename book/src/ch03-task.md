# Übungsaufgabe

## Ziel

* Ein einfacher String soll in einen Operationstyp umgewandelt werden
* Die Berechnung der Operation soll bei der Darstellung der Operationsdatenstruktur durchgeführt werden

## Hilfestellung

* Traits für Rechenoperationen sind in [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) definiert

```rust,editable,should_panic
use std::convert::From;
use std::fmt;

#[derive(Debug)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Operation<T> {
    a: T,
    b: T,
    op: OperationType,
}

impl<T> fmt::Display for Operation<T> 
where
    T: std::marker::Copy,
    T: std::fmt::Display,
    // TODO: weitere Trait-Anforderungen definieren
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", "Not implemented")
    }
}

impl From<&str> for OperationType {
    fn from(input: &str) -> Self {
        match input {
            // TODO: Mapping implementieren
            _ => OperationType::Add,
        }
    }
}

fn main() -> Result<(), String> {
    let op = Operation { a: 1, op: "+".into(), b: 2};
    println!("Result: {}", op);
    Ok(())
}
```

## Musterlösung

* [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=027444cd6b3cf41be45e4ae3fad83d30)
