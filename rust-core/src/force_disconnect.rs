use std::io;
use std::net::{Shutdown, TcpStream};

/// Attempt to forcefully disconnect a TCP stream by shutting it down.
pub fn force_disconnect(stream: &TcpStream) -> io::Result<()> {
    stream.shutdown(Shutdown::Both)
}
