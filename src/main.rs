use env_logger;
use log::{error, info};
use rand::Rng;
use rocksdb::{Options, DB};
use std::time::{Duration, Instant};

fn main() {
    // Initialize the logger with timestamps
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .init();

    // Define the path where RocksDB will store its data
    let db_path = "my_rocksdb";

    // Set up RocksDB options
    let mut opts = Options::default();
    opts.create_if_missing(true); // Create the DB if it doesn't exist

    // Open the RocksDB database
    let db = match DB::open(&opts, db_path) {
        Ok(database) => {
            info!("Opened RocksDB at '{}'", db_path);
            database
        }
        Err(e) => {
            error!("Failed to open RocksDB: {}", e);
            return;
        }
    };

    // Initialize the random number generator
    let mut rng = rand::thread_rng();

    // Define how many key-value pairs you want to insert
    let num_entries = 100_000_000;
    let log_interval = 100_000;

    // Start the overall timer
    let overall_start = Instant::now();

    // Initialize a timer for the current batch
    let mut batch_start = Instant::now();

    for i in 0..num_entries {
        // Generate a random [u8; 32] key
        let key: [u8; 32] = rng.gen();

        // Convert the key to a Vec<u8> (RocksDB expects byte slices)
        let key_vec = key.to_vec();

        // Generate a random u64 value
        let value: u64 = rng.gen();

        // Convert the u64 value to bytes (big-endian)
        let value_bytes = value.to_be_bytes();

        // Insert the key-value pair into RocksDB
        if let Err(e) = db.put(&key_vec, &value_bytes) {
            error!("Error inserting key {}: {}", i, e);
        }

        // Check if we've reached the log interval
        if (i + 1) % log_interval == 0 {
            let batch_duration = batch_start.elapsed();
            info!(
                "Inserted {} records in {:?} seconds.",
                i + 1,
                batch_duration.as_secs_f64()
            );
            // Reset the batch timer
            batch_start = Instant::now();
        }
    }

    // Ensure all writes are flushed to disk
    if let Err(e) = db.flush() {
        error!("Failed to flush RocksDB: {}", e);
    }

    let overall_duration = overall_start.elapsed();
    info!(
        "Finished inserting {} entries into RocksDB in {:?} seconds.",
        num_entries,
        overall_duration.as_secs_f64()
    );
}
