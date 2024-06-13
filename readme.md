# Uptime Monitor

Uptime Monitor is a Rust-based application that periodically checks the status of a list of websites and logs the status to a SQLite database. It helps in monitoring the uptime of specified websites and records their status codes.

## Features

- Periodically checks the status of specified websites.
- Logs the status (up or down) and status code to a SQLite database.
- Handles errors and logs DNS errors specifically.
- Initializes the SQLite database with the necessary schema.

## Prerequisites

- Rust (latest stable version recommended)
- SQLite

## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/your-username/uptime_monitor.git
cd uptime_monitor
```

### 2. Install Dependencies

Ensure you have Rust and Cargo installed. If not, you can install them from [here](https://www.rust-lang.org/tools/install).

### 3. Initialize the Database

Run the `init_db` script to set up the SQLite database.

```sh
cargo run --bin init_db
```

This will create a `uptime_monitor.db` file with the necessary schema.

### 4. Run the Uptime Monitor

Execute the `uptime_monitor` script to start monitoring the websites.

```sh
cargo run --bin uptime_monitor
```

## Configuration

### Adding URLs to Monitor

Modify the `urls` vector in `src/main.rs` to include the websites you want to monitor.

```rust
let urls: Vec<&str> = vec![
    "https://google.com",
    "https://www.subinstha.com.np/",
    "https://www.outcodesoftware.com/",
    "https://asdasdasdasdasdasdasw2123.com",
];
```

### Adjusting the Check Interval

The interval for checking the website status is set to 60 seconds by default. You can change this by modifying the `Duration::from_secs(60)` value in `src/main.rs`.

```rust
time::sleep(Duration::from_secs(60)).await;
```

## Project Structure

- `src/main.rs`: The main script that performs the uptime monitoring and logging.
- `src/init_db.rs`: The script to initialize the SQLite database.
- `Cargo.toml`: The Cargo configuration file specifying dependencies and binary targets.

## Dependencies

- `reqwest`: For making HTTP requests.
- `tokio`: For asynchronous runtime.
- `rusqlite`: For interacting with the SQLite database.

---

Happy Monitoring!
