# Ore Private Server Dashboard

Ein erweitertes Dashboard, das Daten aus einer Logdatei analysiert und in Echtzeit aktualisierte Statistiken und Diagramme für ein Ore-Mining-Netzwerk anzeigt.

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

1. **Rust und Cargo installieren** (falls nicht bereits installiert):
   - Folge den [Anweisungen zur Installation von Rust](https://www.rust-lang.org/tools/install).

2. **Repository klonen**:

   ```bash
   git clone https://github.com/username/erweitertes-dashboard.git
   cd erweitertes-dashboard

