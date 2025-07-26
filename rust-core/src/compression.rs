use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use std::error::Error;
use std::io::{Read, Write};

/// Supported compression algorithms for packet payloads.
pub enum CompressionAlgorithm {
    Lz4,
    Zstd,
}

/// Compress data using the specified algorithm.
pub fn compress(data: &[u8], algo: CompressionAlgorithm) -> Result<Vec<u8>, Box<dyn Error>> {
    match algo {
        CompressionAlgorithm::Lz4 => Ok(compress_prepend_size(data)),
        CompressionAlgorithm::Zstd => {
            let mut encoder = zstd::Encoder::new(Vec::new(), 0)?;
            encoder.write_all(data)?;
            let data = encoder.finish()?
                .to_vec();
            Ok(data)
        }
    }
}

/// Decompress data that was compressed with the specified algorithm.
pub fn decompress(data: &[u8], algo: CompressionAlgorithm) -> Result<Vec<u8>, Box<dyn Error>> {
    match algo {
        CompressionAlgorithm::Lz4 => Ok(decompress_size_prepended(data)?),
        CompressionAlgorithm::Zstd => {
            let mut decoder = zstd::Decoder::new(data)?;
            let mut buf = Vec::new();
            decoder.read_to_end(&mut buf)?;
            Ok(buf)
        }
    }
}
