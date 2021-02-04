fn main() {
    println!(
        "cargo:rustc-link-search=all={}/nakama-cpp-sdk/libs/",
        env!("CARGO_MANIFEST_DIR")
    );
    println!(
        "cargo:rustc-link-search=all={}/nakama-cpp-sdk/libs/linux/x64",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-lib=static=nakama-cpp");
    println!("cargo:rustc-link-lib=static=cpprest");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=protobuf");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static-nobundle=stdc++");
}
