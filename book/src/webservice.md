# Demo Webservice

## Ziel

* Einfacher Service, der uns eine REST-artige Schnittstelle anbietet
    * Anlegen eines Wertes
    * Verändern eines Wertes
    * Löschen eines Wertes
    * Umsetzung mit [`rocket`](https://rocket.rs)
* Datenbank soll unsere Daten halten
    * Zur Vereinfachung SQLite
    * Daten werden nur im Speicher gehalten
    * Umsetzung mit [`sqlx`](https://github.com/launchbadge/sqlx)

## Vorgehen

* Erstellen neues crate mit `cargo`

```sh
cargo new webservice
```

* Abhängigkeiten in der Cargo.toml definieren

```toml
[package]
name = "webservice"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rocket = { version = "0.5.0-rc.1", features = ["json"] }
sqlx = { version = "0.5", features = ["sqlite", "runtime-tokio-native-tls", "uuid", "chrono"] }
uuid = { version = "0.8.2", features = ["serde", "v4"] }
async-trait = "0.1.52"
tokio = "*"
serde = "*"
serde_json = "*"
serial_test = "0.6.0"
```

* Rumpffunktionen für `rocket` anlegen

```rust,compile_fail
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hallo CLT 2022"
}


fn build_server() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index])
}

#[rocket::main]
async fn main() {
    build_server()
        .launch()
        .await
        .unwrap();
}

#[cfg(test)]
mod test {
}
```
