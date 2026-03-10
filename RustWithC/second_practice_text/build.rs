fn main() {
    println!("cargo:rerun-if-changed=c_lib/text_tools.c");
    println!("cargo:rerun-if-changed=c_lib/text_tools.h");

    cc::Build::new()
        .file("c_lib/text_tools.c")
        .include("c_lib")
        .compile("text_tools");
}
