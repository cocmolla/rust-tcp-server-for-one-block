/// Tcp echo server
/// Copied from https://blog.csdn.net/LYBzhangYF/article/details/121000669
/// Modified by sulong at 2022-04-07
// Rust standard net library
use std::net::{TcpListener, TcpStream};
// Create a thread for every comming request
use std::thread;
// In order to use sleep function
use std::time;
// Standard io library
use std::io::{self, Read, Write};

const SERVER_PORT: u32 = 10086;
const BIND_ADDRESS: &str = "0.0.0.0:10086";

/// Handle every request
/// This is a simple version, response directly what client request
/// Accept max 512 bytes data in a single request
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // an u8 buf array, max 1024 bytes
    let mut buf = [0; 1024];
    // Run forever or as a daemon
    loop {
        // Read from tcp stream, save data in buf array created previously
        let bytes_read = stream.read(&mut buf)?;
        // 0 bytes data means read complete
        if bytes_read == 0 {
            return Ok(());
        }
        // Write back directly
        stream.write(&buf[..bytes_read])?;
        // Sleep 1 second
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn main() -> io::Result<()> {
    // Create tcp listener
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    // Create a vector to save thread handler
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // Dev output
    println!("Server started, listening on {}", SERVER_PORT);

    // Start listening comming request
    for stream in listener.incoming() {
        // Make sure every tcp connection established successful
        let stream = stream.expect("failed");
        // Create a thread for each connection and use closure to handle this connection
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        // Push handler to thread handler vector for further use
        thread_vec.push(handle);
    }

    // Wait thread end
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
