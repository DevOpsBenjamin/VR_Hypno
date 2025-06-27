fn main() {
    // Copier les ressources dans le bundle
    println!("cargo:rerun-if-changed=resources/default-data.zip");
    tauri_build::build()
}