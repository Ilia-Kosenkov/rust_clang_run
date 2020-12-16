extern crate clang_sys;

fn main() {
   do_work();
}


#[cfg(feature = "runtime")]
fn do_work() {
    let lib = match clang_sys::load_manually() {
        Ok (lib) => lib,
        Err (message) => panic!("Cannot load library! [{}]", message)
    };

    let idx = unsafe{(lib.functions.clang_createIndex.unwrap())(1, 1)};
    unsafe{(lib.functions.clang_disposeIndex.unwrap())(idx)};
    
    println!("Lib version: {:?}", lib.version().expect("Unable to read into libclang"));
    println!("Lib path: {:?}", lib.path().display().to_string());
}

#[cfg(feature = "static")]
fn do_work() {
    unimplemented!();
}


fn do_work() {
    panic!("Run program with one of the features: \"static\" or \"runtime\"");
}