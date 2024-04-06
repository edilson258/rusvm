use crate::utils::bytestream::ByteStream;
use crate::{Attr, JavaClassFile};

use super::accessflags::parse_method_access_flags;
use super::attributes::parse_attrs;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Method {
    pub access_flags: Vec<String>,
    pub name: String,
    pub descriptor: String,
    pub attrs: Vec<Attr>,
}

pub fn parse_methods(class: &JavaClassFile, bytes: &mut ByteStream) -> Vec<Method> {
    let mut methods: Vec<Method> = vec![];

    for _ in 0..bytes.parse_u2() {
        let mask = bytes.parse_u2();
        let name_index = bytes.parse_u2();
        let descriptor_index = bytes.parse_u2();

        methods.push(Method {
            access_flags: parse_method_access_flags(mask),
            name: class.constant_pool.query(name_index as usize),
            descriptor: class.constant_pool.query(descriptor_index as usize),
            attrs: parse_attrs(bytes, &class.constant_pool),
        });
    }
    methods
}
