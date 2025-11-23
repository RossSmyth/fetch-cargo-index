use crates_index;

#[allow(dead_code)]
#[derive(serde::Serialize)]
struct CrateJson {
    name: String,
    version: String,
    checksum: [u8; 32],
    deps: Vec<String>,
}

fn main() {
    let index = crates_index::GitIndex::with_path("../index", crates_index::git::URL).unwrap();

    let mut registry = Vec::with_capacity(10000);
    for crate_ in index.crates() {
        if let Some(highest) = crate_.highest_normal_version() {
            registry.push(CrateJson {
                name: highest.name().to_string(),
                version: highest.version().to_string(),
                checksum: *highest.checksum(),
                deps: highest.dependencies().iter().map(|dep| dep.crate_name().to_string()).collect()
            });
        }
    }

    let output = serde_json::to_string(registry.as_slice()).unwrap();

    println!("Size of JSON: {}", output.len())
}
