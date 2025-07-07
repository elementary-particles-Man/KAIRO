use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let data = kairof::build_pcap();
    let path = std::path::Path::new("samples/kairof_sample.pcap");
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    let mut file = File::create(path)?;
    file.write_all(&data)?;
    println!("PCAP written to {}", path.display());
    Ok(())
}
