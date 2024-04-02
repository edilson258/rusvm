use std::{fs::File, io::Read, ops::BitAnd, process::exit, usize};

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_INTERFACEMETHODREF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAMEANDTYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_METHODHANDLE: u8 = 15;
const CONSTANT_METHODTYPE: u8 = 16;
const CONSTANT_INVOKEDYNAMIC: u8 = 18;

const CLASS_ACCESS_FLAGS: [(&str, u16); 8] = [
    ("ACC_PUBLIC", 0x0001),
    ("ACC_FINAL", 0x0010),
    ("ACC_SUPER", 0x0020),
    ("ACC_INTERFACE", 0x0200),
    ("ACC_ABSTRACT", 0x0400),
    ("ACC_SYNTHETIC", 0x1000),
    ("ACC_ANNOTATION", 0x2000),
    ("ACC_ENUM", 0x4000),
];

const METHOD_ACCESS_FLAGS: [(&str, u16); 12] = [
    ("ACC_PUBLIC", 0x0001),
    ("ACC_PRIVATE", 0x0002),
    ("ACC_PROTECTED", 0x0004),
    ("ACC_STATIC", 0x0008),
    ("ACC_FINAL", 0x0010),
    ("ACC_SYNCHRONIZED", 0x0020),
    ("ACC_BRIDGE", 0x0040),
    ("ACC_VARARGS", 0x0080),
    ("ACC_NATIVE", 0x0100),
    ("ACC_ABSTRACT", 0x0400),
    ("ACC_STRICT", 0x0800),
    ("ACC_SYNTHETIC", 0x1000),
];

fn main() {
    let file_path = "samples/Main.class";
    let content = read_file2bytes(file_path);
    let mut parser = ParseClass::new(&content);
    parser.parse();
}

#[derive(Debug)]
struct MethodAttr {
    name: String,
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct Method {
    access_flags: Vec<String>,
    name: String,
    descriptor: String,
    attrs: Vec<MethodAttr>,
}

#[derive(Debug)]
struct ConstantPoolInfo {
    tag: u8,
    tag_name: String,
    entries: Vec<(String, u16)>,
    bytes: Option<String>,
}

#[derive(Default, Debug)]
struct ConstantPool {
    info: Vec<ConstantPoolInfo>,
}

#[derive(Default)]
struct JavaClassFile {
    magic: u32,
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: Vec<String>,
    this_class: String,
    super_class: String,
    methods: Vec<Method>,
}

impl JavaClassFile {
    pub fn dump(&self) {
        println!("Magic: 0x{:x}", self.magic);
        println!("Minor Version: 0x{:x}", self.minor);
        println!("Major Version: 0x{:x}", self.major);
        println!("{:#?}", self.constant_pool);
        println!("Access Flags: {:?}", self.access_flags);
        println!(
            "This Class : {:?}, Super Class: {:?}",
            self.this_class, self.super_class
        );
        println!("{:#?}", self.methods);
    }
}

struct ParseClass<'a> {
    rawbytes: &'a [u8],
    class: JavaClassFile,
}

impl<'a> ParseClass<'a> {
    pub fn new(rawbytes: &'a [u8]) -> Self {
        Self {
            rawbytes,
            class: JavaClassFile::default(),
        }
    }

    pub fn parse(&mut self) {
        self.class.magic = self.parse_u4();
        self.class.minor = self.parse_u2();
        self.class.major = self.parse_u2();
        self.class.constant_pool = self.parse_constant_pool();
        let access_flag_mask = self.parse_u2();
        self.class.access_flags = self.parse_access_flags(access_flag_mask, &CLASS_ACCESS_FLAGS);

        let this_class_index = self.parse_u2();
        let super_class_index = self.parse_u2();
        self.class.this_class = self
            .constant_pool_query(this_class_index as usize)
            .unwrap();
        self.class.super_class = self
            .constant_pool_query(1 + super_class_index as usize)
            .unwrap();

        let interfaces_count = self.parse_u2();
        for _ in 0..interfaces_count {
            assert!(
                false,
                "[ERROR]:{}:{}: Interface parser not implemented yet\n",
                file!(),
                line!()
            );
        }

        let fields_count = self.parse_u2();
        for _ in 0..fields_count {
            assert!(
                false,
                "[ERROR]:{}:{}: Field parser not implemented yet\n",
                file!(),
                line!()
            );
        }

        self.class.methods = self.parse_methods();

        self.class.dump();
    }

