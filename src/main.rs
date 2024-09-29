include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let lib = unsafe { TestLib::new("./foo.dll") }.unwrap();

    let _ = unsafe { lib.SSV_InitLIB2(c"foo".as_ptr()) };

    todo!();
}
