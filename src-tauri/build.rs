fn main() {
    // The content is embedded (src/content.rs): rebuild when it changes.
    println!("cargo:rerun-if-changed=../content");
    // So is the submitted work (docs/submissions.md).
    println!("cargo:rerun-if-changed=../submissions");
    tauri_build::build()
}
