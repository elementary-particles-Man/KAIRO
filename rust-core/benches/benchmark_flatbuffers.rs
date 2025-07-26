// Benchmark for FlatBuffers serialization and deserialization
// This uses Criterion to measure performance of building a FlatBuffer
// and parsing it via the PacketParser.

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use flatbuffers::FlatBufferBuilder;

// Import generated FlatBuffers schema and packet parser from the crate
use kairo_core::ai_tcp_packet_generated::aitcp as fb;
use kairo_core::packet_parser::PacketParser;

/// Helper function to build a sample AITcpPacket and return the encoded bytes
fn build_sample_packet() -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    // Create sample fields for the packet
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_id: u64 = 42;
    let seq_id_vec = builder.create_vector(&seq_id.to_le_bytes());
    let payload_vec = builder.create_vector(&[0u8; 0]);
    let signature_vec = builder.create_vector(&[0u8; 64]);

    // Build the actual packet using the generated API
    let packet_offset = fb::AITcpPacket::create(
        &mut builder,
        &fb::AITcpPacketArgs {
            version: 1,
            ephemeral_key: Some(ephemeral_key_vec),
            nonce: Some(nonce_vec),
            encrypted_sequence_id: Some(seq_id_vec),
            encrypted_payload: Some(payload_vec),
            signature: Some(signature_vec),
            header: None, // 追加
            payload: None, // 追加
            footer: None, // 追加
        },
    );

    builder.finish(packet_offset, None);
    builder.finished_data().to_vec()
}

/// Benchmark the serialization (FlatBuffer building) step
fn bench_serialize(c: &mut Criterion) {
    c.bench_function("serialize_flatbuffers", |b| {
        b.iter(|| {
            // Measure the cost of creating a FlatBuffer packet
            let _buf = black_box(build_sample_packet());
        })
    });
}

/// Benchmark the deserialization using PacketParser
fn bench_deserialize(c: &mut Criterion) {
    // Prepare a sample buffer outside the loop so we only measure parsing time
    let buffer = build_sample_packet();

    c.bench_function("deserialize_flatbuffers", |b| {
        b.iter(|| {
            // PacketParser::parse returns Result<AITcpPacket, KairoError>
            let mut parser = PacketParser::new(Vec::new());
            let packet = parser.parse(black_box(&buffer)).expect("parse failed");
            black_box(packet);
        })
    });
}

// Register the benchmark functions with Criterion
criterion_group!(benches, bench_serialize, bench_deserialize);
criterion_main!(benches);

