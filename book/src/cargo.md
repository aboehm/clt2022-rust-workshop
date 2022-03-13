# Cargo

Zuerst erstellen wir ein Standardprojekt für eine Applikation

```sh
cargo new my-simple-project
```

`cargo` erstellt uns dann folgende Dateien

* Cargo.toml   # Projektbeschreibung mit all seinen Abhängigkeiten 
* src/main.rs  # Unser erstes Programm

Dann können wir das Projekt auch sofort bauen

```sh
cd my-simple-project
cargo build
```

und auch starten

```sh
cargo run
```

Das Result sollte dann so aussehen

```text
Hello, world!
```

Die optimierte Variante wird wie folgt erstellt:

```sh
cargo build --release
```

## Die Cargo.toml

Die [Cargo.toml](Cargo.toml) verwaltet grundsätzliche Dinge zum Projekt, wie:

* Projektdefintion selbst
  * Name
  * Beschreibung
  * Version
  * Rust-Version
  * Links zum Repository
  * Veröffentlichungsregeln
* Paketabhängigkeiten
  * Versionen
  * Features
  * Lokale Quellverzeichnisse / Git Repositories
* Unterprojekte

Für unser erstes Projekt ist dies noch recht klein:

```toml
[package]
name = "my-simple-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Wird ein Build erzeugt, werden die abhängigen `crate` Versionen in der `Cargo.lock` festgehalten. Es wird generell empfohlen Versionen zu pinnen.

Die [Cargo.toml](Cargo.toml) zu diesem Projekt ist dann etwas komplexer.

Die komplette Referenz, könnt ihr auf der [`cargo`-Dokumentationseite](https://doc.rust-lang.org/cargo/reference/manifest.html) finden.

## Cargo für Rust Programme

`cargo` kann nicht nur verwendet werden, um Rust Projekte zu erstellen und zu bauen. Es kann auch dazu verwendet werden, um Applikationen zu installieren.

Beispielsweise um dieses Tutorial wurde `mdbook` verwendet, was mittels

```sh
cargo install mdbook
```

installiert wurde. Die fertige Anwendung wird dann standardmäßig in `~/.cargo/bin` abgelegt.
