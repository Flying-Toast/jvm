mod parser;

use crate::{ConstantPool, FieldRef, MethodRef};

/// A VM instruction, with the opcode and all its operands.
/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5>
#[derive(Debug)]
pub enum Op<'cp> {
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Invokespecial(MethodRef<'cp>),
    Return,
    Ireturn,
    Bipush(i8),
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Iconstm1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Iadd,
    New(&'cp str),
    Dup,
    Putfield(FieldRef<'cp>),
    Getfield(FieldRef<'cp>),
    Invokevirtual(MethodRef<'cp>),
}

/// Parses a series of instructions out of the code given by `raw`.
pub(crate) fn parse_ops_from_code_bytes(raw: &[u8], cp: &ConstantPool) -> Vec<Op<'static>> {
    parser::Parser::new(raw, cp).run()
}
