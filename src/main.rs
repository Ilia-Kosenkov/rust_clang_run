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
    let ptr_size = std::mem::size_of::<usize>();
    println!("Ptr size {:?}", ptr_size);
    
    println!("Index created by `libclang`: {:?}", idx);

    unsafe{(lib.functions.clang_disposeIndex.unwrap())(idx)};
    
    println!("Lib version: {:?}", lib.version().expect("Unable to read into libclang"));
    println!("Lib path: {:?}", lib.path().display().to_string());
}

#[cfg(feature = "static")]
fn do_work() {
    let idx = unsafe{ clang_sys::clang_createIndex(1, 1) };
    let ptr_size = std::mem::size_of::<usize>();
    println!("Ptr size {:?}", ptr_size);
    println!("Index created by `libclang`: {:?}", idx);
    unsafe{ clang_sys::clang_disposeIndex(idx) };
}

#[cfg(all(not(feature = "runtime"), not(feature = "static")))]
fn do_work() {
    panic!("Run program with one of the features: \"static\" or \"runtime\"");
}