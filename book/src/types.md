# Einfache Datentypen, Variablen und Funktionen

## Variablendfinition

* Variablen sind standardmäßig unveränderbar und müssen mit dem Keyword `mut` markiert werden
* An vielen Stellen hilft es die Variable einfach neu zu definieren

```rust,compile_fail,editable
# fn main() {
    let a = 1;
    // Hier scheitert der Kompiler
    a = 2;

    // mut definiert eine Variable als veränderbar
    let mut b = 1;
    // damit ist dies möglich
    b = 2;

    // Eine Neudefinition ist ebenfalls möglich
    let c = 1;
    let c = 2;

    println!("a={}", a);
    println!("b={}", b);
    println!("c={}", c);
# }
```

## Typendeklaration und -inferenz

* Viele Typen werden automatisch erschloßen
* Für ganze Zahlen wird automatisch ein 32-Bit signed Integer angenommen
* Für rationale Zahlen wird eine 32-Bit Fließkommazahl angenommen
* Explizite Typisierung meist nicht notwendig, da der Kompiler die Typen erschließen kann

```rust,editable
# fn main() {
    let integer: u16 = 1;
    let float: f32 = 1.23;
    let boolean: bool = false;
    let a_char: char = 'X';
    let void: () = ();
    let tuple: (u8, char) = (1, 'a');
    let simple_string: &str = "a char array";

    // Standardausgaben der einfachen Datentypen
    println!("integer = {}", integer);
    println!("float = {}", float);
    println!("char = {}", a_char);
    println!("bool = {}", boolean);
    println!("simple string = {}", simple_string);

    // Ein komplexer String
    let complex_string: String = String::from("a more complex string");
    println!("complex_string = {}", complex_string);

    // Für manche ist die Ausgabe nicht verfügbar, dafür aber eine Debug-Ausgabe
    println!("void = {:?}", void);
    println!("tuple = {:?}", tuple);

    let a_huge_integer: i128 = 1_000_000_000_000_000_000_000_000_000_000;
    let a_signed_integer = -1;
    let a_precise_float: f64 = 1.055736284329874932749329479237492374;
    println!("a_huge_integer = {:?}", a_huge_integer);
    println!("a_signed_integer = {:?}", a_signed_integer);
    println!("a_precise_float = {:?}", a_precise_float);
# }
```

* Verschiede größen sind vordefiniert wie unsigned/signed Integer für 8,16,32,64,128 oder Gleitkommatype für 32,64 Bit.

## Funktionen

* Funktionen müssen typisiert werden
* die letzte Zeile gibt den Funktionswert zurück
* ist der letzte Zeile durch ein Semikolon beendet, ist der Typ leer

```rust,editable
fn void_func() -> () {
    println!("void function");
}


fn another_void_func(value: &str) {
    println!("another void function: {}", value);
}

fn all_together(value: &str) -> usize {
    value.len()
}

fn main() {
    void_func();
    another_void_func("Hello CLT");
    println!("String length {}", all_together("Hello CLT"));
}
```

* Lambdas sind ebenfalls möglich

```rust
fn main() {
    let lambda = |x, y| x*y;
    println!("Lambda result {}", lambda(3, 4));
}
```


## Typenkonvertierung

* Variablen können explizit typisiert werden
* Der Kompiler gibt ein Fehler aus, wenn
  * ein Typ nicht bekannt ist
  * mehrere mögliche Interpretationen gibt
  * der Quell- und Ziel Typ nicht zusammen passen
* Gerade um den Kompiler bei der richtigen Implementierungswahl zu helfen, ist es of einfacher eine Variablendefintion zu Typisierungen
* Vielen Konvertieren können auch über Traits umgesetzt werden, dazu später mehr

```rust, editable
# fn main() {
    // Explizite Definition von Variablentypen
    let unsigned_16bit_integer = 1_u16; // Am Wert
    let signed_16bit_integer: i16 = -1; // An der Variablendeklaration

    // Funktionsdefition mit den spezifischen Datyentypen
    fn mul(a: i32, b: i32) -> i32 {
        a * b // ein expliztes return ist nicht notwendig
    }

    // Funktionsdefintion mit leeren Rückgabetyp, dies ist äquivalent
    // fn do_useless_add(a: i32, b: i32) -> () { ... }
    fn do_useless_add(a: i32, b: i32) {
        // Ohne Semikolon würde diese Zeile ein Fehler erzeugen
        a * b;
    }

    let a_result = mul(signed_16bit_integer as i32, unsigned_16bit_integer as i32);
    println!("Result of {} * {} = {}",
        signed_16bit_integer, unsigned_16bit_integer, a_result);
# }
```

## Referenzen

* Die meist verbreiteste Referenz dürfte die zu einem einfachen String sein
* Referenzen müssen verwendet werden, wo keine explizte Datentypgröße vorhanden ist bspw. bei generische Datentypen
* Für referenzierte Werte gelten ebenfalls die Regeln für Veränderlichkeit, d.h. per Standard sind die referenzierte Daten unveränderbar

```rust,editable
# fn main() {
    let a_string: &str = "Hello world";
    println!("a_string={}", a_string);

    let a_u32 = 2022;
    let ref_to_u32: &u32 = &a_u32;
    // Zugriff auf den Wert mittels Dereferenzierung
    println!("ref_to_u32={}", *ref_to_u32);

    // Integer ist veränderlich
    let mut a_mut_u32 = 2022;
    // Die Referenz selbst nicht, aber der referenzierte Wert
    let ref_to_mut_u32 = &mut a_mut_u32;
    // Damit kann auch der referenzierte Wert verändert werden
    *ref_to_mut_u32 = 42;

    println!("ref_to_mut_u32={}", *ref_to_mut_u32);

    // Referenzen an sich dürfen ebenfalls verändert werden
    let mut mut_ref_to_u32 = &1;
    mut_ref_to_u32 = &2;
    println!("mut_ref_to_u32={}", *mut_ref_to_u32);
# }
```

* Wird eine Referenz genutzt, wird automatisch der Borrow Checker involviert und es stellt sich die Frage des Ownerships

## Arrays

* Arrays werden als `slices` bezeichnet
* Rust folgt bei Zeichenketten der C++ Definition, es gibt einfache Zeichenketten ähnlich char-Arrays und komplexere Strings
* *Hinweis*: einfache Strings und Arrays können je nach Definition eine unbekannte Größe haben und müssen als Referenz behandelt werden

```rust,editable
# fn main() {
    let integer_array: [u8; 3] = [1u8, 2, 3];

    // Auch hier können wir nur eine Debug-Ausgabe durchführen
    println!("integer_array = {:?}", integer_array);
    println!("2nd element of integer_array = {}", integer_array[1]);
# }
```

* Durch die statische Größe, kann der Kompiler auch Range Checks vorab durchführen
* Ein Zugriff außerhalb des Bereichs führt zu einem Kompilierfehler

```rust,editable,compile_fail
# fn main() {
    let integer_array: [u8; 3] = [1u8, 2, 3];
    integer_array[3];
# }
```
