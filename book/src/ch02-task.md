# Übungsaufgabe

## Ziel

* Modelliert ein Freundschaftsnetzwerk
* Eine `struct Friend` soll lediglich einen namen enthalten
* Am Ende soll es möglich sein, eine Dreiecks-Bezeihung zu modellieren
* Bezieherungen zu Freunden sollen mittels Referenzen dargestellt werden
* Über einen Freund kann ich zu nächsten Freund navigieren

## Hilfestellung

* Ihr könnt für Listen den Typ [`Vec<Friend>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) verwenden
* Dokumentation von [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)
* Dokumentation von [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

```rust,editable
#![allow(unused_imports)]
#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

struct Friend {
    name: String,
    // TODO: Liste mit Freunden definieren
}

impl Friend {
    fn new(name: &str) -> Self {
        Friend {
            name: name.to_string(),
            // TODO: Liste mit Freunden definieren
        }
    }
}

fn main() {
    let a = Friend::new("Zero Cool");
    let b = Friend::new("Acid Burn");
    let c = Friend::new("Cereal Killer");

    // TODO: Freundesnetzwerk erstellen
}
```

## Musterlösung

* [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=73e6ff269b1eb0c5085a34eb9e3be79c)
