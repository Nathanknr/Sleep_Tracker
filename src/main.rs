use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use chrono::{Local, Days};

/// Represents a single sleep tracking entry with all relevant metrics
#[derive(Debug, Serialize, Deserialize)]
struct Answer {
    id: i64,
    entry_date: String,
    bedtime: String,
    wake_time_target: String,
    wake_time_actual: String,
    notes: String,
    nap_minutes: i32,
    sleep_quality_score: i32,
    total_sleep_minutes: i32,
    awake_minutes: i32,
    sleep_latency_minutes: i32,
    wake_count: i32,
}

/// Custom error type for handling user exits
#[derive(Debug)]
enum AppError {
    UserExit,
    Database(rusqlite::Error),
    Io(io::Error),
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

/// Data Access Object for managing sleep tracking entries in SQLite database
struct AnswerDao {
    conn: Connection,
}

impl AnswerDao {
    fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(AnswerDao { conn })
    }

    fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS answer (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entry_date TEXT,
                bedtime TEXT,
                wake_time_target TEXT,
                wake_time_actual TEXT,
                nap_minutes INTEGER,
                sleep_quality_score INTEGER,
                total_sleep_minutes INTEGER,
                awake_minutes INTEGER,
                sleep_latency_minutes INTEGER,
                wake_count INTEGER,
                notes TEXT
            )",
            [],
        )?;
        Ok(())
    }

    fn insert(
        &self,
        entry_date: &str,
        bedtime: &str,
        wake_target: &str,
        wake_actual: &str,
        nap: i32,
        quality: i32,
        total: i32,
        awake: i32,
        latency: i32,
        count: i32,
        notes: &str,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO answer (
                entry_date, bedtime, wake_time_target, wake_time_actual,
                nap_minutes, sleep_quality_score, total_sleep_minutes,
                awake_minutes, sleep_latency_minutes, wake_count, notes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                entry_date, bedtime, wake_target, wake_actual,
                nap, quality, total, awake, latency, count, notes
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    fn list_all(&self) -> Result<Vec<Answer>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, entry_date, bedtime, wake_time_target, wake_time_actual,
                    nap_minutes, sleep_quality_score, total_sleep_minutes,
                    awake_minutes, sleep_latency_minutes, wake_count, notes
             FROM answer ORDER BY id DESC",
        )?;

        let answer_iter = stmt.query_map([], |row| {
            Ok(Answer {
                id: row.get(0)?,
                entry_date: row.get(1)?,
                bedtime: row.get(2)?,
                wake_time_target: row.get(3)?,
                wake_time_actual: row.get(4)?,
                nap_minutes: row.get(5)?,
                sleep_quality_score: row.get(6)?,
                total_sleep_minutes: row.get(7)?,
                awake_minutes: row.get(8)?,
                sleep_latency_minutes: row.get(9)?,
                wake_count: row.get(10)?,
                notes: row.get(11)?,
            })
        })?;

        let mut answers = Vec::new();
        for answer in answer_iter {
            answers.push(answer?);
        }
        Ok(answers)
    }

    fn get_recent_entries(&self, days: i32) -> Result<Vec<Answer>> {
        let cutoff_date = Local::now()
            .checked_sub_days(Days::new(days as u64))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();

        let mut stmt = self.conn.prepare(
            "SELECT id, entry_date, bedtime, wake_time_target, wake_time_actual,
                    nap_minutes, sleep_quality_score, total_sleep_minutes,
                    awake_minutes, sleep_latency_minutes, wake_count, notes
             FROM answer
             WHERE entry_date >= ?1
             ORDER BY entry_date DESC",
        )?;

        let answer_iter = stmt.query_map([cutoff_date], |row| {
            Ok(Answer {
                id: row.get(0)?,
                entry_date: row.get(1)?,
                bedtime: row.get(2)?,
                wake_time_target: row.get(3)?,
                wake_time_actual: row.get(4)?,
                nap_minutes: row.get(5)?,
                sleep_quality_score: row.get(6)?,
                total_sleep_minutes: row.get(7)?,
                awake_minutes: row.get(8)?,
                sleep_latency_minutes: row.get(9)?,
                wake_count: row.get(10)?,
                notes: row.get(11)?,
            })
        })?;

        let mut answers = Vec::new();
        for answer in answer_iter {
            answers.push(answer?);
        }
        Ok(answers)
    }
}

