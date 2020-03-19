#![windows_subsystem = "windows"]

use system_shutdown::force_reboot;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader};

const PORT: u16 = 64321;

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    let secret = std::env::args().nth(1).expect("Failed to get secret");
    loop {
        let mut text = String::new();
        match reader.read_line(&mut text) {
            Ok(count) => {
                if count == 0 {
                    break;
                }

                if text.contains(&secret) {
                    force_reboot().expect("Failed to reboot");
                }
            },
            Err(_) => break
        }
    }
}

fn main() {
    let listener = TcpListener::bind(("0.0.0.0", PORT)).expect("Failed to bind to TCP port");

    for stream in listener.incoming() {
        std::thread::spawn(move || {
            handle_client(stream.expect("Failed to get stream"));
        });
    }
}
