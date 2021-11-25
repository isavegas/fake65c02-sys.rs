fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=fake65c02/fake6502.h");
    println!("cargo:rerun-if-changed=fake65c02/fake6502.c");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=LDFLAGS");
    println!("cargo:rerun-if-env-changed=AR");

    let mut builder = cc::Build::new();
    builder
        .file("fake65c02/fake65c02.c")
        .flag("-Wno-unused-parameter")
        .define("UNDOCUMENTED", "1")
        .compile("fake65c02");
}
