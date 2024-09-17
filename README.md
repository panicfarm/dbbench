## key-value store benches

For write-heavy loads, rocksdb rocks, redb sucks

```
$ RUST_LOG=info cargo run --release -- redb
   Compiling rocksdb_example v0.1.0 (/home/alecm/rocksdb_example)
    Finished release [optimized] target(s) in 4.90s
     Running `target/release/rocksdb_example redb`
[2024-09-17T19:00:42Z INFO  rocksdb_example] Inserted 100000 records in 0.43 seconds.
[2024-09-17T19:00:43Z INFO  rocksdb_example] Inserted 200000 records in 0.80 seconds.
[2024-09-17T19:00:44Z INFO  rocksdb_example] Inserted 300000 records in 0.94 seconds.
[2024-09-17T19:00:46Z INFO  rocksdb_example] Inserted 400000 records in 1.93 seconds.
[2024-09-17T19:00:48Z INFO  rocksdb_example] Inserted 500000 records in 2.17 seconds.
[2024-09-17T19:00:50Z INFO  rocksdb_example] Inserted 600000 records in 2.39 seconds.
[2024-09-17T19:00:53Z INFO  rocksdb_example] Inserted 700000 records in 3.02 seconds.
[2024-09-17T19:00:56Z INFO  rocksdb_example] Inserted 800000 records in 2.58 seconds.
[2024-09-17T19:00:59Z INFO  rocksdb_example] Inserted 900000 records in 2.74 seconds.
[2024-09-17T19:01:01Z INFO  rocksdb_example] Inserted 1000000 records in 2.36 seconds.
[2024-09-17T19:01:04Z INFO  rocksdb_example] Inserted 1100000 records in 2.54 seconds.
^C
$ RUST_LOG=info cargo run --release -- rocksdb
    Finished release [optimized] target(s) in 0.14s
     Running `target/release/rocksdb_example rocksdb`
[2024-09-17T18:42:29Z INFO  rocksdb_example] Opened RocksDB at 'my_rocksdb'
[2024-09-17T18:42:29Z INFO  rocksdb_example] Inserted 100000 records in 0.23 seconds.
[2024-09-17T18:42:30Z INFO  rocksdb_example] Inserted 200000 records in 0.25 seconds.
[2024-09-17T18:42:30Z INFO  rocksdb_example] Inserted 300000 records in 0.24 seconds.
[2024-09-17T18:42:30Z INFO  rocksdb_example] Inserted 400000 records in 0.26 seconds.
[2024-09-17T18:42:31Z INFO  rocksdb_example] Inserted 500000 records in 0.29 seconds.
[2024-09-17T18:42:31Z INFO  rocksdb_example] Inserted 600000 records in 0.31 seconds.
```