fn main() {
    match run_app() {
        Ok(()) => println!("Program completed successfully."),
        Err(AppError::UserExit) => println!("\nGoodbye! 👋"),
        Err(AppError::Database(e)) => eprintln!("Database error: {}", e),
        Err(AppError::Io(e)) => eprintln!("IO error: {}", e),
    }
}

fn run_app() -> std::result::Result<(), AppError> {
    let dao = AnswerDao::new("tracker.sqlite")?;
    dao.create_table()?;

    println!("--- Sleep Tracker ---");
    println!("💡 Tip: Type 'exit', 'quit', or 'q' at any time to stop the program\n");
    println!("1. Enter new sleep data");
    println!("2. View sleep efficiency averages");
    print!("Choose option (1 or 2): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    if is_exit_command(choice.trim()) {
        return Err(AppError::UserExit);
    }

    match choice.trim() {
        "1" => enter_sleep_data(&dao)?,
        "2" => show_efficiency_averages(&dao)?,
        _ => {
            println!("Invalid choice. Defaulting to entering new sleep data.");
            enter_sleep_data(&dao)?;
        }
    }

    Ok(())
}

fn is_exit_command(input: &str) -> bool {
    let input_lower = input.to_lowercase();
    matches!(input_lower.as_str(), "exit" | "quit" | "q" | "stop")
}

fn enter_sleep_data(dao: &AnswerDao) -> std::result::Result<(), AppError> {
    let entry_date = Local::now().format("%Y-%m-%d").to_string();

    println!("\n--- Enter Sleep Data for {} ---", entry_date);
    println!("💡 Reminder: Type 'exit', 'quit', or 'q' at any prompt to stop");

    let bedtime = get_input("What time did you go to bed last night? (HH:MM): ")?;
    let wake_target = get_input("What time did you plan to wake up? (HH:MM): ")?;
    let wake_actual = get_input("What time did you actually get out of bed? (HH:MM): ")?;
    let nap = get_number_input("How many minutes did you nap yesterday? (0 if none): ")?;
    let quality = get_number_input("Rate your sleep quality (1-5): ")?;
    let total_sleep_str = get_input("Total sleep time? (HH:MM): ")?;
    let awake = get_number_input("Minutes awake during night: ")?;
    let latency = get_number_input("Minutes to fall asleep: ")?;
    let wake_count = get_number_input("How many times did you wake up: ")?;
    let notes = get_input("Any additional notes (optional): ")?;

    let total_min = to_minutes(&total_sleep_str);
    let efficiency = calc_efficiency(total_min, awake, latency);

    let id = dao.insert(
        &entry_date,
        &bedtime,
        &wake_target,
        &wake_actual,
        nap,
        quality,
        total_min,
        awake,
        latency,
        wake_count,
        &notes,
    )?;

    println!("\n✓ Sleep data saved successfully!");
    println!("Entry ID: {}", id);
    println!("Sleep Efficiency: {:.1}%", efficiency);
    println!("Total Sleep: {:.1} hours", total_min as f64 / 60.0);

    Ok(())
}

