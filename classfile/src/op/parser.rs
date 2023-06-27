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
                    let methodref_index = self.next_be_u16();
                    let r = match self.cp.get(methodref_index).unwrap() {
                        Constant::InterfaceMethodref(x) => self.cp.get_methodref(*x, true),
                        Constant::Methodref(x) => self.cp.get_methodref(*x, false),
                        _ => panic!("expected [interface]methodref"),
                    };

                    Invokespecial(r)
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
                    let idx = self.next_be_u16();
                    New(self.cp.get_class(idx))
                }
                0x59 => Dup,
                0xb5 => {
                    let idx = self.next_be_u16();
                    Putfield(self.cp.get_fieldref(idx))
                }
                0xb4 => {
                    let idx = self.next_be_u16();
                    Getfield(self.cp.get_fieldref(idx))
                }
                0xac => Ireturn,
                0x4b => Astore0,
                0x4c => Astore1,
                0x4d => Astore2,
                0x4e => Astore3,
                0xb6 => {
                    let idx = self.next_be_u16();
                    let r = match self.cp.get(idx).unwrap() {
                        Constant::Methodref(x) => self.cp.get_methodref(*x, false),
                        _ => panic!("expected methodref"),
                    };

                    Invokevirtual(r)
                }
                x => panic!("Unknown opcode 0x{x:X}"),
            };
            ops.push(unsafe { std::mem::transmute::<Op<'_>, Op<'static>>(op) });
        }

        ops
    }

    fn next_u8(&mut self) -> u8 {
        let it = self.raw[0];
        self.raw = &self.raw[1..];
        it
    }

    fn next_be_u16(&mut self) -> u16 {
        let index1: u16 = self.next_u8().into();
        let index2: u16 = self.next_u8().into();
        index1 << 8 | index2
    }
}
