# Kontrollfluß nund Funktionen

## If/Then/Else

```rust,editable
# fn main() {
    if true {
        // wird ausgeführt
    } else {
        // wird nicht ausgeführt
    }

    let i = 3;
    if i == 0 {
        // ...
    } else if i == 1 {
        // ...
    } else {
        // ...
    }
# }
```

## Schleife

* Einfache Schleife

```rust,editable
# fn main() {
    let mut i = 0;
    loop {
        if i > 100 {
            break;
        }
        i += 1;
    }
# }
```

* For-Schleife

```rust,editable
# fn main() {
    for i in 0..5 {
        println!("i = {}", i);
    }
# }
```

## Match-Operator

```rust,editable
# fn main() {
    let num = 1;

    let string: &str = match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Too lazy",
    };

    println!("Number {}", string);
# }
```

## Elvis-Operator

* Fehlerbehandling ist oft viel Schreibarbeit
* Der Elvis-Operator kann Vieles vereinfachen
* Wird ein fehlerhafter oder leerer Wert zurückgegeben

```rust,editable
fn do_something_error_prone(will_fail: bool) -> Result<String, String> {
    match will_fail {
        false => Ok("Ok".to_string()),
        true  => Err("Ups".to_string()),
    }
}

fn main() -> Result<(), String> {
    let result = do_something_error_prone(false)?;
    println!("It's ok? {}", result);

    let result = do_something_error_prone(false)?;
    println!("It's also ok? {}", result);
    Ok(())
}
```

* Die Behandlung von optionalen Werten kann mitunter auch viel Schreibarbeit sein
* Optionale Werte

```rust,editable
fn be_positive(num: i32) -> Option<i32> {
    if num >= 0 {
        Some(num)
    } else {
        None
    }
}

fn check_number(num: i32) -> Option<i32> {
    let result: i32 = be_positive(num)?;
    println!("Number {} is positive", result);
    Some(result)
}

fn main() {
    println!("Call with 1: {:?}", check_number(1));
    println!("Call with 2: {:?}", check_number(2));
    println!("Call with -1: {:?}", check_number(-1));
}
```

## Don't Panic - but Panic

* Es werden verschiede Konstrukte angeboten, um das Program sofort zu beenden
    * `panic!` Makro
    * Für Ergebnis Typen die Funktion `expect`

```rust,editable,should_panic
# fn main() {
    panic!("Something really terrible happens");
# }
```

```rust,editable,should_panic
# fn main() {
    let open_result: Result<String, String> = Ok(String::from("It's ok"));
    let result: String = open_result.expect("An ok");
    println!("Ok? {:?}", result);

    let open_result: Result<String, String> = Err(String::from("It's not ok"));
    let result = open_result.expect("An ok");
    // Wird nicht mehr ausgeführt
    println!("Ok? {:?}", result);
# }
```
