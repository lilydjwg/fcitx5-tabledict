fn main() {
  cc::Build::new()
    .file("src/tabledict.cc")
    .cpp(true)
    .include("/usr/include/LibIME")
    .include("/usr/include/Fcitx5/Utils")
    .flag("-std=c++17")
    .compile("tabledict");

  println!("cargo:rerun-if-changed=src/tabledict.cc");

  println!("cargo:rustc-link-lib=IMETable");
}
