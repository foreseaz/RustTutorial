pub fn simple_test2() -> Result<(), failure::Error> {
    println!("simiple test");
    let path = ".storage";
    let k = "apple".as_bytes();
    let v = "data".as_bytes();
    let db = sled::open(path)?; // as in fs::open
    if let Ok(value) = db.get(k) {
        if let Some(value2) = value {
            println!(
                "{} = {}",
                String::from_utf8(k.to_vec())?,
                String::from_utf8(value2.to_vec())?
            );
        }
    }
    db.insert(k, v)?;
    db.flush();
    println!("ok");
    loop {
        println!("waiting..");
        std::thread::sleep(std::time::Duration::from_secs(100000000));
    }
}

pub fn simple_test() -> Result<(), failure::Error> {
    println!("simiple test");
    let path = ".storage";
    let k = "apple".as_bytes();
    let v = "data".as_bytes();
    //  let db = sled::open(path)?; // as in fs::open

    let db = sled::Config::default()
        .path(&path)
        .flush_every_ms(Some(000))
        .open()?;

    if let Ok(value) = db.get(k) {
        if let Some(value2) = value {
            println!(
                "{} = {}",
                String::from_utf8(k.to_vec())?,
                String::from_utf8(value2.to_vec())?
            );
        }
    }
    db.insert(k, v)?;
    db.flush();
    println!("ok");
    loop {
        println!("waiting..");
        std::thread::sleep(std::time::Duration::from_secs(100000000));
    }
}
