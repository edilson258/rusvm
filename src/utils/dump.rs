use crate::JavaClassFile;

pub fn dump_class_file(class: &JavaClassFile) {
    println!("Magic: 0x{:x}", class.magic);
    println!("Minor Version: 0x{:x}", class.minor);
    println!("Major Version: 0x{:x}", class.major);
    println!("Access Flags: {:?}", class.access_flags);
    println!(
        "This Class : {:?}\nSuper Class: {:?}",
        class.this_class, class.super_class
    );
    println!("{:#?}", class.constant_pool);
    println!("{:#?}", class.methods);
    println!("{:#?}", class.attrs);
}
