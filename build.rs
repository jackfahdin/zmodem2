// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2017-2020 Alexey Arbuzov
// Copyright (c) 2023-2025 Jarkko Sakkinen

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_lrzsz)");

    let rz = find_prog("ZMODEM_RZ_BIN", &["rz", "lrzsz-rz"]);
    let sz = find_prog("ZMODEM_SZ_BIN", &["sz", "lrzsz-sz"]);

    // An absent lrzsz is deliberately silent. `has_lrzsz` only gates this crate's
    // own integration tests, so a consumer that never builds them has nothing to
    // act on, and lrzsz is not normally installed on Windows or macOS.
    if let (Some(rz_path), Some(sz_path)) = (rz, sz) {
        println!("cargo:rustc-cfg=has_lrzsz");
        println!("cargo:rustc-env=ZMODEM_RZ_BIN={}", rz_path.display());
        println!("cargo:rustc-env=ZMODEM_SZ_BIN={}", sz_path.display());
    }
}

fn find_prog(env_var: &str, candidates: &[&str]) -> Option<PathBuf> {
    if let Ok(path) = env::var(env_var) {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }

    let path = env::var_os("PATH")?;
    for dir in env::split_paths(&path) {
        for &name in candidates {
            let full = dir.join(name);
            if full.is_file() {
                return Some(full);
            }
        }
    }

    None
}
