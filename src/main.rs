extern crate clang_sys;

fn main() {
    let lib = match clang_sys::load_manually() {
        Ok (lib) => lib,
        Err (message) => panic!("Cannot load library! [{}]", message)
    };

    let idx = unsafe{(lib.functions.clang_createIndex.unwrap())(1, 1)};
    unsafe{(lib.functions.clang_disposeIndex.unwrap())(idx)};
    
    println!("Lib version: {:?}", lib.version().expect("Unable to read into libclang"));
    println!("Lib path: {:?}", lib.path().display().to_string());
}
