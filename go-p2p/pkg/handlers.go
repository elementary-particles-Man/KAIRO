package pkg

import (
    "encoding/json"
    "net/http"
)

// APIHandler exposes HTTP endpoints to interact with the P2P manager.
type APIHandler struct{
    M *Manager
}

func NewAPIHandler(m *Manager) *APIHandler {
    return &APIHandler{M: m}
}

func (h *APIHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
    switch r.URL.Path {
    case "/force_disconnect":
        h.M.ForceDisconnect()
        json.NewEncoder(w).Encode(map[string]bool{"disconnected": true})
    default:
        w.WriteHeader(http.StatusNotFound)
    }
}
