# Selbstdefinierte Datentypen

* Datentypen werden über das Schlüsselwort `struct` definiert
* Datentypen können auch keinerlei Inhalt haben
* Wrapper für die Kappselung von anderen Datentypen
* Implementierungen für `structs`
* Mittels Macros können verschiedene Eigenschaften hinzugefügt werden, bspw.
    * Kopierverhalten
    * Bereitstellung einer Debug-Darstellung
    * Sonstige Erweiterungen für bspw. Datenbankeigenschaften


```rust,editable
#[derive(Debug)]
struct OneValue {
    value: u32,
}

#[derive(Debug)]
struct Empty { }

#[derive(Debug)]
struct Wrapper(u32);

impl OneValue {
    fn new() -> Self {
        OneValue {
            value: 0,
        }
    }

    fn do_something(&self) {
        println!("My value is {}", self.value);
    }

    fn change_something(&mut self) {
        self.value += 1;
    }
}

impl Empty {
    fn do_something_static() {
        println!("A voice from the void");
    }
}

fn main() {
    let mut one_value = OneValue::new();
    one_value.do_something();
    one_value.change_something();

    let empty = Empty { };
    Empty::do_something_static();

    let wrapped = Wrapper (1);

    // Ausgabe über Debug-Macro
    println!("one_value = {:?}", one_value);
    println!("empty = {:?}", empty);
    println!("wrapped = {:?}", wrapped);
}
```

* Enums orienteren sich an der C-Notation
* Enums können aber mit weiteren Strukturen erweitert werden
* Auch Enums können mit Macros erweitert werden

```rust,editable
// Unterstützung für Gleichheits-Operator
#[derive(Debug,PartialEq)]
enum Status {
    Ok,
    GenericError(String),
    ApplicationError { prio: u32, cause: String },
}

impl Status {
    fn am_i_ok(&self) -> bool {
        *self == Status::Ok
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let result: Status = Status::Ok;
    println!("Ok {:?}", result);
    println!("Ok? {:?}", result.am_i_ok());

    let result = Status::GenericError(String::from("Something went wrong"));
    println!("{:?}", result);

    let result = Status::ApplicationError { prio: 1, cause: String::from("Something more specific") };
    println!("{:?}", result);
}
```
