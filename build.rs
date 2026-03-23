use std::{fs, io::Write};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(io_uring_skip_arch_check)");
    println!("cargo:rustc-check-cfg=cfg(io_uring_use_own_sys)");

    let mut builder = bindgen::builder();

    builder = builder.header_contents(
        "include.h",
        "#include <linux/io_uring.h>
#include <sys/syscall.h>",
    );

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./src/generated_bindings.rs")
        .unwrap();

    // unsafe_op_in_unsafe_fn,
    // unused,
    // dead_code,
    // non_upper_case_globals
    // + more, just allowing warnings is easier
    const BINDGEN_PREFACE: &str = "#![allow(
    clippy::all,
    warnings
)]";

    writeln!(&mut file, "{}", BINDGEN_PREFACE)
        .expect("Should be able to write to our generated bindings file");

    builder
        .generate_comments(true)
        .derive_default(true)
        .use_core()
        .generate()
        .unwrap()
        .write(Box::new(&file))
        .unwrap();
}
