# Ore Private Server Dashboard

[Deutsch](README.de.md) | [English](README.md)

An advanced dashboard that analyzes data from the Ore Private Server log file and displays real-time updated statistics and charts for Ore mining.

## Features

- **Live Dashboard**: Displays current data such as Ore balance, number of miners, total rewards, successful submissions, average, minimum, and maximum difficulty, as well as error rates.
- **Data Visualization**: Shows earnings rate over time using `Chart.js`.
- **Dynamic Data Updates**: Automatically updates data and charts every 30 seconds.
- **Log File Upload**: Allows the upload of a log file, which is then analyzed and displayed on the dashboard.

## Requirements

To run the dashboard, the following software and libraries are needed:

- **Rust**: Required to compile and run the backend.
- **Node.js** (optional): For frontend libraries, if you choose to install them manually.
- **Chart.js**: For visualizing the earnings rate.
- **Bootstrap**: For styling and layout of HTML components.
- **Tera**: For rendering HTML templates in the backend.
- **Actix Web**: The web framework that serves the application.

## Installation

### 1. Install Rust and Cargo

Follow the [Rust installation instructions](https://www.rust-lang.org/tools/install) if Rust is not already installed.

### 2. Clone the repository

```bash
git clone https://github.com/Mistergrow/Ore-Server-Dashboard
cd Ore-Server-Dashboard
cargo run
don´t close the app, open browser and enter 127.0.7.1:8080
on top of the page enter path to your lofile from srv (target/release/log/) press load
