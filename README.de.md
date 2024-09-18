[Deutsch](README.de.md) | [English](README.md)

Ein erweitertes Dashboard, das Daten aus der Logdatei vom Ore Private Server analysiert und in Echtzeit aktualisierte Statistiken und Diagramme für das Ore-Mining anzeigt.

## Funktionen

- **Live-Dashboard**: Zeigt die aktuellen Daten wie Ore-Balance, Anzahl der Miner, Gesamtbelohnungen, erfolgreiche Einsendungen, Durchschnitts-, Min-, und Max-Schwierigkeit sowie Fehlerraten an.
- **Datenvisualisierung**: Diagramm der Verdienstrate über die Zeit mithilfe von `Chart.js`.
- **Dynamische Datenaktualisierung**: Die angezeigten Daten und das Diagramm werden alle 30 Sekunden automatisch aktualisiert.
- **Logdatei-Upload**: Ermöglicht das Hochladen einer Logdatei, die dann analysiert und im Dashboard angezeigt wird.

## Voraussetzungen

Um das Dashboard auszuführen, müssen folgende Software und Bibliotheken installiert sein:

- **Rust**: Rust ist erforderlich, um das Backend zu kompilieren und zu betreiben.
- **Node.js** (optional): Für Frontend-Bibliotheken, wenn du sie manuell installieren möchtest.
- **Chart.js**: Für die grafische Darstellung der Verdienstrate.
- **Bootstrap**: Für das Styling und Layout der HTML-Komponenten.
- **Tera**: Für das Rendern von HTML-Templates im Backend.
- **Actix Web**: Das Web-Framework, das den Server bereitstellt.

## Installation

### 1. Rust und Cargo installieren

Folge den [Anweisungen zur Installation von Rust](https://www.rust-lang.org/tools/install), falls Rust noch nicht installiert ist.

### 2. Repository klonen

```bash
git clone https://github.com/Mistergrow/Ore-Server-Dashboard
cd Ore-Server-Dashboard
cargo run
