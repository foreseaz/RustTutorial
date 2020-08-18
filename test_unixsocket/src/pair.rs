use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream,UnixListener};
use std::thread;
use std::io::Write;
use std::io::Read;


pub fn main()
{
    let (mut s1, mut s2) = UnixStream::pair().unwrap();

    thread::spawn(move || {
            loop {
                let mut buf = [0; 1024];
                let count = s1.read(&mut buf).unwrap();
                let response = String::from_utf8(buf[..count].to_vec()).unwrap();
                let reply= format!("received hello world {}",chrono::Local::now());
                //println!("received {}", response);
                s1.write_all(reply.as_bytes()).unwrap();
            }
    });

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let m= format!("hello world {}",chrono::Local::now());
            s2.write_all(m.as_bytes()).unwrap();
            let mut buf = [0; 1024];
            let count = s2.read(&mut buf).unwrap();
            let response = String::from_utf8(buf[..count].to_vec()).unwrap();
            println!("received {}  {}", count, response);        }

}