    fn parse_methods(&mut self) -> Vec<Method> {
        let mut methods: Vec<Method> = vec![];

        for _ in 0..self.parse_u2() {
            let mask = self.parse_u2();
            let name_index = self.parse_u2();
            let descriptor_index = self.parse_u2();

            methods.push(Method {
                access_flags: self.parse_access_flags(mask, &METHOD_ACCESS_FLAGS),
                name: self
                    .constant_pool_query((name_index - 1) as usize)
                    .unwrap(),
                descriptor: self
                    .constant_pool_query((descriptor_index - 1) as usize)
                    .unwrap(),
                attrs: self.parse_method_attrs(),
            });
        }
        methods
    }

    fn parse_method_attrs(&mut self) -> Vec<MethodAttr> {
        let mut attrs: Vec<MethodAttr> = vec![];
        for _ in 0..self.parse_u2() {
            let name_index = self.parse_u2();
            let length = self.parse_u4();

            attrs.push(MethodAttr {
                name: self
                    .constant_pool_query((name_index - 1) as usize)
                    .unwrap(),
                bytes: self.parse_n(length as usize).to_vec(),
            });
        }
        attrs
    }

    pub fn parse_access_flags(&self, mask: u16, access_flags: &[(&str, u16)]) -> Vec<String> {
        let mut flags: Vec<String> = vec![];
        for (name, value) in access_flags {
            if mask.bitand(value) != 0 {
                flags.push(name.to_string());
            }
        }
        flags
    }

    pub fn parse_constant_pool(&mut self) -> ConstantPool {
        let mut constant_pool = ConstantPool { info: vec![] };

        let count = self.parse_u2();
        for _ in 0..(count - 1) {
            let tag = self.parse_u1();
            match tag {
                CONSTANT_CLASS => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_CLASS".to_string(),
                    entries: vec![("name_index".to_string(), self.parse_u2())],
                    bytes: None,
                }),
                CONSTANT_METHODREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_METHODREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), self.parse_u2()),
                        ("name_and_type_index".to_string(), self.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_NAMEANDTYPE => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_NAMEANDTYPE".to_string(),
                    entries: vec![
                        ("name_index".to_string(), self.parse_u2()),
                        ("descriptor_index".to_string(), self.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_UTF8 => {
                    let length = self.parse_u2();
                    constant_pool.info.push(ConstantPoolInfo {
                        tag,
                        tag_name: "CONSTANT_UTF8".to_string(),
                        entries: vec![("length".to_string(), length)],
                        bytes: Some(
                            String::from_utf8(self.parse_n(length as usize).to_vec()).unwrap(),
                        ),
                    })
                }
                CONSTANT_FIELDREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_FIELDREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), self.parse_u2()),
                        ("name_and_type_index".to_string(), self.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_STRING => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_STRING".to_string(),
                    entries: vec![("string_index".to_string(), self.parse_u2())],
                    bytes: None,
                }),
                _ => {
                    eprintln!("[ERROR]:{}:{}: Unknown tag {}", file!(), line!(), tag);
                    exit(1);
                }
            }
        }

        constant_pool
    }

    fn constant_pool_query(&self, index: usize) -> Option<String> {
        self.class.constant_pool.info.get(index).unwrap().bytes.clone()
    }

    fn parse_u1(&mut self) -> u8 {
        if self.rawbytes.len() < 1 {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = self.rawbytes[0];
        self.rawbytes = &self.rawbytes[1..];
        res
    }

    fn parse_u2(&mut self) -> u16 {
        if self.rawbytes.len() < 2 {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = &self.rawbytes[0..2];
        self.rawbytes = &self.rawbytes[2..];
        u16::from_be_bytes(res.try_into().unwrap())
    }

    fn parse_u4(&mut self) -> u32 {
        if self.rawbytes.len() < 4 {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = &self.rawbytes[0..4];
        self.rawbytes = &self.rawbytes[4..];
        u32::from_be_bytes(res.try_into().unwrap())
    }

    fn parse_n(&mut self, n: usize) -> &[u8] {
        if self.rawbytes.len() < n {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = &self.rawbytes[0..n];
        self.rawbytes = &self.rawbytes[n..];
        res
    }
}

fn read_file2bytes(path: &str) -> Vec<u8> {
    let mut file = File::open(path)
        .map_err(|err| {
            eprintln!("[ERROR]: Couldn't open file: {err}");
        })
        .unwrap();

    let mut buf: Vec<u8> = vec![];
    File::read_to_end(&mut file, &mut buf)
        .map_err(|err| {
            eprintln!("[ERROR]: Couldn't read file content {err}");
        })
        .unwrap();

    buf
}
