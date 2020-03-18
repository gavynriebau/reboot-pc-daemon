use system_shutdown::force_reboot;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader};

const REBOOT_CODE: &str = "<SECRET>";

fn reboot() {
    match force_reboot() {
        Ok(_) => println!("Rebooting..."),
        Err(_) => println!("Failed to reboot")
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    loop {
        let mut text = String::new();
        match reader.read_line(&mut text) {
            Ok(count) => {
                if count == 0 {
                    break;
                }

                if text == REBOOT_CODE {
                    reboot();
                }
            },
            Err(_) => break
        }
    }
}

fn main() {
    println!("Starting reboot server listener");

    let listener = TcpListener::bind("0.0.0.0:64321").expect("Failed to bind to TCP port");

    for stream in listener.incoming() {
        std::thread::spawn(move || {
            handle_client(stream.expect("Failed to get stream"));
        });
    }
}
