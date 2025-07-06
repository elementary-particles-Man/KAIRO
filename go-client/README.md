# KAIRO Go Client

This package provides a lightweight SDK for talking to the KAIRO API. It hides all cryptographic details while supporting connection reuse and adaptive rate limiting.

## Usage

```go
import (
    "context"
    klient "github.com/elementary-particles-Man/KAIRO/go-client"
)

func main() {
    c := klient.Connect("https://api.kairo.local", "my-api-key")

    var resp map[string]any
    if err := c.Send(context.Background(), "/generate", map[string]any{"foo": "bar"}, &resp); err != nil {
        panic(err)
    }

    stream, _ := c.Stream(context.Background(), "/events")
    for msg := range stream {
        // handle msg
        _ = msg
    }
}
```
