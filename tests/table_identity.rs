//! The four lookup tables must survive the EPIC-80 move byte-for-byte.
//! Hashes captured from the files immediately after the move, which were
//! verified identical to pkcore's copies in Task 1 Step 1.

use std::process::Command;

/// (path, expected sha256)
const TABLES: [(&str, &str); 4] = [
    (
        "src/standard52/lookups/flushes.rs",
        "e67a564444ff154df0fb2269347d79fbd338d15b299791097c1470d6a5c85986",
    ),
    (
        "src/standard52/lookups/products.rs",
        "bb7f731d73f759a6f6b696cba68e85207ba7bdf2222d74d09f0ad221e2a6b087",
    ),
    (
        "src/standard52/lookups/unique5.rs",
        "f37642ba10abdb9a26dbe33df4b31ef31175594a77a8fff3030dcc1103755126",
    ),
    (
        "src/standard52/lookups/values.rs",
        "80c3451c3d438499691938724c070e2cd3194acfd0d709a5002ac15bbf4424e4",
    ),
];

fn sha256(path: &str) -> String {
    let out = Command::new("shasum")
        .args(["-a", "256", path])
        .output()
        .expect("shasum must be available");
    String::from_utf8(out.stdout)
        .expect("utf8")
        .split_whitespace()
        .next()
        .expect("hash")
        .to_string()
}

#[test]
fn lookup_tables_unchanged() {
    for (file, expected) in TABLES {
        assert_eq!(sha256(file), expected, "lookup table {file} changed");
    }
}
