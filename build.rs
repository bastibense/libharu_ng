// Copyright (c) 2023-2024 Bastian Bense
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Contact: Bastian Bense, bb@neosw.de

fn main() {
    // let dst = cmake::build("libharu");

    let outdir = std::env::var("OUT_DIR").unwrap();

    let dst = cmake::Config::new("libharu")
        .define("CMAKE_INSTALL_PREFIX", outdir)
        .build();

    // add /opt/homebrew/opt to search
    println!("cargo:rustc-link-search=/opt/homebrew/opt/libpng/lib");
    println!("cargo:rustc-link-search=/opt/homebrew/opt/zlib/lib");

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=static=hpdf");
    println!("cargo:rustc-link-lib=dylib=png");
    println!("cargo:rustc-link-lib=dylib=z");
}
