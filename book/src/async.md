# Asynchrone Verarbeitung

* Selbst heutige Mikrokontroller sind Mehrprozessorsystem
* Die parallele Ausführung von Code spart oft nicht nur Zeit sondern auch Energie
* An vielen Stellen kann auch ein Programm auf Daten warten, d.h. Aufgaben können aufgeschoben werden, wenn sie notwendig sind
* Rust nutzt hier für die `async`/`await` Syntax
    * eine Funktion ist `async`hron
    * auf der Ergebnis einer asynchronen Funkten muss gewartet werden (`await`)
* In der Vergangenheit wurden nebenläufige Funktionen oft mit Threads des Betriebssystems umgesetzt
    * relativ schwergewichtig sowohl für CPU als auch Arbeitsspeicher
    * fast überall einsetzbar
* Durch die Integration in der Sprache, kann der Kompiler besser darüber entscheiden
* asynchrone Funktionen können Rust in einer Runtime ausgeführt werden
    * Die Runtime entscheidet, wann welcher Funktion ausgeführt wird und verwaltet die Ausführung über mehrere Threads
    * Eine klassische Ausführung über Threads mit Rusts selbst ist möglich
    * Bekannte Vertreter für Runtimes sind `tokio` und `async-std`
* async Traits werden zurzeit nicht von Rust nativ unterstützt, dafür kann aber das [async-trait](https://github.com/dtolnay/async-trait) crate genutzt werden

## Probleme von Nebenläufigkeit

### Verwendung von gemeinsamen Speicher

* Nur lesen ist einfach
* Wird Speicher verändert müssen die Zugriffe verwaltet werden
* Verwaltung kostet Leistung

Lösung:

* Um dies besser zu implementiert, kommt uns der Borrow-Checker zu Hilfe
* Explizter Ausdrücke für veränderliche Speicherbereihe
* Borrow-Konzept markiert Bereiche die veränderlich/unveränderlich sind
* Shared-Nothing-Konzepte sollen bevorzugt werden, hierfür können bspw. `Channel` verwendet werden, um zwischen Funktionen Speicherbereiche zu "versenden"
* Synchronisierungsobjekte wie `Mutex` oder Verwendung von Objekten, die `Send` und `Sync` Traits implementieren

### Lebenszeit von Speicher

* Solange Speicher von Funktionen nur auf dem Stack stattfindet ist es einfacher
* Speicherbereiche müssen allokiert werden und ggf. ausgetauscht werden
* Mehrere getrennte Funktionen greifen unabhängig auf Speicher zu

Lösung:

* Referenzzähler für eine asynchrone Verarbeitung, da Nutzung von Objekten nicht mehr im Voraus planbar ist
* statische Lebenszeit für Objekte

## Beispiel

```rust,compile_fail,noplayground
{{#include ../examples/asynchandling.rs}}
```
