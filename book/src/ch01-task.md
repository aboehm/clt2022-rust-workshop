# Aufgabe: Rechner

## Ziel

* Schreibt einen einfachen Rechner
* Operand `a` ist eine Zahl
* Operator `op` ist ein String
* Das Ergebnis wird ausgerechnet und ausgegeben

## Hilfen

Folgende Datei könnt ihr als Ausgangsgrundlage nutzen

```rust,editable,should_panic
enum Operation {
    Add,
    // TODO: Operation hinzufügen
}

/// Führt eine Operation aus
fn operate(a: i32, op: Operation, b: i32) -> Result<i32, String> {

    // TODO: Operation implementieren

    Err("Not implemented".to_string())
}

/// Wandelt ein Operations-String in eine Operation um
fn map_string_to_operation(input: &str) -> Result<Operation, String> {
    match input {
        "+" => Ok(Operation::Add),
        // TODO: Operation ergänzen
        _   => Err("Operation not implemented".to_string()),
    }
}

fn main() -> Result<(), String> {
    let a = 1;
    let op = "-";
    let b = 2;

    println!("Result: {}", operate(a, map_string_to_operation(op)?, b)?);
    Ok(())
}
```

## Musterlösung 

* [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f764782a023d3e436e1c66cc4b967754)
