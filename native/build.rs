extern crate bindgen;

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .whitelist_function("napi_.+")
        .whitelist_type("napi_.+")
        .header("./src/napi_sys/node_api.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("./src/napi_sys/bindings.rs")
        .expect("Couldn't write bindings!");
}
