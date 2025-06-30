package pkg

import (
    "github.com/google/flatbuffers/go"
    "github.com/elementary-particles-Man/KAIRO/go-p2p/pkg/generated/AITCP"
)

func SerializeAITcpPacket(seqID uint64, payload []byte) []byte {
    builder := flatbuffers.NewBuilder(0)

    // 必要ならベクトル化
    ephemeralKey := builder.CreateByteVector(make([]byte, 32))
    nonce := builder.CreateByteVector(make([]byte, 12))
    encryptedSeqID := builder.CreateByteVector([]byte{
        byte(seqID >> 56), byte(seqID >> 48), byte(seqID >> 40), byte(seqID >> 32),
        byte(seqID >> 24), byte(seqID >> 16), byte(seqID >> 8), byte(seqID),
    })
    encryptedPayload := builder.CreateByteVector(payload)
    signature := builder.CreateByteVector(make([]byte, 64)) // 仮でダミー

    AITCP.AITcpPacketStart(builder)
    AITCP.AITcpPacketAddVersion(builder, 1)
    AITCP.AITcpPacketAddEphemeralKey(builder, ephemeralKey)
    AITCP.AITcpPacketAddNonce(builder, nonce)
    AITCP.AITcpPacketAddEncryptedSequenceId(builder, encryptedSeqID)
    AITCP.AITcpPacketAddEncryptedPayload(builder, encryptedPayload)
    AITCP.AITcpPacketAddSignature(builder, signature)
    pkt := AITCP.AITcpPacketEnd(builder)

    builder.Finish(pkt)
    return builder.FinishedBytes()
}
