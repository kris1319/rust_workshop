fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/indexshim.cpp")
        .std("c++17")
        .compile("rust_cxx");

    println!("cargo:rerun-if-changed=include/index.h");
    println!("cargo:rerun-if-changed=include/indexshim.h");
}