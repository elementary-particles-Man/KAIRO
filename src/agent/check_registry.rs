use kairo_lib::registry::load_registry;
use std::collections::HashSet;

fn main() {
    let path = "agent_registry.json";
    match load_registry(path) {
        Ok(entries) => {
            let mut names = HashSet::new();
            let mut addrs = HashSet::new();
            for e in &entries {
                if !names.insert(&e.name) {
                    eprintln!("Duplicate agent name found: {}", e.name);
                    std::process::exit(1);
                }
                if !addrs.insert(&e.p_address) {
                    eprintln!("P address collision: {}", e.p_address);
                    std::process::exit(1);
                }
            }
            println!("Registry OK: {} entries", entries.len());
        }
        Err(e) => {
            eprintln!("Failed to load registry: {}", e);
            std::process::exit(1);
        }
    }
}
