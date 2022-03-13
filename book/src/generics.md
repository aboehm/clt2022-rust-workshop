# Generics

* Rust übernimmt für Generics ähnliche Konstrukte wie bei C++ oder Java
* Auch hier spielt die Typeninferenz wieder eine Rolle, da die konkreten Typen aus dem Zusammenhang geschlußfolgert werden können
* Konkrete  müssen nicht zur Laufzeit instanziiert und dann konvertiert werden

```rust,editable
struct MyGenericStruct<T> {
    items: Vec<T>,
}

fn main() {
    let struct_with_u32: MyGenericStruct<u32> = MyGenericStruct {
        items: vec![],
    };

    let struct_with_u16 = MyGenericStruct::<u16> {
        items: Vec::new()
    };

    let struct_with_u8 = MyGenericStruct {
        items: Vec::<u8>::new()
    };
}
```

* Generische Typen können konkrete Implementierungen nur für einen Typ besitzen

```rust,editable
struct MyGenericStruct<T> {
    a: T,
    b: T,
}

impl MyGenericStruct<i32> {
    fn sum(&self) -> i32 {
        self.a + self.b
    }
}

fn main() {
    // Für i32 liegt eine Implementierung vor, u32 Typen können nicht verarbeitet werden
    MyGenericStruct { a: 1, b: 2 }.sum();
}
```

* Auch für generische Typen können generische Implementierungen verwendet werden
* Damit er Kompiler dies richtig umsetzen kann, müssen wir ggf. Randbedingungen an unsere Generics stellen, welche Traits umgesetzt werden müssen

```rust,editable
struct MyGenericStruct<T> {
    a: T,
    b: T,
}

impl<T> MyGenericStruct<T> where T: std::ops::Add<Output=T>, T: std::marker::Copy {
    fn sum(&self) -> T {
        self.a + self.b
    }
}

fn main() {
    MyGenericStruct { a: 1i32, b: 2i32 }.sum();
    MyGenericStruct { a: 3u32, b: 4u32 }.sum();
    MyGenericStruct { a: 5i8, b: 6i8 }.sum();
    MyGenericStruct { a: 7.1, b: 8.2 }.sum();
}
```

