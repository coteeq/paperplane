use cmake;

fn main() {
    let openssl_libs = vec![
        "/usr/local/Cellar/openssl@1.1/1.1.1g/lib/libssl.dylib",
        "/usr/local/Cellar/openssl@1.1/1.1.1g/lib/libcrypto.dylib"
    ].join(";");

    let dst = cmake::Config::new("td")
        .define("OPENSSL_ROOT_DIR", "/usr/local/Cellar/openssl@1.1/1.1.1g/")
        .define("OPENSSL_LIBRARIES", openssl_libs)
        .generator("Ninja")
        .profile("Release")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=tdjson");
}
