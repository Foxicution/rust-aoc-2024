pub fn read_input(file_name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fp = format!("{}/data/{}", manifest_dir, file_name);
    std::fs::read_to_string(fp).expect("d1.txt should exist in data/ directory")
}
