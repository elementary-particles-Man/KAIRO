package main

import (
	"log"
	"net/http"

	"github.com/elementary-particles-Man/KAIRO/go-p2p/pkg"
)

func main() {
	manager := pkg.NewManager()
	apiHandler := pkg.NewAPIHandler(manager)

	log.Println("Starting API server on :8080")
	log.Fatal(http.ListenAndServe(":8080", apiHandler))
}
