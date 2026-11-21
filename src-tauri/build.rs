fn main() {
    // The content is embedded (src/content.rs): rebuild when it changes.
    println!("cargo:rerun-if-changed=../content");
    tauri_build::build()
}
