use env_logger;
use log::{error, info};
use rand::Rng;
use redb::{Database, TableDefinition};
use rocksdb::{
    DBCompactionStyle, DBCompressionType, Options, WriteBatch, WriteOptions as RocksWriteOptions,
    DB,
};
use std::env;
use std::time::Instant;

fn main() {
    // Initialize the logger with timestamps
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .init();

    // Benchmark RocksDB
    bench_rocksdb();

    // Benchmark REDB
    bench_redb();
}

fn bench_rocksdb() {
    // Define the path where RocksDB will store its data
    let db_path = "my_rocksdb";

    // Set up RocksDB options
    let mut opts = Options::default();
    opts.create_if_missing(true); // Create the DB if it doesn't exist

    // ===========================
    // Threshold Trigger Settings
    // ===========================

    // Set the size of the write buffer (memtable) in bytes (e.g., 512MB)
    opts.set_write_buffer_size(512 * 1024 * 1024);

    // Set the maximum number of write buffers that are built up in memory
    opts.set_max_write_buffer_number(3);

    // Set the maximum number of background jobs (compactions and flushes)
    opts.set_max_background_jobs(4);

    // Set the base size for level 1 (in level-based compaction)
    opts.set_max_bytes_for_level_base(1024 * 1024 * 1024); // 1GB

    // Set target file size for level compaction
    opts.set_target_file_size_base(128 * 1024 * 1024); // 128MB

    // ===========================
    // Compaction Settings
    // ===========================

    // Set compaction style (Level, Universal, FIFO)
    opts.set_compaction_style(DBCompactionStyle::Level);

    // Set the number of files to trigger a level 0 compaction
    opts.set_level_zero_file_num_compaction_trigger(4);

    // Set the maximum number of level 0 files before slowing down writes
    opts.set_level_zero_slowdown_writes_trigger(20);

    // Set the maximum number of level 0 files before stopping writes
    opts.set_level_zero_stop_writes_trigger(24);

    // ===========================
    // Additional Optimizations (Optional)
    // ===========================

    // Optimize RocksDB for faster writes (tweak based on your workload)
    opts.optimize_for_point_lookup(10);

    // Set the compression algorithm (Snappy is default)
    opts.set_compression_type(DBCompressionType::Snappy);

    // ===========================

    // Open the RocksDB database with the configured options
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
    let num_entries = 1_000_000;
    let log_interval = 10_000;
    let batch_size = 10_000; // Number of records per batch

    // Create WriteOptions with disable_wal set to true
    let mut write_opts = RocksWriteOptions::default();
    write_opts.disable_wal(true);

    // Start the overall timer
    let overall_start = Instant::now();

    // Initialize a timer for the current batch
    let mut batch_start = Instant::now();

    // Initialize a WriteBatch
    let mut batch = WriteBatch::default();

    for i in 0..num_entries {
        // Generate a random [u8; 32] key
        let key: [u8; 32] = rng.gen();

        // Convert the key to a Vec<u8> (RocksDB expects byte slices)
        let key_vec = key.to_vec();

        // Generate a random u64 value
        let value: u64 = rng.gen();

        // Convert the u64 value to bytes (big-endian)
        let value_bytes = value.to_be_bytes();

        // Add the key-value pair to the batch
        batch.put(&key_vec, &value_bytes);

        // If the batch is full, write it to RocksDB using write_opt
        if (i + 1) % batch_size == 0 {
            if let Err(e) = db.write_opt(batch, &write_opts) {
                error!("Error writing batch at record {}: {}", i + 1, e);
            }
            // Clear the batch for the next set of records
            batch = WriteBatch::default();
        }

        // Check if we've reached the log interval
        if (i + 1) % log_interval == 0 {
            let batch_duration = batch_start.elapsed();
            info!(
                "Inserted {} records in {:.2} seconds.",
                i + 1,
                batch_duration.as_secs_f64()
            );
            // Reset the batch timer
            batch_start = Instant::now();
        }
    }

    // Write any remaining records in the batch
    if !batch.is_empty() {
        if let Err(e) = db.write_opt(batch, &write_opts) {
            error!("Error writing final batch: {}", e);
        }
    }

    // Ensure all writes are flushed to disk
    if let Err(e) = db.flush() {
        error!("Failed to flush RocksDB: {}", e);
    }

    let overall_duration = overall_start.elapsed();
    info!(
        "Finished inserting {} entries into RocksDB in {:.2} seconds.",
        num_entries,
        overall_duration.as_secs_f64()
    );
}

