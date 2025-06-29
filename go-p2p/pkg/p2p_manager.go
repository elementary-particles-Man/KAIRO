package pkg

import (
    "context"

    libp2p "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/host"
)

// Manager holds the libp2p host instance.
type Manager struct {
    ctx  context.Context
    Host host.Host
}

// NewManager creates a new libp2p host listening on any available port.
func NewManager(ctx context.Context) (*Manager, error) {
    h, err := libp2p.New(libp2p.ListenAddrStrings("/ip4/0.0.0.0/tcp/0"))
    if err != nil {
        return nil, err
    }
    return &Manager{ctx: ctx, Host: h}, nil
}

// Close shuts down the libp2p host.
func (m *Manager) Close() error {
    return m.Host.Close()
}

// ForceDisconnect calls into rust-core via FFI to trigger a shutdown.
func (m *Manager) ForceDisconnect() {
    forceDisconnect()
}
