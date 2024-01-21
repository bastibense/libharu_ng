// add libharu

fn main() {
    println!(
        "cargo:rustc-link-search=native={}",
        "/opt/homebrew/opt/libharu/lib"
    );

    // look in /usr/local/lib for libharu
    println!("cargo:rustc-link-search=/usr/local/lib");

    println!("cargo:rustc-link-lib=hpdf");
}
