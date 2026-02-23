use std::env;

fn main() {
    println!("cargo:rerun-if-changed=led_btn.c");
    println!("cargo:rerun-if-changed=led_btn.h");

    let target = env::var("TARGET").unwrap_or_default();
    let mut build = cc::Build::new();

    build.file("led_btn.c");

    if target == "thumbv7em-none-eabihf" {
        build
            .flag("-mcpu=cortex-m4")
            .flag("-mthumb")
            .flag("-mfloat-abi=hard")
            .flag("-mfpu=fpv4-sp-d16");
    }

    if let Err(err) = build.try_compile("led_btn") {
        panic!(
            "failed to compile led_btn.c for {target}: {err}\n\
             Make sure an ARM C toolchain is installed and available in PATH.\n\
             Expected compiler: arm-none-eabi-gcc\n\
             You can also set CC_thumbv7em_none_eabihf and AR_thumbv7em_none_eabihf."
        );
    }
}
