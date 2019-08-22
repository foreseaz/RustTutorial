extern crate futures;
extern crate tokio;
extern crate websocket;

use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::sync::mpsc::Sender;
//use jsonrpc_ws_client::connect;
use jsonrpc_client_transports::transports::ws::connect;
use jsonrpc_client_transports::{RawClient, TypedClient};
use jsonrpc_client_transports::{RpcChannel, RpcError};
use std::io::stdin;
use std::thread;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};
const CONNECTION: &'static str = "ws://localhost:26657/websocket";
#[derive(Clone)]
struct TestClient(TypedClient);

impl From<RpcChannel> for TestClient {
    fn from(channel: RpcChannel) -> Self {
        TestClient(channel.into())
    }
}

impl TestClient {
    fn hello(&self, msg: &'static str) -> impl Future<Item = String, Error = RpcError> {
        self.0.call_method("hello", "String", (msg,))
    }
    fn fail(&self) -> impl Future<Item = (), Error = RpcError> {
        self.0.call_method("fail", "()", ())
    }
}
fn main2() {
    let a = connect::<TestClient>(CONNECTION);

    let mut runtime = tokio::runtime::current_thread::Builder::new()
        .build()
        .unwrap();
    runtime.block_on(a.unwrap());

    // rt::run(run);

    // then
    // let result = rx.recv_timeout(Duration::from_secs(3)).unwrap();

    println!("OK");
}

fn main() {
    println!("Connecting to {}", CONNECTION);
    let mut runtime = tokio::runtime::current_thread::Builder::new()
        .build()
        .unwrap();

    // standard in isn't supported in mio yet, so we use a thread
    // see https://github.com/carllerche/mio/issues/321
    let (usr_msg, stdin_ch) = mpsc::channel(0);
    thread::spawn(|| {
        let mut input = String::new();
        let mut stdin_sink = usr_msg.wait();
        loop {
            input.clear();
            stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();

            let (close, msg) = match trimmed {
                "/close" => (true, OwnedMessage::Close(None)),
                "/ping" => (false, OwnedMessage::Ping(b"PING".to_vec())),
                _ => (false, OwnedMessage::Text(trimmed.to_string())),
            };

            stdin_sink
                .send(msg)
                .expect("Sending message across stdin channel.");

            if close {
                break;
            }
        }
    });

    let runner = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .async_connect_insecure()
        .and_then(|(duplex, _)| {
            let (sink, stream) = duplex.split();
            stream
                .filter_map(|message| {
                    println!("Received Message: {:?}", message);
                    match message {
                        OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
                        OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                        _ => None,
                    }
                })
                .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
                .forward(sink)
        });
    runtime.block_on(runner).unwrap();
}
