package pkg

import (
	"fmt"
)

type Manager struct {
	// TODO: 必要に応じて P2P の状態管理フィールドを追加
}

func NewManager() *Manager {
	fmt.Println("NewManager created!")
	return &Manager{}
}
