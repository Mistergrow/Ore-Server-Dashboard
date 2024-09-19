use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tera::{Context, Tera};
use regex::Regex;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
struct LogData {
    ore_balance: f64,
    miner_count: usize,
    difficulty: u32,
    total_rewards: f64,
    pubkeys: Vec<String>,
    successful_submissions: usize,
    avg_difficulty: f64,
    min_difficulty: u32,
    max_difficulty: u32,
    rewards_over_time: Vec<(String, f64)>,
    error_count: usize,
    top_pubkeys: Vec<(String, usize)>,
    avg_time_between_success: f64,
    failed_submissions: usize,
}

struct AppState {
    template: Tera,
    data: Mutex<Option<LogData>>,
    log_file_path: Mutex<Option<String>>,
}

#[derive(Deserialize)]
struct LogFileInput {
    log_file: String,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();

    if let Some(log_data) = &*data.data.lock().unwrap() {
        ctx.insert("data", &log_data);
    }

    let rendered = data.template.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn upload_log(
    form: web::Form<LogFileInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    let log_file_path = form.log_file.clone();

    // Speichern des Pfads in AppState
    *data.log_file_path.lock().unwrap() = Some(log_file_path.clone());

    // Erste Parsing-Runde
    match std::fs::read_to_string(&log_file_path) {
        Ok(content) => {
            let log_data = parse_logs(&content);
            *data.data.lock().unwrap() = Some(log_data);
            HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish()
        }
        Err(e) => HttpResponse::BadRequest()
            .body(format!("Fehler beim Lesen der Logdatei: {}", e)),
    }
}

async fn update(data: web::Data<AppState>) -> impl Responder {
    if let Some(log_data) = &*data.data.lock().unwrap() {
        HttpResponse::Ok().json(log_data)
    } else {
        HttpResponse::NoContent().finish()
    }
}

fn parse_logs(log_content: &str) -> LogData {
    let mut ore_balance = 0.0;
    let mut difficulty = 0;
    let mut total_rewards = 0.0;
    let mut pubkeys = Vec::new();
    let mut successful_submissions = 0;
    let mut difficulties = Vec::new();
    let mut rewards_over_time = Vec::new();
    let mut error_count = 0;
    let mut pubkey_submissions: HashMap<String, usize> = HashMap::new();
    let mut success_timestamps = Vec::new();
    let mut failed_submissions = 0;

    let re_ore_balance = Regex::new(r"New balance: (?P<balance>\d+\.\d+)").unwrap();
    let re_rewards = Regex::new(r"Earned: (?P<rewards>\d+\.\d+) ORE").unwrap();
    let re_difficulty = Regex::new(r"with  diff (?P<diff>\d+) ").unwrap();
    let re_pubkeys = Regex::new(r"(?P<pubkey>[A-Za-z0-9]{6}\.\.\.[A-Za-z0-9]{4}) found diff: \d+").unwrap();
    let re_success = Regex::new(r"Success!!").unwrap();
    let re_error = Regex::new(r"\x1b\[\d+m ERROR\x1b\[0m").unwrap();
    let re_failure = Regex::new(r"unable to confirm transaction").unwrap();
    let re_timestamp = Regex::new(r"\x1b\[2m(?P<timestamp>.*?)Z\x1b\[0m").unwrap();

    for line in log_content.lines() {
        if let Some(cap) = re_ore_balance.captures(line) {
            ore_balance = cap["balance"].parse::<f64>().unwrap_or(ore_balance);
        }

        if let Some(cap) = re_rewards.captures(line) {
            let reward = cap["rewards"].parse::<f64>().unwrap_or(0.0);
            total_rewards += reward;

            if let Some(cap_time) = re_timestamp.captures(line) {
                let timestamp = &cap_time["timestamp"];
                rewards_over_time.push((timestamp.to_string(), reward));
            }
        }

        if let Some(cap) = re_difficulty.captures(line) {
            let diff = cap["diff"].parse::<u32>().unwrap_or(difficulty);
            difficulties.push(diff);
            difficulty = diff;
        }

        if let Some(cap) = re_pubkeys.captures(line) {
            let pubkey = cap["pubkey"].to_string();
            *pubkey_submissions.entry(pubkey.clone()).or_insert(0) += 1;
            if !pubkeys.contains(&pubkey) {
                pubkeys.push(pubkey);
            }
        }

        if re_success.is_match(line) {
            successful_submissions += 1;

            if let Some(cap_time) = re_timestamp.captures(line) {
                if let Ok(timestamp) = NaiveDateTime::parse_from_str(&cap_time["timestamp"], "%Y-%m-%dT%H:%M:%S%.f") {
                    success_timestamps.push(timestamp);
                }
            }
        }

        if re_error.is_match(line) {
            error_count += 1;
        }

        if re_failure.is_match(line) {
            failed_submissions += 1;
        }
    }

    let miner_count = pubkeys.len();

    // Difficulty Berechnungen
    let avg_difficulty = if !difficulties.is_empty() {
        difficulties.iter().sum::<u32>() as f64 / difficulties.len() as f64
    } else {
        0.0
    };
    let min_difficulty = difficulties.iter().min().cloned().unwrap_or(0);
    let max_difficulty = difficulties.iter().max().cloned().unwrap_or(0);

    // Top 5 Pubkeys ermitteln
    let mut top_pubkeys: Vec<(String, usize)> = pubkey_submissions.into_iter().collect();
    top_pubkeys.sort_by(|a, b| b.1.cmp(&a.1));
    top_pubkeys.truncate(5);

    // Durchschnittliche Zeit zwischen Erfolgen
    let avg_time_between_success = if success_timestamps.len() > 1 {
        let mut total_diff = 0;
        for i in 1..success_timestamps.len() {
            let diff = success_timestamps[i] - success_timestamps[i - 1];
            total_diff += diff.num_seconds();
        }
        total_diff as f64 / (success_timestamps.len() - 1) as f64
    } else {
        0.0
    };

    LogData {
        ore_balance,
        miner_count,
        difficulty,
        total_rewards,
        pubkeys,
        successful_submissions,
        avg_difficulty,
        min_difficulty,
        max_difficulty,
        rewards_over_time,
        error_count,
        top_pubkeys,
        avg_time_between_success,
        failed_submissions,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/templates/**/*").unwrap();

    let data = web::Data::new(AppState {
        template: tera,
        data: Mutex::new(None),
        log_file_path: Mutex::new(None),
    });

    // Starten eines Hintergrundthreads für die periodische Aktualisierung
    let data_clone = data.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));

            let log_file_path_option = data_clone.log_file_path.lock().unwrap().clone();
            if let Some(log_file_path) = log_file_path_option {
                if let Ok(content) = std::fs::read_to_string(&log_file_path) {
                    let log_data = parse_logs(&content);
                    *data_clone.data.lock().unwrap() = Some(log_data);
                }
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            )
            .service(
                web::resource("/upload")
                    .route(web::post().to(upload_log))
            )
            .service(
                web::resource("/update")
                    .route(web::get().to(update))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
