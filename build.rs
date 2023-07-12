fn main() {
    let mut build = cc::Build::new();
    build.cpp(true).file("src/lib1.cpp").file("src/lib2.cpp").flag("-ffunction-sections").compile("lib_cpp");

    println!("cargo:rerun-if-changed=src/lib.cpp");
}
