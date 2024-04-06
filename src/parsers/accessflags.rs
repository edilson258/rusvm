use std::ops::BitAnd;

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

pub fn parse_class_access_flags(mask: u16) -> Vec<String> {
    parse_access_flags(mask, &CLASS_ACCESS_FLAGS)
}

pub fn parse_method_access_flags(mask: u16) -> Vec<String> {
    parse_access_flags(mask, &METHOD_ACCESS_FLAGS)
}

fn parse_access_flags(mask: u16, access_flags: &[(&str, u16)]) -> Vec<String> {
    let mut flags: Vec<String> = vec![];
    for (name, value) in access_flags {
        if mask.bitand(value) != 0 {
            flags.push(name.to_string());
        }
    }
    flags
}
