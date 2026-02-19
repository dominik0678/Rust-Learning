fn main() {
    println!("cargo:rerun-if-changed=led_btn.c");
    println!("cargo:rerun-if-changed=led_btn.h");

    cc::Build::new()
        .file("led_btn.c")
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .compile("led_btn");
}
