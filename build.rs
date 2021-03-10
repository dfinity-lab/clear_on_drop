extern crate cc;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("no TARGET_ARCH set");

    if cfg!(not(feature = "no_cc")) && target_arch != "wasm32" {
        cc::Build::new()
            .file("src/hide.c")
            .compile("clear_on_drop");
    }
}
