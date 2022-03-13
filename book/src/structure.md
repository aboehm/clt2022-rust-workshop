# Crates, Module, Sichtbarkeit

* Jedes Cargo Projekt ist automatisch ein `crate`

```rust,editable
struct MyStruct;

fn main() {
    let a = crate::MyStruct;
}
```

* Applikationen, wenn nicht anders definiert, nutzen meist die Datei `main.rs`
* Libraries, nutzen `lib.rs`

# Module und Sichtbarkeit

* Crates können Applikationen oder Libraries für Funktionen, Datenstrukturen, Makros, ... sein
* Module bilden Namensräume die ganze oder nur Teile daraus über `use` importiert werden können
* Nur auf Datentypen, Funktionen und Module, die als `pub` deklariert sind, kann von außen zugegriffen werden


```rust,editable
mod my_module {
    pub struct MyPublicStruct { }

    impl MyPublicStruct {
        pub fn public_func(&self) { }

        fn private_func() { }
    }

    struct MyPrivateStruct { }

    impl MyPrivateStruct {
        fn func(&self) { }
    }
}

use my_module::{MyPublicStruct};

fn main() {
    MyPublicStruct { }.public_func();
    // alternativ, kann aber Probleme mit Traits machen
    my_module::MyPublicStruct { }.public_func();
}
```

* Module können auch ohne weiteres in andere Dateien ausgelagert werden
* Hierfür muss nur der Modulname deklariert werden und der Kompiler bezieht die Datei mit ein

```rust,compile_fail,noplayground,should_panic
pub mod my_module;

use my_module::{MyPublicStruct};

fn main() {
    MyPublicStruct { };
}
```

```rust,compile_fail,noplayground,should_panic
// my_module.rs oder my_module/mod.rs
pub struct MyPublicStruct { }
```
