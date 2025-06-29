package main

import (
    "context"
    "log"
    "net/http"

    "github.com/elementary-particles-Man/KAIRO/go-p2p/pkg"
)

func main() {
    ctx := context.Background()
    mgr, err := pkg.NewManager(ctx)
    if err != nil {
        log.Fatalf("failed to create p2p manager: %v", err)
    }
    defer mgr.Close()

    handler := pkg.NewAPIHandler(mgr)
    srv := &http.Server{Addr: ":8080", Handler: handler}
    log.Println("HTTP API listening on :8080")
    if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
        log.Fatalf("server error: %v", err)
    }
}
