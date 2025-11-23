use crates_index;

const INDEX_LOCATION: &str = "~/crates.io-index";

#[allow(dead_code)]
struct CrateJson {
    name: String,
    version: String,
    checksum: [u8; 32],
}

fn main() {
    let index = crates_index::GitIndex::try_with_path(INDEX_LOCATION, crates_index::git::URL).unwrap().unwrap();

    let mut registry = Vec::with_capacity(10000);
    for crate_ in index.crates() {
        if let Some(highest) = crate_.highest_normal_version() {
            registry.push(CrateJson {
                name: highest.name().to_string(),
                version: highest.version().to_string(),
                checksum: *highest.checksum(),
            });
        }
    }

    println!("Minimum bound of crates: {}", registry.len())
}
