//! Embeds the Zed extension (`editors/zed`) so `learn zed` can install it.
//! Release builds compile it first and point `LEARN_ZED_WASM` at the result;
//! a local `cargo build --release --target wasm32-wasip2` in editors/zed is
//! picked up too. Without either, `learn zed` says this build has none.

use std::path::{Path, PathBuf};

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let zed = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("editors").join("zed");
    let local = zed.join("target").join("wasm32-wasip2").join("release").join("zed_polysyntax.wasm");
    let manifest = zed.join("extension.toml");
    println!("cargo:rerun-if-env-changed=LEARN_ZED_WASM");
    // A path that doesn't exist would rerun this (and rebuild learn) every time.
    println!("cargo:rerun-if-changed={}", if zed.is_dir() { zed.display().to_string() } else { "build.rs".into() });

    let wasm = std::env::var_os("LEARN_ZED_WASM").map(PathBuf::from).or_else(|| local.is_file().then_some(local));
    let (wasm, manifest) = match (wasm, std::fs::read(&manifest)) {
        (Some(w), Ok(m)) => (std::fs::read(&w).unwrap_or_else(|e| panic!("LEARN_ZED_WASM: can't read {}: {e}", w.display())), m),
        _ => (Vec::new(), Vec::new()),
    };
    std::fs::write(out.join("zed.wasm"), wasm).unwrap();
    std::fs::write(out.join("zed-extension.toml"), manifest).unwrap();
}
