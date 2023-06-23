use super::Op;
use crate::constant_pool::*;

/// Parser state for parsing `Op`s
#[derive(Debug)]
pub(crate) struct Parser<'a> {
    raw: &'a [u8],
    cp: &'a ConstantPool,
}

impl<'a> Parser<'a> {
    pub fn new(raw: &'a [u8], cp: &'a ConstantPool) -> Self {
        Self { raw, cp }
    }

    /// Parses `Op`s from the parser's `raw` content (see `Self::new`).
    pub fn run(mut self) -> Vec<Op<'static>> {
        let mut ops = Vec::new();

        while !self.raw.is_empty() {
            use Op::*;

            let opcode = self.next_u8();
            let op = match opcode {
                0x2a => Aload0,
                0x2b => Aload1,
                0x2c => Aload2,
                0x2d => Aload3,
                0x1a => Iload0,
                0x1b => Iload1,
                0x1c => Iload2,
                0x1d => Iload3,
                0xb7 => {
                    let index1: u16 = self.next_u8().into();
                    let index2: u16 = self.next_u8().into();
                    Invokespecial(index1 << 8 | index2)
                }
                0xb1 => Return,
                0x10 => Bipush(self.next_u8()),
                0x3b => Istore0,
                0x3c => Istore1,
                0x3d => Istore2,
                0x3e => Istore3,
                0x2 => Iconstm1,
                0x3 => Iconst0,
                0x4 => Iconst1,
                0x5 => Iconst2,
                0x6 => Iconst3,
                0x7 => Iconst4,
                0x8 => Iconst5,
                0x60 => Iadd,
                0xbb => {
                    let index1: u16 = self.next_u8().into();
                    let index2: u16 = self.next_u8().into();
                    let idx = index1 << 8 | index2;
                    let Constant::Class(ClassConstant { name_index }) = self.cp.get(idx).unwrap() else {
                        panic!("expected classconst");
                    };
                    let Constant::Utf8(s) = self.cp.get(*name_index).unwrap() else {
                        panic!("expected utf8");
                    };

                    New(s)
                }
                0x59 => Dup,
                0xb5 => {
                    let index1: u16 = self.next_u8().into();
                    let index2: u16 = self.next_u8().into();
                    Putfield(index1 << 8 | index2)
                }
                0xb4 => {
                    let index1: u16 = self.next_u8().into();
                    let index2: u16 = self.next_u8().into();
                    Getfield(index1 << 8 | index2)
                }
                0xac => Ireturn,
                0x4b => Astore0,
                0x4c => Astore1,
                0x4d => Astore2,
                0x4e => Astore3,
                0xb6 => {
                    let index1: u16 = self.next_u8().into();
                    let index2: u16 = self.next_u8().into();
                    Invokevirtual(index1 << 8 | index2)
                }
                x => panic!("Unknown opcode 0x{x:X}"),
            };
            ops.push(op);
        }

        unsafe { std::mem::transmute::<Vec<Op<'_>>, Vec<Op<'static>>>(ops) }
    }

    fn next_u8(&mut self) -> u8 {
        let it = self.raw[0];
        self.raw = &self.raw[1..];
        it
    }
}
