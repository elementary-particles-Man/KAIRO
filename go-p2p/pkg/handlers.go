package pkg

import (
	"fmt"
	"net/http"
)

type APIHandler struct {
	manager *Manager
}

func NewAPIHandler(manager *Manager) *APIHandler {
	fmt.Println("NewAPIHandler created with manager:", manager)
	return &APIHandler{manager: manager}
}

// ServeHTTP を実装して http.Handler にする
func (h *APIHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello from APIHandler! Manager: %+v\n", h.manager)
}
