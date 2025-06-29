package pkg

import (
    "encoding/binary"
)

// Packet represents the AI-TCP packet structure.
type Packet struct {
    Version            byte
    EphemeralKey       []byte
    Nonce              []byte
    EncryptedSeqID     []byte
    EncryptedPayload   []byte
    Signature          []byte
}

// Serialize converts the packet into the binary format used by the Rust parser.
func Serialize(p Packet) []byte {
    size := 1 + 1 + len(p.EphemeralKey) + 1 + len(p.Nonce) + 2 + len(p.EncryptedSeqID) + 2 + len(p.EncryptedPayload) + 2 + len(p.Signature)
    out := make([]byte, 0, size)
    out = append(out, p.Version)
    out = append(out, byte(len(p.EphemeralKey)))
    out = append(out, p.EphemeralKey...)
    out = append(out, byte(len(p.Nonce)))
    out = append(out, p.Nonce...)
    tmp := make([]byte, 2)
    binary.BigEndian.PutUint16(tmp, uint16(len(p.EncryptedSeqID)))
    out = append(out, tmp...)
    out = append(out, p.EncryptedSeqID...)
    binary.BigEndian.PutUint16(tmp, uint16(len(p.EncryptedPayload)))
    out = append(out, tmp...)
    out = append(out, p.EncryptedPayload...)
    binary.BigEndian.PutUint16(tmp, uint16(len(p.Signature)))
    out = append(out, tmp...)
    out = append(out, p.Signature...)
    return out
}

