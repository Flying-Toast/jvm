//! This crate contains structs and parsing methods for of .class files.
//!
//! The main type is [`ClassFile`].

pub mod op;

pub use attribute::*;
pub use descriptor::*;

mod attribute;
mod constant_pool;
mod descriptor;
mod parser;

use constant_pool::ConstantPool;
use std::mem;

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.1>
#[derive(Debug)]
pub struct ClassFile {
    // UNSOUND: All the 'static lifetimes in here aren't actually static.
    // Rather, they should be tied to the lifetime of the owned ConstantPool.
    // tldr: this is really cursed
    major_version: u16,
    minor_version: u16,
    _constant_pool: ConstantPool,
    fields: Vec<FieldInfo<'static>>,
    access_flags: AccessFlags,
    methods: Vec<MethodInfo<'static>>,
    attributes: Vec<Attribute<'static>>,
    this_class: &'static str,
    super_class: &'static str,
    interfaces: Vec<&'static str>,
}

impl ClassFile {
    /// Parses a `ClassFile` from the provided bytes of a .class file
    pub fn parse_from_bytes(raw: &[u8]) -> Self {
        parser::Parser::new(raw).run()
    }

    pub fn access_flags(&self) -> AccessFlags {
        self.access_flags
    }

    pub fn fields(&self) -> &[FieldInfo] {
        let ret: &[FieldInfo<'static>] = &self.fields;
        unsafe { mem::transmute(ret) }
    }

    pub fn methods(&self) -> &[MethodInfo] {
        let ret: &[MethodInfo<'static>] = &self.methods;
        unsafe { mem::transmute(ret) }
    }

    pub fn attributes(&self) -> &[Attribute] {
        let ret: &[Attribute<'static>] = &self.attributes;
        unsafe { mem::transmute(ret) }
    }

    pub fn this_class(&self) -> &str {
        unsafe { mem::transmute(self.this_class) }
    }

    /// (major, minor)
    pub fn version(&self) -> (u16, u16) {
        (self.major_version, self.minor_version)
    }

    pub fn super_class(&self) -> &str {
        unsafe { mem::transmute(self.super_class) }
    }

    pub fn interfaces(&self) -> &[&str] {
        let ret: &[&'static str] = &self.interfaces;
        unsafe { mem::transmute(ret) }
    }
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.5>
#[derive(Debug)]
pub struct FieldInfo<'cp> {
    pub access_flags: AccessFlags,
    pub name: &'cp str,
    pub descriptor: FieldDescriptor<'cp>,
    pub attributes: Vec<Attribute<'cp>>,
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.6>
#[derive(Debug)]
pub struct MethodInfo<'cp> {
    pub access_flags: AccessFlags,
    pub name: &'cp str,
    pub descriptor: MethodDescriptor<'cp>,
    pub attributes: Vec<Attribute<'cp>>,
}

#[derive(Debug)]
pub struct FieldRef<'cp> {
    pub class: &'cp str,
    pub name: &'cp str,
    pub descriptor: FieldDescriptor<'cp>,
}

#[derive(Debug)]
pub struct MethodRef<'cp> {
    /// if `false`: this is a class method. If `true`: this is an interface method.
    pub is_interface: bool,
    pub class: &'cp str,
    pub name: &'cp str,
    pub descriptor: MethodDescriptor<'cp>,
}

/// A bitflag to check against `AccessFlags`.
/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.1-200-E.1>
#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum Access {
    Public = 0x001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.1-200-E.1>
#[derive(Copy, Clone)]
pub struct AccessFlags(u16);

impl AccessFlags {
    pub fn has(self, flag: Access) -> bool {
        self.0 & (flag as u16) != 0
    }

    fn new(bits: u16) -> Self {
        Self(bits)
    }
}

impl std::fmt::Debug for AccessFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Access::*;

        f.write_str("{ ")?;

        for (string, access) in [
            ("Public", Public),
            ("Final", Final),
            ("Super", Super),
            ("Interface", Interface),
            ("Abstract", Abstract),
            ("Synthetic", Synthetic),
            ("Annotation", Annotation),
            ("Enum", Enum),
            ("Module", Module),
        ] {
            if self.has(access) {
                f.write_fmt(format_args!("{string} "))?;
            }
        }

        f.write_str("}")
    }
}
