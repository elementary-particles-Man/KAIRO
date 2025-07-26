package pkg

/*
#cgo LDFLAGS: -L../../rust-core/target/release -lrust_core
void force_disconnect();
void log_parse_error();
*/
import "C"

// forceDisconnect invokes the Rust core library function.
func forceDisconnect() {
    C.force_disconnect()
}

// LogParseError calls into the Rust library to log parsing failures.
func LogParseError() {
    C.log_parse_error()
}