fn bench_redb() {
    // Define the path where REDB will store its data
    let db_path = "my_redb";

    // Define a table for key-value pairs
    // Use a key type that implements `Key`, e.g., [u8;32]
    const TABLE_DEF: TableDefinition<[u8; 32], u64> = TableDefinition::new("kv");

    // Open or create the REDB database
    let db = match Database::create(db_path) {
        Ok(db) => {
            info!("Opened REDB at '{}'", db_path);
            db
        }
        Err(e) => {
            error!("Failed to open REDB: {}", e);
            return;
        }
    };

    // Initialize the random number generator
    let mut rng = rand::thread_rng();

    // Define how many key-value pairs you want to insert
    let num_entries = 1_000_000;
    let log_interval = 10_000;
    let batch_size = 10_000; // Number of records per transaction

    // Start the overall timer
    let overall_start = Instant::now();

    // Calculate the number of full batches
    let num_full_batches = num_entries / batch_size;
    let remaining_entries = num_entries % batch_size;

    for batch_idx in 0..num_full_batches {
        // Begin a write transaction
        let transaction = match db.begin_write() {
            Ok(tx) => tx,
            Err(e) => {
                error!(
                    "Failed to begin transaction for batch {}: {}",
                    batch_idx + 1,
                    e
                );
                continue;
            }
        };

        // Open or create the table within the transaction
        {
            let mut table = match transaction.open_table(TABLE_DEF) {
                Ok(table) => table,
                Err(e) => {
                    error!("Failed to open table in batch {}: {}", batch_idx + 1, e);
                    continue;
                }
            };

            // Insert BATCH_SIZE number of key-value pairs
            for i in 0..batch_size {
                let global_idx = batch_idx * batch_size + i;

                // Generate a random [u8; 32] key
                let key: [u8; 32] = rng.gen();

                // Generate a random u64 value
                let value: u64 = rng.gen();

                // Insert the key-value pair into the table
                if let Err(e) = table.insert(&key, &value) {
                    error!("Error inserting key {}: {}", global_idx + 1, e);
                }

                // Log progress at specified intervals
                if (global_idx + 1) % log_interval == 0 {
                    let elapsed = overall_start.elapsed();
                    info!(
                        "Inserted {} records in {:.2} seconds.",
                        global_idx + 1,
                        elapsed.as_secs_f64()
                    );
                }
            }
        } // table is dropped here

        // Commit the transaction
        if let Err(e) = transaction.commit() {
            error!(
                "Failed to commit transaction for batch {}: {}",
                batch_idx + 1,
                e
            );
        }
    }

    // Handle any remaining entries that didn't fit into a full batch
    if remaining_entries > 0 {
        // Begin a write transaction
        let transaction = match db.begin_write() {
            Ok(tx) => tx,
            Err(e) => {
                error!("Failed to begin transaction for remaining entries: {}", e);
                return;
            }
        };

        // Open or create the table within the transaction
        {
            let mut table = match transaction.open_table(TABLE_DEF) {
                Ok(table) => table,
                Err(e) => {
                    error!("Failed to open table for remaining entries: {}", e);
                    return;
                }
            };

            // Insert remaining_entries number of key-value pairs
            for i in 0..remaining_entries {
                let global_idx = num_full_batches * batch_size + i;

                // Generate a random [u8; 32] key
                let key: [u8; 32] = rng.gen();

                // Generate a random u64 value
                let value: u64 = rng.gen();

                // Insert the key-value pair into the table
                if let Err(e) = table.insert(&key, &value) {
                    error!("Error inserting key {}: {}", global_idx + 1, e);
                }

                // Log progress
                if (global_idx + 1) % log_interval == 0 {
                    let elapsed = overall_start.elapsed();
                    info!(
                        "Inserted {} records in {:.2} seconds.",
                        global_idx + 1,
                        elapsed.as_secs_f64()
                    );
                }
            }
        } // table is dropped here

        // Commit the transaction
        if let Err(e) = transaction.commit() {
            error!("Failed to commit transaction for remaining entries: {}", e);
        }
    }

    let overall_duration = overall_start.elapsed();
    info!(
        "Finished inserting {} entries into REDB in {:.2} seconds.",
        num_entries,
        overall_duration.as_secs_f64()
    );
}
