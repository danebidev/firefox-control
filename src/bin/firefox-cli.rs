use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;

const SOCKET_PATH: &str = "/tmp/firefox-extension.sock";

#[derive(Debug)]
enum Command {
    List,
    Open(Option<String>),
    Close(String),
    Select(String),
}

fn usage_info() {
    println!("Usage: firefox-cli [COMMAND]\n");
    println!("Commands:");
    println!("  list                List all open tabs");
    println!("  open [URL]          Open a new tab with the given URL. If no URL is given, open a new tab with the default homepage");
    println!("  close <index>       Close the tab at the given index");
    println!("  select <index>      Select the tab at the given index");
}

impl Command {
    unsafe fn execute(&self, stream: UnixStream) {
        let command = match self {
            Command::List => "list\n",
            Command::Open(url) => {
                let url = url.as_deref().unwrap_or("about:blank");
                &format!("open {}\n", url)[..]
            }
            Command::Close(index) => &format!("close {}\n", index)[..],
            Command::Select(index) => &format!("select {}\n", index)[..],
        };

        let mut writer = BufWriter::new(&stream);
        writer
            .write_all(command.as_bytes())
            .expect("Failed to write to the server");
        writer.flush().expect("Failed to flush the writer");

        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader
            .read_until(b'\0', response.as_mut_vec())
            .expect("Failed to read from the server");

        println!("{}", response);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing command");
        usage_info();
        std::process::exit(1);
    }

    let command = match args[1].as_str() {
        "list" => Command::List,
        "open" => Command::Open(args.get(2).cloned()),
        "close" => Command::Close(args.get(2).expect("Missing tab index").clone()),
        "select" => Command::Select(args.get(2).expect("Missing tab index").clone()),
        _ => {
            println!("Invalid command");
            usage_info();
            std::process::exit(1);
        }
    };

    let stream = UnixStream::connect(SOCKET_PATH).expect("Failed to connect to the server");

    unsafe {
        command.execute(stream);
    }
}
