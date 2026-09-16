mod native_preview;

use std::{fs, path::PathBuf, process::Command};

fn main() {
    println!("Hello from Rust. Generating a 2D scatter plot.");

    let output_file = output_file_path();
    let html = native_preview::build_scatter_demo_html();

    fs::write(&output_file, html).expect("failed to write 2D scatter HTML");

    #[cfg(target_os = "windows")]
    {
        println!("Opening 2D scatter plot...");
        let _ = Command::new("cmd")
            .args(["/C", "start", "", output_file.to_str().expect("invalid output path")])
            .spawn();
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("Open this file in a browser: {}", output_file.display());
    }
}

fn output_file_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .unwrap_or(&manifest_dir)
        .to_path_buf();

    workspace_root.join("2dscatter.html")
}