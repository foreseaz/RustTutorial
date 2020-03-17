use blake2::{Blake2b, Blake2s, Digest};
use rocksdb::{Options, DB};
use std::time::{Duration, Instant};
fn test_sled(count: u64) -> Result<(), String> {
    let tree = sled::open("./sled_storage").map_err(|e| e.to_string())?;
    let mut i: u64 = 0;
    loop {
        let mut hasher = Blake2s::new();
        let key = format!("{}", i);
        hasher.input(key.as_bytes());

        let mut hasher_data = Blake2b::new();
        let data = format!("data {}", i * 100);
        hasher_data.input(data.as_bytes());

        let key_res = hasher.result();
        let data_res = hasher_data.result();
        println!(
            "{} {} {}",
            i,
            hex::encode(&key_res[..]),
            hex::encode(&data_res[..])
        );
        tree.insert(&key_res[..], &data_res[..]);
        i = i + 1;
        println!("i={}", i);
        if i > count {
            break;
        }
    }

    Ok(())
}

fn test_rocksdb(count: u64) -> Result<(), String> {
    let db = DB::open_default("./rocksdb_storage")?;
    let mut i: u64 = 0;
    loop {
        let mut hasher = Blake2s::new();
        let key = format!("{}", i);
        hasher.input(key.as_bytes());

        let mut hasher_data = Blake2b::new();
        let data = format!("data {}", i * 100);
        hasher_data.input(data.as_bytes());

        let key_res = hasher.result();
        let data_res = hasher_data.result();
        println!(
            "{} {} {}",
            i,
            hex::encode(&key_res[..]),
            hex::encode(&data_res[..])
        );
        db.put(&key_res[..], &data_res[..]);
        i = i + 1;
        println!("i={}", i);
        if i > count {
            break;
        }
    }

    Ok(())
}

fn test_sledtree(count: u64) -> Result<(), String> {
    let db = sled::open("./sledtree_storage").map_err(|e| e.to_string())?;
    let tree = db.open_tree("core").map_err(|e| e.to_string())?;
    let mut i: u64 = 0;
    loop {
        let mut hasher = Blake2s::new();
        let key = format!("{}", i);
        hasher.input(key.as_bytes());

        let mut hasher_data = Blake2b::new();
        let data = format!("data {}", i * 100);
        hasher_data.input(data.as_bytes());

        let key_res = hasher.result();
        let data_res = hasher_data.result();
        println!(
            "{} {} {}",
            i,
            hex::encode(&key_res[..]),
            hex::encode(&data_res[..])
        );
        tree.insert(&key_res[..], &data_res[..]);
        i = i + 1;
        println!("i={}", i);
        if i > count {
            break;
        }
    }

    Ok(())
}

fn test_sledtree_batch(count: u64) -> Result<(), String> {
    let db = sled::open("./sledtreebatch_storage").map_err(|e| e.to_string())?;
    let tree = db.open_tree("core").map_err(|e| e.to_string())?;
    let mut i: u64 = 0;
    loop {
        let mut j: u64 = 0;

        let mut batch = sled::Batch::default();
        for j in 0..100 {
            let mut hasher = Blake2s::new();
            let key = format!("{}", i);
            hasher.input(key.as_bytes());

            let mut hasher_data = Blake2b::new();
            let data = format!("data {}", i * 100);
            hasher_data.input(data.as_bytes());

            let key_res = hasher.result();
            let data_res = hasher_data.result();
            println!(
                "{} {} {}",
                i,
                hex::encode(&key_res[..]),
                hex::encode(&data_res[..])
            );
            batch.insert(&key_res[..], &data_res[..]);

            i = i + 1;
            println!("i={}", i);
            if i > count {
                break;
            }
        }
        println!("applying");
        tree.apply_batch(batch).unwrap();

        if i > count {
            break;
        }
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
    //    let count=1000000;
    let count = 200;
    let mut start = Instant::now();

    start = Instant::now();
    test_rocksdb(count);
    let e1 = start.elapsed();

    start = Instant::now();
    test_sled(count);
    let e2 = start.elapsed();

    start = Instant::now();
    test_sledtree(count);
    let e3 = start.elapsed();

    start = Instant::now();
    test_sledtree_batch(count);
    let e4 = start.elapsed();
    println!("{:?} {:?} {:?}", e1, e2, e3);
    println!("{:?}", e4);
}
