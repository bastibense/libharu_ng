// add libharu

fn main() {
    println!(
        "cargo:rustc-link-search=native={}",
        "/opt/homebrew/opt/libharu/lib"
    );
    println!("cargo:rustc-link-lib=hpdf");
}
