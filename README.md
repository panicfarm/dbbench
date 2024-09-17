## key-value store benches

For write-heavy loads, rocksdb rocks, redb sucks

```
$ RUST_LOG=info cargo run --release -- redb
   Compiling rocksdb_example v0.1.0 (/home/alecm/rocksdb_example)
    Finished release [optimized] target(s) in 4.90s
     Running `target/release/rocksdb_example redb`
[2024-09-17T18:41:55Z INFO  rocksdb_example] Opened REDB at 'my_redb'
[2024-09-17T18:41:55Z INFO  rocksdb_example] Inserted 100000 records in 0.58 seconds.
[2024-09-17T18:41:58Z INFO  rocksdb_example] Inserted 200000 records in 2.95 seconds.
[2024-09-17T18:42:01Z INFO  rocksdb_example] Inserted 300000 records in 6.00 seconds.
[2024-09-17T18:42:04Z INFO  rocksdb_example] Inserted 400000 records in 8.80 seconds.
[2024-09-17T18:42:07Z INFO  rocksdb_example] Inserted 500000 records in 11.92 seconds.
[2024-09-17T18:42:10Z INFO  rocksdb_example] Inserted 600000 records in 15.09 seconds.
[2024-09-17T18:42:13Z INFO  rocksdb_example] Inserted 700000 records in 18.03 seconds.
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
