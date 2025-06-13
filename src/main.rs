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
    /// Creates a new AnswerDao instance and opens database connection
    /// 
    /// # Arguments
    /// * `db_path` - Path to the SQLite database file
    /// 
    /// # Returns
    /// * `Result<Self>` - New AnswerDao instance or error
    fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(AnswerDao { conn })
    }

    /// Creates the answer table if it doesn't exist
    /// 
    /// # Returns
    /// * `Result<()>` - Success or database error
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

    /// Inserts a new sleep tracking entry into the database
    /// 
    /// # Arguments
    /// * `entry_date` - Date of the sleep entry (YYYY-MM-DD format)
    /// * `bedtime` - Time when user went to bed (HH:MM format)
    /// * `wake_target` - Intended wake up time (HH:MM format)
    /// * `wake_actual` - Actual wake up time (HH:MM format)
    /// * `nap` - Total nap time in minutes
    /// * `quality` - Sleep quality score (1-5)
    /// * `total` - Total sleep time in minutes
    /// * `awake` - Time spent awake during the night in minutes
    /// * `latency` - Time to fall asleep in minutes
    /// * `count` - Number of times woken up during the night
    /// * `notes` - Additional notes about the sleep
    /// 
    /// # Returns
    /// * `Result<i64>` - ID of the inserted record or database error
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

    /// Retrieves all sleep entries from the database, ordered by most recent first
    /// 
    /// # Returns
    /// * `Result<Vec<Answer>>` - Vector of all Answer entries or database error
    fn list_all(&self) -> Result<Vec<Answer>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, entry_date, bedtime, wake_time_target, wake_time_actual,
                    nap_minutes, sleep_quality_score, total_sleep_minutes,
                    awake_minutes, sleep_latency_minutes, wake_count, notes
             FROM answer ORDER BY id DESC"
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

    /// Retrieves sleep entries from the last N days
    /// 
    /// # Arguments
    /// * `days` - Number of days to look back
    /// 
    /// # Returns
    /// * `Result<Vec<Answer>>` - Vector of Answer entries within the date range or database error
    fn get_recent_entries(&self, days: i32) -> Result<Vec<Answer>> {
        let cutoff_date = Local::now()
            .checked_sub_days(chrono::Days::new(days as u64))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();

        let mut stmt = self.conn.prepare(
            "SELECT id, entry_date, bedtime, wake_time_target, wake_time_actual,
                    nap_minutes, sleep_quality_score, total_sleep_minutes,
                    awake_minutes, sleep_latency_minutes, wake_count, notes
             FROM answer 
             WHERE entry_date >= ?1 
             ORDER BY entry_date DESC"
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

/// Main entry point for the sleep tracker application
fn main() {
    match run_app() {
        Ok(()) => println!("Program completed successfully."),
        Err(AppError::UserExit) => println!("\nGoodbye! 👋"),
        Err(AppError::Database(e)) => eprintln!("Database error: {}", e),
        Err(AppError::Io(e)) => eprintln!("IO error: {}", e),
    }
}

/// Main application logic with proper error handling
fn run_app() -> std::result::Result<(), AppError> {
    let dao = AnswerDao::new("tracker.sqlite")?;
    dao.create_table()?;

    println!("--- Sleep Tracker ---");
    println!("💡 Tip: Type 'exit', 'quit', or 'q' at any time to stop the program");
    println!();
    println!("1. Enter new sleep data");
    println!("2. View sleep efficiency averages");
    print!("Choose option (1 or 2): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    
    // Check for exit command
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

/// Checks if the input is an exit command
/// 
/// # Arguments
/// * `input` - The user input to check
/// 
/// # Returns
/// * `bool` - True if the input is an exit command
fn is_exit_command(input: &str) -> bool {
    let input_lower = input.to_lowercase();
    matches!(input_lower.as_str(), "exit" | "quit" | "q" | "stop")
}

/// Prompts user to enter sleep data and saves it to the database
/// 
/// # Arguments
/// * `dao` - Reference to the AnswerDao for database operations
/// 
/// # Returns
/// * `Result<()>` - Success or error from database operations
fn enter_sleep_data(dao: &AnswerDao) -> std::result::Result<(), AppError> {
    let entry_date = Local::now().format("%Y-%m-%d").to_string();
    
    println!("\n--- Enter Sleep Data for {} ---", entry_date);
    println!("💡 Reminder: Type 'exit', 'quit', or 'q' at any prompt to stop");

    // Get sleep timing information
    let bedtime = get_input("What time did you go to bed last night? (HH:MM format, e.g., 22:30): ")?;
    let wake_target = get_input("What time did you plan to wake up? (HH:MM format, e.g., 07:00): ")?;
    let wake_actual = get_input("What time did you actually wake up this morning? (HH:MM format, e.g., 07:15): ")?;
    
    // Get sleep quality metrics
    let nap = get_number_input("How many minutes did you nap yesterday? (enter 0 if no naps): ")?;
    let quality = get_number_input("Rate your sleep quality (1=very poor, 2=poor, 3=fair, 4=good, 5=excellent): ")?;
    
    // Get sleep duration and disruptions
    let total_sleep_str = get_input("How much total sleep did you get? (HH:MM format, e.g., 07:30): ")?;
    let awake = get_number_input("How many minutes were you awake during the night (not counting time to fall asleep)? ")?;
    let latency = get_number_input("How many minutes did it take you to fall asleep initially? ")?;
    let wake_count = get_number_input("How many times did you wake up during the night? ")?;
    
    // Get optional notes
    let notes = get_input("Any additional notes about your sleep (optional, press Enter to skip): ")?;

    // Convert total sleep time to minutes and calculate efficiency
    let total_min = to_minutes(&total_sleep_str);
    let efficiency = calc_efficiency(total_min, awake, latency);

    // Save to database
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

    println!(
        "\n✓ Sleep data saved successfully!"
    );
    println!("Entry ID: {}", id);
    println!("Sleep Efficiency: {:.1}%", efficiency);
    println!("Total Sleep: {:.1} hours", total_min as f64 / 60.0);

    Ok(())
}

/// Displays sleep efficiency averages and recent entries summary
/// 
/// # Arguments
/// * `dao` - Reference to the AnswerDao for database operations
/// 
/// # Returns
/// * `Result<()>` - Success or error from database operations
fn show_efficiency_averages(dao: &AnswerDao) -> std::result::Result<(), AppError> {
    println!("\n--- Sleep Efficiency Averages ---");
    
    // Last 7 days statistics
    let entries_7_days = dao.get_recent_entries(7)?;
    if !entries_7_days.is_empty() {
        let avg_efficiency_7 = calculate_average_efficiency(&entries_7_days);
        let avg_quality_7 = calculate_average_quality(&entries_7_days);
        let avg_sleep_7 = calculate_average_sleep_hours(&entries_7_days);
        println!("Last 7 days ({} entries):", entries_7_days.len());
        println!("  Average Sleep Efficiency: {:.1}%", avg_efficiency_7);
        println!("  Average Sleep Quality: {:.1}/5", avg_quality_7);
        println!("  Average Sleep Duration: {:.1} hours", avg_sleep_7);
    } else {
        println!("Last 7 days: No data available");
    }

    // Last 30 days statistics
    let entries_30_days = dao.get_recent_entries(30)?;
    if !entries_30_days.is_empty() {
        let avg_efficiency_30 = calculate_average_efficiency(&entries_30_days);
        let avg_quality_30 = calculate_average_quality(&entries_30_days);
        let avg_sleep_30 = calculate_average_sleep_hours(&entries_30_days);
        println!("\nLast 30 days ({} entries):", entries_30_days.len());
        println!("  Average Sleep Efficiency: {:.1}%", avg_efficiency_30);
        println!("  Average Sleep Quality: {:.1}/5", avg_quality_30);
        println!("  Average Sleep Duration: {:.1} hours", avg_sleep_30);
    } else {
        println!("Last 30 days: No data available");
    }

    // Show recent entries summary
    if !entries_7_days.is_empty() {
        println!("\n--- Recent Entries Summary ---");
        for (i, entry) in entries_7_days.iter().take(5).enumerate() {
            let efficiency = calc_efficiency(
                entry.total_sleep_minutes,
                entry.awake_minutes,
                entry.sleep_latency_minutes,
            );
            println!(
                "{}. {} - {:.1} hrs sleep, {:.1}% efficiency, quality {}/5",
                i + 1,
                entry.entry_date,
                entry.total_sleep_minutes as f64 / 60.0,
                efficiency,
                entry.sleep_quality_score
            );
        }
    }

    // Ask if user wants to continue viewing data
    println!("\nPress Enter to continue or type 'exit'/'quit'/'q' to stop...");
    let input = get_input("")?;
    // If user enters exit command, it will be handled by get_input

    Ok(())
}

/// Prompts user for text input with the given prompt and handles exit commands
/// 
/// # Arguments
/// * `prompt` - The prompt message to display to the user
/// 
/// # Returns
/// * `Result<String, AppError>` - The user's input as a trimmed string or UserExit error
fn get_input(prompt: &str) -> std::result::Result<String, AppError> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let trimmed_input = input.trim().to_string();
    
    // Check if user wants to exit
    if is_exit_command(&trimmed_input) {
        return Err(AppError::UserExit);
    }
    
    Ok(trimmed_input)
}

/// Prompts user for numeric input with validation, retry logic, and exit handling
/// 
/// # Arguments
/// * `prompt` - The prompt message to display to the user
/// 
/// # Returns
/// * `Result<i32, AppError>` - The parsed integer value or UserExit error
fn get_number_input(prompt: &str) -> std::result::Result<i32, AppError> {
    loop {
        let input = get_input(prompt)?;
        
        // Allow empty input for optional fields or default to 0
        if input.is_empty() {
            return Ok(0);
        }
        
        match input.parse::<i32>() {
            Ok(num) => return Ok(num),
            Err(_) => println!("Please enter a valid number (or 'exit' to quit)."),
        }
    }
}

/// Converts time in HH:MM format to total minutes
/// 
/// # Arguments
/// * `hhmm` - Time string in HH:MM format
/// 
/// # Returns
/// * `i32` - Total minutes, or 0 if parsing fails
/// 
/// # Examples
/// ```
/// assert_eq!(to_minutes("07:30"), 450);
/// assert_eq!(to_minutes("12:00"), 720);
/// ```
fn to_minutes(hhmm: &str) -> i32 {
    match hhmm.split(':').collect::<Vec<&str>>().as_slice() {
        [hours_str, minutes_str] => {
            let hours = hours_str.parse::<i32>().unwrap_or(0);
            let minutes = minutes_str.parse::<i32>().unwrap_or(0);
            hours * 60 + minutes
        }
        _ => 0,
    }
}

/// Calculates the time window between bedtime and target wake time
/// Handles cases where wake time is the next day
/// 
/// # Arguments
/// * `bedtime` - Bedtime in HH:MM format
/// * `wake_target` - Target wake time in HH:MM format
/// 
/// # Returns
/// * `i32` - Time window in minutes
fn calc_window(bedtime: &str, wake_target: &str) -> i32 {
    let bedtime_minutes = to_minutes(bedtime);
    let mut wake_time_minutes = to_minutes(wake_target);

    // If wake time is earlier in the day than bedtime, assume next day
    if wake_time_minutes <= bedtime_minutes {
        wake_time_minutes += 24 * 60; // Add 24 hours
    }

    wake_time_minutes - bedtime_minutes
}

/// Rounds a floating point value to 2 significant figures
/// 
/// # Arguments
/// * `value` - The value to round
/// 
/// # Returns
/// * `f64` - The rounded value
fn round_to_2_sig_figs(value: f64) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    let scale = value.abs().log10().floor() as i32;
    let factor = 10.0f64.powi(1 - scale);
    (value * factor).round() / factor
}

/// Calculates sleep efficiency as a percentage
/// Sleep efficiency = (time asleep / time in bed) * 100
/// 
/// # Arguments
/// * `sleep` - Total sleep time in minutes
/// * `awake` - Time awake during the night in minutes
/// * `latency` - Time to fall asleep in minutes
/// 
/// # Returns
/// * `f64` - Sleep efficiency percentage (0-100)
fn calc_efficiency(sleep: i32, awake: i32, latency: i32) -> f64 {
    let time_in_bed = sleep + awake + latency;
    if time_in_bed == 0 {
        return 0.0;
    }
    round_to_2_sig_figs(sleep as f64 / time_in_bed as f64 * 100.0)
}

/// Calculates the average sleep efficiency for a collection of entries
/// 
/// # Arguments
/// * `entries` - Slice of Answer entries
/// 
/// # Returns
/// * `f64` - Average sleep efficiency percentage
fn calculate_average_efficiency(entries: &[Answer]) -> f64 {
    if entries.is_empty() {
        return 0.0;
    }
    
    let total_efficiency: f64 = entries
        .iter()
        .map(|entry| {
            calc_efficiency(
                entry.total_sleep_minutes,
                entry.awake_minutes,
                entry.sleep_latency_minutes,
            )
        })
        .sum();
    
    total_efficiency / entries.len() as f64
}

/// Calculates the average sleep quality score for a collection of entries
/// 
/// # Arguments
/// * `entries` - Slice of Answer entries
/// 
/// # Returns
/// * `f64` - Average sleep quality score (1-5 scale)
fn calculate_average_quality(entries: &[Answer]) -> f64 {
    if entries.is_empty() {
        return 0.0;
    }
    
    let total_quality: i32 = entries.iter().map(|entry| entry.sleep_quality_score).sum();
    total_quality as f64 / entries.len() as f64
}

/// Calculates the average sleep duration in hours for a collection of entries
/// 
/// # Arguments
/// * `entries` - Slice of Answer entries
/// 
/// # Returns
/// * `f64` - Average sleep duration in hours
fn calculate_average_sleep_hours(entries: &[Answer]) -> f64 {
    if entries.is_empty() {
        return 0.0;
    }
    
    let total_minutes: i32 = entries.iter().map(|entry| entry.total_sleep_minutes).sum();
    (total_minutes as f64 / entries.len() as f64) / 60.0
}



