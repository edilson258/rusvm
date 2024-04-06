use std::process::exit;

use crate::ByteStream;

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_METHODREF: u8 = 10;
// const CONSTANT_INTERFACEMETHODREF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
// const CONSTANT_INTEGER: u8 = 3;
//const CONSTANT_FLOAT: u8 = 4;
// const CONSTANT_LONG: u8 = 5;
// const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAMEANDTYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
// const CONSTANT_METHODHANDLE: u8 = 15;
// const CONSTANT_METHODTYPE: u8 = 16;
// const CONSTANT_INVOKEDYNAMIC: u8 = 18;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct ConstantPoolInfo {
    tag: u8,
    tag_name: String,
    entries: Vec<(String, u16)>,
    bytes: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct ConstantPool {
    pub count: usize,
    pub info: Vec<ConstantPoolInfo>,
}

impl ConstantPool {
    pub fn query(&self, index: usize) -> String {
        let index: usize = index - 1;

        if index > self.count {
            eprintln!(
                "[ERROR]:{}:{}: Constant Pool Index out of bound",
                file!(),
                line!()
            );
            exit(1);
        }

        if self.info[index].tag != CONSTANT_UTF8 {
            return self.query(self.info[index].entries[0].1 as usize);
        }

        self.info[index].bytes.clone().unwrap()
    }

    pub fn parse(&mut self, bytes: &mut ByteStream) -> ConstantPool {
        let count = bytes.parse_u2();

        let mut constant_pool = ConstantPool {
            count: count as usize,
            info: vec![],
        };

        for _ in 0..(count - 1) {
            let tag = bytes.parse_u1();
            match tag {
                CONSTANT_CLASS => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_CLASS".to_string(),
                    entries: vec![("name_index".to_string(), bytes.parse_u2())],
                    bytes: None,
                }),
                CONSTANT_METHODREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_METHODREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), bytes.parse_u2()),
                        ("name_and_type_index".to_string(), bytes.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_NAMEANDTYPE => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_NAMEANDTYPE".to_string(),
                    entries: vec![
                        ("name_index".to_string(), bytes.parse_u2()),
                        ("descriptor_index".to_string(), bytes.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_UTF8 => {
                    let length = bytes.parse_u2();
                    constant_pool.info.push(ConstantPoolInfo {
                        tag,
                        tag_name: "CONSTANT_UTF8".to_string(),
                        entries: vec![("length".to_string(), length)],
                        bytes: Some(
                            String::from_utf8(bytes.parse_n(length as usize)).unwrap(),
                        ),
                    })
                }
                CONSTANT_FIELDREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_FIELDREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), bytes.parse_u2()),
                        ("name_and_type_index".to_string(), bytes.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_STRING => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_STRING".to_string(),
                    entries: vec![("string_index".to_string(), bytes.parse_u2())],
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
}
