use std::io::{self, Read};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::thread;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

const SOCKET_PATH: &str = "/tmp/firefox-extension.sock";

fn read_input(mut input: io::Stdin) -> io::Result<serde_json::Value> {
    let length = input.read_u32::<NativeEndian>().unwrap();
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;
    let json_val: serde_json::Value = serde_json::from_slice(&buffer).unwrap();
    Ok(json_val)
}

fn write_output(mut output: io::Stdout, value: &serde_json::Value) -> io::Result<()> {
    let msg = serde_json::to_string(value)?;
    let len = msg.len();
    output.write_u32::<NativeEndian>(len as u32)?;
    output.write_all(msg.as_bytes())?;
    output.flush()?;
    Ok(())
}

fn list() -> String {
    let _ = write_output(io::stdout(), &serde_json::json!({"command": "list"}));
    let res = &read_input(io::stdin()).unwrap();
    let res = res.get("titles").unwrap();
    let res = res.as_array().unwrap();
    let mut r = String::new();
    for item in res {
        r.push_str(item.as_str().unwrap());
        r.push_str("\n");
    }
    format!("{}\0", r)
}

fn open(url: &str) -> String {
    let _ = write_output(
        io::stdout(),
        &serde_json::json!({"command": "open", "url" : url}),
    );
    String::from("Opening URL\0")
}

fn close(index: &str) -> String {
    let _ = write_output(
        io::stdout(),
        &serde_json::json!({"command": "close", "index" : index}),
    );
    String::from("Closing tab (if it exists)\0")
}

fn select(index: &str) -> String {
    let _ = write_output(
        io::stdout(),
        &serde_json::json!({"command": "select", "index" : index}),
    );
    String::from("Selecting tab (if it exists)\0")
}

fn handle_client(stream: UnixStream) {
    let reader = BufReader::new(&stream);
    let writer = BufWriter::new(&stream);
    let writer = Arc::new(Mutex::new(writer));

    for line in reader.lines() {
        let line = &line.unwrap();
        let mut line = line.split_whitespace();
        let command = line.next().unwrap();
        let arg = line.next();

        let response = match command {
            "list" => list(),
            "open" => open(arg.unwrap()),
            "close" => close(arg.unwrap()),
            "select" => select(arg.unwrap()),

            _ => String::from("Invalid command\n"),
        };

        let mut writer = writer.lock().unwrap();
        writer.write_all(response.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}

fn main() {
    if std::path::Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH).expect("Failed to remove old socket file");
    }

    let listener = UnixListener::bind(SOCKET_PATH).unwrap();

    let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
    let listener_arc = Arc::new(listener);
    let listener_handle = listener_arc.clone();

    thread::spawn(move || {
        for stream in listener_handle.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| handle_client(stream));
                }
                _ => {
                    break;
                }
            }
        }
    });

    // Wait for shutdown signal
    for sig in signals.forever() {
        match sig {
            SIGINT | SIGTERM => {
                break;
            }
            _ => unreachable!(),
        }
    }

    // Cleanup
    drop(listener_arc);
    std::fs::remove_file(SOCKET_PATH).expect("Failed to remove socket file");
}
