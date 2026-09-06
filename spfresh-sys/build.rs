use std::env;
use std::path::PathBuf;

fn main() {
    let spfresh_root = env::var("SPFRESH_ROOT")
        .expect("SPFRESH_ROOT must be set to the SPFresh source directory");
    let rocksdb_root = env::var("ROCKSDB_ROOT")
        .expect("ROCKSDB_ROOT must be set to the RocksDB install prefix");

    println!("cargo:rerun-if-env-changed=SPFRESH_ROOT");
    println!("cargo:rerun-if-env-changed=ROCKSDB_ROOT");
    println!("cargo:rerun-if-changed=spfresh_c.h");
    println!("cargo:rerun-if-changed=spfresh_c.cpp");

    // Compile our thin C wrapper
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .include(format!("{}/AnnService", spfresh_root))
        .file("spfresh_c.cpp")
        .compile("spfresh_c");

    // SPTAG internal libraries
    println!("cargo:rustc-link-search=native={}/Release", spfresh_root);
    println!("cargo:rustc-link-lib=static=SPTAGLibStatic");
    println!("cargo:rustc-link-lib=static=DistanceUtils");
    println!("cargo:rustc-link-lib=static=zstd");

    // RocksDB
    println!("cargo:rustc-link-search=native={}/lib", rocksdb_root);
    println!("cargo:rustc-link-lib=static=rocksdb");

    // System dependencies
    println!("cargo:rustc-link-lib=tbb");
    println!("cargo:rustc-link-lib=numa");
    println!("cargo:rustc-link-lib=gomp");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=snappy");
    println!("cargo:rustc-link-lib=jemalloc");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=rt");

    // Generate bindings with bindgen
    let bindings = bindgen::Builder::default()
        .header("spfresh_c.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
