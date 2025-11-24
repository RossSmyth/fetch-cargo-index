use rayon::iter::ParallelIterator;

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

    let registry: Vec<CrateJson> = index
        .crates_parallel()
        .filter_map(|c| c.ok())
        .filter_map(|c| c.highest_normal_version().cloned())
        .map(|c| CrateJson {
            name: c.name().to_string(),
            version: c.version().to_string(),
            checksum: *c.checksum(),
            deps: c
                .dependencies()
                .iter()
                .map(|dep| dep.crate_name().to_string())
                .collect(),
        })
        .collect();

    let output = serde_json::to_string(registry.as_slice()).unwrap();

    println!("Size of JSON: {}", output.len())
}
