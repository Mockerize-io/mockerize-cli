use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if cfg!(target_os = "windows") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let manifest_path = PathBuf::from("resources/Manifest.xml");
        let dest_path = PathBuf::from(out_dir).join("Manifest.xml");

        fs::copy(manifest_path, dest_path).unwrap();
        println!("cargo:rustc-link-arg-bin=mockerize-cli=/MANIFEST:embed");
        println!("cargo:rerun-if-changed=resources/Manifest.xml");
    }
}
