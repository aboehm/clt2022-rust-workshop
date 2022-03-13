# Traits

* Traits definieren ein gemeinsames Verhalten am ehsten Vergleichbar mit Interfaces
* Datentypen implementieren Traits
* Traits können ebenfalls Randbedingungen für Generics sein, um auf notwendige Implementierungen zu verweisen

```rust
pub trait Noise {
    fn noise(&self) -> String;
}

fn make_some_noise<T>(something: &T) -> String where T: Noise {
    something.noise()
}

struct Cat;

impl Noise for Cat {
    fn noise(&self) -> String {
        "Moew".to_string()
    }
}

fn main() {
    let cat = Cat { };
    println!("Cat does {}", cat.noise());
}
```

## Einfach erweitern

* Viele Standardmethoden werden über Implementierung von Traits realisiert
    * Bereitstellung von Debug-Informationen mittels [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
    * String-Umwandlung mittels [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html)
    * formatierte Ausgabe mittels [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
    * Typenkonvertierung mittels [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) und [`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html)

```rust,editable
use std::fmt;
use std::convert::From;

struct Cat {
    name: String,
}

struct Dog {
    name: String,
}

impl fmt::Display for Cat {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "A cat named {}", self.name)
    }
}

impl fmt::Display for Dog {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "A dog named {}", self.name)
    }
}

impl From<Cat> for Dog {
    fn from(cat: Cat) -> Self {
        Dog { name: cat.name }
    }
}

fn main() {
    let cat = Cat { name: "Mitze".to_string(), };
    println!("What is it? {}", cat);
    let dog: Dog = cat.into();
    println!("What is it? {}", dog);
}

```
