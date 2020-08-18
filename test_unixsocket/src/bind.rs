use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream,UnixListener};
use std::thread;

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap());
    }
}

fn test_connect()
{
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
       // println!("client..");
    }
}
pub fn main() {
    let listener = UnixListener::bind("/tmp/my_uds.sock").unwrap();

    thread::spawn(|| {
        test_connect();

    });
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}