# Speicherverwaltung

## Clone and Copy

* Standardmäßig sind bei eigensdefinierten Datentypen keine Kopierfunktionen vorhanden
* Für diesen Zweck definiert Rust zwei Traits
    * `Copy`: Passiert implizit und führt eine bitweise Kopie der Variable durch
    * `Clone`: Muss explizt angefordert werden und k Bitweises kopieren erzeugt eine vollständige Kopie
    * Jedes Objekt, welches Copy implementiert, implementiert auch Clone
    * Aber nicht jedes Objekt, welches Clone implementiert, implementiert auch Copy
* `structs` können beispielsweise nicht duplizierbare Bestandteile haben
* In den meisten Fällen reicht jedoch ein Standardverhalten vorzuschreiben
* Für diesen Zwecks gibt es den `Clone`-Trait
* Um automatisch eine gültige Implementierung zu nutzen, kann das `derive`-Makro mit `Clone` verwendet werden

```rust
#[derive(Clone, Debug)]
struct CloneableItem { value: u32, }

#[derive(Clone, Copy, Debug)]
struct CopyableItem { value: u32, }

# fn main() {
    let item_cloneable = CloneableItem { value: 1 };
    let item_cloned = item_cloneable.clone();

    let item_copyable = CopyableItem { value: 2 };
    let item_copied1 = item_copyable;
    let item_copied2 = item_copyable;

    println!("Pointer:");
    println!("Cloneable          {:p}", &item_cloneable);
    println!("Cloned             {:p}", &item_cloned);
    println!("Copyable           {:p}", &item_copyable);
    println!("Copied 1st         {:p}", &item_copied1);
    println!("Copied 2nd         {:p}", &item_copied2);
}
```

## Speicherobjekte auf dem Heap

* Nicht immer ist es möglich Speicher zu auf dem Stack zu verwalten
  * Speicherbereich kann zu begrenzt sein
  * Instanzen müssen an vielen Stellen referenziert und verändert werden
* Für diesen Zweck gibt es den Box-Typ
* Bei der Instanziierung wird ein Speicherbereich auf dem Heap allokiert
* Wird die `Box` nicht mehr gebraucht, dann wird der Bereich wieder freigegeben

```rust,editable
# fn main() {
    // Object auf dem Heap anlegen
    let heap_var: Box<[u32]> = Box::new([42u32; 4096]);
    // Dereferenzierung für einen Zugriff
    println!("heap_var = {}", heap_var[0]);

# }
```

## Dynamische Listen

* Gerade bei veränderlichen Listen sind die statischen Arrays hinderlich
* Hierfür kann der generische Datentyp `Vec` genutzt werden

```rust,editable
# fn main() {
    let integer_array = vec![1, 2, 3];
    // explizt typisiert
    let integer_array: Vec<i32> = vec![1, 2, 3];

    // Für die Veränderung einer Liste, muss diese wieder mit `mut` gekennzeichnet werden
    let mut list = vec![];
    list.push(1);
    list.push(2);
    println!("list = {:?}", list);
# }
```