fn show_efficiency_averages(dao: &AnswerDao) -> std::result::Result<(), AppError> {
    println!("\n--- Sleep Efficiency Averages ---");

    let entries_7_days = dao.get_recent_entries(7)?;
    if !entries_7_days.is_empty() {
        let avg_efficiency_7 = calculate_average_efficiency(&entries_7_days);
        let avg_quality_7    = calculate_average_quality(&entries_7_days);
        let avg_sleep_7      = calculate_average_sleep_hours(&entries_7_days);
        let avg_sleep_nap_7  = calculate_average_total_sleep_with_nap(&entries_7_days);

        println!("Last 7 days ({} entries):", entries_7_days.len());
        println!("  Average Sleep Efficiency:      {:.1}%", avg_efficiency_7);
        println!("  Average Sleep Quality:         {:.1}/5", avg_quality_7);
        println!("  • Avg Night-only Sleep:        {:.1} hours", avg_sleep_7);
        println!("  • Avg Total Sleep (incl. naps):{:.1} hours", avg_sleep_nap_7);
    } else {
        println!("Last 7 days: No data available");
    }

    let entries_30_days = dao.get_recent_entries(30)?;
    if !entries_30_days.is_empty() {
        let avg_efficiency_30 = calculate_average_efficiency(&entries_30_days);
        let avg_quality_30    = calculate_average_quality(&entries_30_days);
        let avg_sleep_30      = calculate_average_sleep_hours(&entries_30_days);
        let avg_sleep_nap_30  = calculate_average_total_sleep_with_nap(&entries_30_days);

        println!("\nLast 30 days ({} entries):", entries_30_days.len());
        println!("  Average Sleep Efficiency:      {:.1}%", avg_efficiency_30);
        println!("  Average Sleep Quality:         {:.1}/5", avg_quality_30);
        println!("  • Avg Night-only Sleep:        {:.1} hours", avg_sleep_30);
        println!("  • Avg Total Sleep (incl. naps):{:.1} hours", avg_sleep_nap_30);
    } else {
        println!("Last 30 days: No data available");
    }

    println!("\nPress Enter to continue or type 'exit' to stop...");
    let _ = get_input("")?;
    Ok(())
}

fn get_input(prompt: &str) -> std::result::Result<String, AppError> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    if is_exit_command(&trimmed) {
        return Err(AppError::UserExit);
    }
    Ok(trimmed)
}

fn get_number_input(prompt: &str) -> std::result::Result<i32, AppError> {
    loop {
        let input = get_input(prompt)?;
        if input.is_empty() {
            return Ok(0);
        }
        match input.parse::<i32>() {
            Ok(n) => return Ok(n),
            Err(_) => println!("Please enter a valid number (or 'exit' to quit)."),
        }
    }
}

fn to_minutes(hhmm: &str) -> i32 {
    match hhmm.split(':').collect::<Vec<&str>>().as_slice() {
        [h, m] => h.parse().unwrap_or(0) * 60 + m.parse().unwrap_or(0),
        _ => 0,
    }
}

fn calc_window(bedtime: &str, wake_target: &str) -> i32 {
    let bt = to_minutes(bedtime);
    let mut wt = to_minutes(wake_target);
    if wt <= bt { wt += 24 * 60; }
    wt - bt
}

fn round_to_2_sig_figs(value: f64) -> f64 {
    if value == 0.0 { return 0.0; }
    let scale = value.abs().log10().floor() as i32;
    let factor = 10.0f64.powi(1 - scale);
    (value * factor).round() / factor
}

fn calc_efficiency(sleep: i32, awake: i32, latency: i32) -> f64 {
    let tib = sleep + awake + latency;
    if tib == 0 { return 0.0; }
    round_to_2_sig_figs(sleep as f64 / tib as f64 * 100.0)
}

fn calculate_average_efficiency(entries: &[Answer]) -> f64 {
    if entries.is_empty() { return 0.0; }
    let total: f64 = entries.iter()
        .map(|e| calc_efficiency(e.total_sleep_minutes, e.awake_minutes, e.sleep_latency_minutes))
        .sum();
    total / entries.len() as f64
}

fn calculate_average_quality(entries: &[Answer]) -> f64 {
    if entries.is_empty() { return 0.0; }
    let sum: i32 = entries.iter().map(|e| e.sleep_quality_score).sum();
    sum as f64 / entries.len() as f64
}

fn calculate_average_sleep_hours(entries: &[Answer]) -> f64 {
    if entries.is_empty() { return 0.0; }
    let total: i32 = entries.iter().map(|e| e.total_sleep_minutes).sum();
    (total as f64 / entries.len() as f64) / 60.0
}

/// Calculates the average total sleep (night + naps) in hours
fn calculate_average_total_sleep_with_nap(entries: &[Answer]) -> f64 {
    if entries.is_empty() {
        return 0.0;
    }
    let total_with_naps: i32 = entries
        .iter()
        .map(|e| e.total_sleep_minutes + e.nap_minutes)
        .sum();
    (total_with_naps as f64 / entries.len() as f64) / 60.0
}
