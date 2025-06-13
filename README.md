# Sleep Tracker

A simple command-line sleep tracking application built in Rust that helps you monitor your sleep patterns, quality, and efficiency over time.
It helps you track your sleep during sleep restriction therapy. 
## Features

- **Sleep Data Entry**: Record detailed sleep metrics including bedtime, wake times, sleep quality, and disturbances
- **Sleep Efficiency Calculation**: Automatically calculates sleep efficiency percentages
- **Historical Analysis**: View averages for the last 7 and 30 days
- **SQLite Database**: Persistent storage of all sleep data
- **Graceful Exit**: Type `exit`, `quit`, `q`, or `stop` at any time to safely exit

## Installation

1. Make sure you have Rust installed on your system
2. Clone or download the source code
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:
```bash
cargo run
```

### Main Menu Options

1. **Enter new sleep data** - Record your sleep metrics for the current date
2. **View sleep efficiency averages** - See your sleep statistics and recent entries

### Sleep Metrics Tracked

- **Bedtime & Wake Times**: When you went to bed and woke up
- **Sleep Quality**: Rate from 1 (very poor) to 5 (excellent)
- **Total Sleep Time**: Actual hours and minutes of sleep
- **Sleep Disruptions**: Minutes awake during the night
- **Sleep Latency**: Time taken to fall asleep initially
- **Wake Events**: Number of times you woke up
- **Naps**: Daytime nap duration
- **Notes**: Optional additional observations

### Sleep Efficiency

The app calculates sleep efficiency as:
```
Sleep Efficiency = (Total Sleep Time / Time in Bed) × 100%
```

Where Time in Bed = Total Sleep + Time Awake + Sleep Latency

## Data Storage

All data is stored in a local SQLite database file (`tracker.sqlite`) in the same directory as the executable.

## Exiting the Program

You can exit the program at any time by typing:
- `exit`
- `quit`
- `q`
- `stop`


## License

This project is open source and available under the MIT License.
