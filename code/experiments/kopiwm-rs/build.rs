use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xft");
    println!("cargo:rustc-link-lib=fontconfig");

    let mut builder = bindgen::Builder::default();

    let freetype2 = pkg_config::probe_library("freetype2")
        .expect("Could not find freetype2 via pkg-config");
    for incl in freetype2.include_paths {
        builder = builder.clang_arg(format!("-I{}", incl.display()))
    }

    let bindings = builder
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(Path::new("src").join("generated_bindings.rs"))
        .expect("Couldn't write bindings!");
}
