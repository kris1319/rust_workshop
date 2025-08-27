fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/indexshim.cpp")
        // .include("/opt/homebrew/Cellar/folly/2025.08.18.00/include")
        // .include("/opt/homebrew/Cellar/boost/1.89.0/include")
        // .include("/opt/homebrew/Cellar/double-conversion/3.3.1/include")
        // .include("/opt/homebrew/Cellar/gflags/2.2.2/include")
        // .include("/opt/homebrew/Cellar/fmt/11.2.0/include")
        // .include("/opt/homebrew/Cellar/glog/0.6.0/include")
        // .include("/opt/homebrew/Cellar/gflags/2.2.2/include")
        // .include("/opt/homebrew/Cellar/libevent/2.1.12_1/include")
        // .include("/opt/homebrew/Cellar/lz4/1.10.0/include")
        // .include("/opt/homebrew/Cellar/snappy/1.2.2/include")
        .std("c++17")
        .compile("rust_cxx");

    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/folly/2025.08.18.00/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/boost/1.89.0/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/double-conversion/3.3.1/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/fmt/11.2.0/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/glog/0.6.0/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/gflags/2.2.2/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/libevent/2.1.12_1/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/lz4/1.10.0/lib");
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/snappy/1.2.2/lib");

    // println!("cargo:rustc-link-lib=static=folly");

    // println!("cargo:rustc-link-lib=static=fmt");
    // println!("cargo:rustc-link-lib=static=glog");
    // println!("cargo:rustc-link-lib=static=gflags");
    // println!("cargo:rustc-link-lib=static=lz4");
    // println!("cargo:rustc-link-lib=static=z");
    // println!("cargo:rustc-link-lib=static=double-conversion");
    // println!("cargo:rustc-link-lib=static=event");
    // println!("cargo:rustc-link-lib=static=snappy");
    // println!("cargo:rustc-link-lib=static=event_pthreads");
    // println!("cargo:rustc-link-lib=static=boost_context");

    println!("cargo:rerun-if-changed=include/index.h");
    println!("cargo:rerun-if-changed=include/indexshim.h");
    // println!("cargo:rerun-if-changed=include/storage.h");
}