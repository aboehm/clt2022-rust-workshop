# Ownership

## Das grundsätzliche Problem

* Besitz bedeutet, wer allokiert den Speicher und darf eventuell den Speicher verändern
* Ist dies nicht eindeutig geklärt, wird der Kompiler einen Fehler werfen

```rust,editable,should_panic
# fn main() {
    let mut owned = 1;
    let next_owner = &owned;
    println!("owned = {}", owned);           // Leses ist Ok
    owned = 2;                               // Schreiben schlägt fehl
    println!("next_owner = {}", next_owner); // next_owner ist der Besitzer
# }
```

* Speicherbereiche können ausgeborgt werden
* Ist das Verhältnis, wer welchen Bereich ausgeborgt hat und darauf zugreift ungeklärt, wird der Kompiler einen Fehler werfen

```rust,editable
# fn main() {
    let owned = 1;
    let borrowed = &owned;
    println!("owner = {}", owned);
    println!("borrowed = {}", borrowed);
# }
```

* Können Speicherbereiche verändert werden und werden sie ausgeborgt, gibt es auch Probleme

```rust,editable,should_panic
# fn main() {
    let mut owned = 1;
    let borrowed = &owned;

    println!("owner = {}", owned);

    owned += 1; // Hier streikt der Kompiler
    println!("borrowed = {}", borrowed);
# } 
```

* Auch das explizite Ausleihen eine veränderbaren referenzierten Wert, wird nicht funktionieren

```rust,editable,should_panic
# fn main() {
    let mut owned = 1;
    let borrowed1 = &mut owned;
    let borrowed2 = &mut owned;

    println!("owner = {}", borrowed1);

    *borrowed2 += 1;
    println!("borrowed = {}", borrowed2);
# } 
```

## Das unmögliche, möglich machen - veränderliche geteilte Referenzen

* Normalerweise würde der Kompiler veränderbaren geteilten Referenzen streiken
* Gerade bei selbstreferenziellen Datenstrukturen (Graphen) kann dies ein Problem sein
* Dennoch ist dies möglich durch die Anwendung von Typen wie
  * `Cell`: Teilbarer veränderlicher Container
  * `RefCell`: Referenz zu veränderlichen Container

```rust,editable
use std::cell::RefCell;

# fn main() {
    let owned = RefCell::new(1);
    let borrowed1 = owned.clone();
    let borrowed2 = owned.clone();

    println!("owner = {}", owned.borrow());
    println!("borrowed1 = {}", borrowed1.borrow());

    *borrowed2.borrow_mut() += 1; // Explizite Veränderlichkeit anfordern
    println!("borrowed2 = {}", borrowed2.borrow());

# }
```

* Bei multithreaded Anwendungen können diese Typen nicht verwendet werden, allerdings gibt es Alternativen wie:
    * `Channels`: Simple Queue mit Sender und Empfänger
    * `Mutex`: Mutex zum temporären Sperren von Ressourcen

