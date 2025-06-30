use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Tell cargo to rerun if the eBPF program changes
    println!("cargo:rerun-if-changed=../fw-ebpf/src");

    // Compile the eBPF program
    let mut cmd = std::process::Command::new("cargo");
    cmd.current_dir("../fw-ebpf")
        .env("CARGO_CFG_TARGET_ARCH", "bpf")
        .args(&[
            "build",
            "--target=bpfel-unknown-none",
            "--release",
        ]);

    let output = cmd.output().expect("Failed to build eBPF program");

    if !output.status.success() {
        panic!(
            "Failed to build eBPF program:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Copy the compiled eBPF object to our output directory
    let ebpf_obj = PathBuf::from("../fw-ebpf/target/bpfel-unknown-none/release/fw-ebpf");
    let dest = out_dir.join("fw-ebpf.o");

    std::fs::copy(&ebpf_obj, &dest)
        .unwrap_or_else(|e| panic!("Failed to copy eBPF object from {:?} to {:?}: {}", ebpf_obj, dest, e));

    println!("cargo:rustc-env=EBPF_OBJECT_PATH={}", dest.display());
}
