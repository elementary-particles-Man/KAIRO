package pkg

/*
#cgo LDFLAGS: -L../../rust-core/target/release -lrust_core
void force_disconnect();
*/
import "C"

// forceDisconnect invokes the Rust core library function.
func forceDisconnect() {
    C.force_disconnect()
}
