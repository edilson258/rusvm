use std::process::exit;

#[derive(Debug)]
pub struct ByteStream {
    pub xs: Vec<u8>,
}

impl ByteStream {
    pub fn parse_u1(&mut self) -> u8 {
        self.check_bound(1);
        self.xs.remove(0)
    }

    pub fn parse_u2(&mut self) -> u16 {
        self.check_bound(2);
        let res = self.xs[0..2].to_vec();
        self.xs = self.xs[2..].to_vec();
        u16::from_be_bytes(res.try_into().unwrap())
    }

    pub fn parse_u4(&mut self) -> u32 {
        self.check_bound(4);
        let res = self.xs[0..4].to_vec();
        self.xs = self.xs[4..].to_vec();
        u32::from_be_bytes(res.try_into().unwrap())
    }

    pub fn parse_n(&mut self, n: usize) -> Vec<u8> {
        self.check_bound(n);
        let res = self.xs[0..n].to_vec();
        self.xs = self.xs[n..].to_vec();
        res
    }

    fn check_bound(&self, n: usize) {
        if self.xs.len() < n {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
    }
}
