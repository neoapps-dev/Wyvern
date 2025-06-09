extern crate gl_generator;
extern crate wayland_scanner;

use gl_generator::{Api, Fallbacks, Profile, Registry};
use wayland_scanner::{Side, generate_code};
use std::{
    env,
    fs::File,
    process::Command,
    path::PathBuf,
};

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
   let screencopy_protocol_file = "resources/wayland-screencopy.xml";
    generate_code(
        screencopy_protocol_file,
        std::path::Path::new("src/protocols/ext_screencopy.rs"),
        Side::Server,
    );
}
