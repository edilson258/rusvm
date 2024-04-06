use crate::{utils::bytestream::ByteStream, JavaClassFile};

use super::{
    accessflags::parse_class_access_flags, attributes::parse_attrs, constantpool::ConstantPool,
    method::parse_methods,
};

pub struct JavaClassFileParser {
    bytes: ByteStream,
    class: JavaClassFile,
}

impl JavaClassFileParser {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: ByteStream { xs: bytes },
            class: JavaClassFile::default(),
        }
    }

    pub fn parse(&mut self) -> JavaClassFile {
        self.class.magic = self.bytes.parse_u4();
        self.class.minor = self.bytes.parse_u2();
        self.class.major = self.bytes.parse_u2();
        self.class.constant_pool = ConstantPool::default().parse(&mut self.bytes);
        self.class.access_flags = parse_class_access_flags(self.bytes.parse_u2());

        self.class.this_class = self
            .class
            .constant_pool
            .query(self.bytes.parse_u2() as usize);
        self.class.super_class = self
            .class
            .constant_pool
            .query(self.bytes.parse_u2() as usize);

        let interfaces_count = self.bytes.parse_u2();
        for _ in 0..interfaces_count {
            assert!(
                false,
                "[ERROR]:{}:{}: Interface parser not implemented yet\n",
                file!(),
                line!()
            );
        }

        let fields_count = self.bytes.parse_u2();
        for _ in 0..fields_count {
            assert!(
                false,
                "[ERROR]:{}:{}: Field parser not implemented yet\n",
                file!(),
                line!()
            );
        }

        self.class.methods = parse_methods(&self.class, &mut self.bytes);
        self.class.attrs = parse_attrs(&mut self.bytes, &self.class.constant_pool);

        self.class.to_owned()
    }
}
