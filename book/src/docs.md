# Dokumentation

* Vollintegrierte Dokumentation
* Erstellung mit `cargo doc`
* Paketdokumentation können auf [docs.rs](https://docs.rs) gefunden werden
* Dokumentation von alle genutzten `crates` werden standardmäßig inkludiert
* Rust Code, der dokumentiert wird, wird für Libraries automatisch getest

```rust
/// Gibt das Ergebnis einer vorzeichenbehafteten Addition mit einem 16-Bit Integer zurück
///
/// # Argumente
///
/// * `a` - 1. Summand
/// * `b` - 2. Summand
///
/// # Beispiele
///
/// ```rust
/// add_signed_int16(1, 2);
/// ```
fn add_signed_int16(a: i16, b: i16) -> i16 {
    a + b
}

# fn main() { }
```

## mdbook

* Dieser Workshop wurde mittels [mdbook](https://github.com/rust-lang/mdBook)
* Alle Code läuft dabei durch einen Tests, wenn nicht besonders markiert
* Nutzt Standard Markdown mit ein paar Erweiterungen

* Installtion

```sh
cargo install mdbook
```

* Lokal anschauen

```sh
mdbook serve
```

* HTML erzeugen

```sh
mdbook build
```

* Testen

```sh
mdbook test
```
