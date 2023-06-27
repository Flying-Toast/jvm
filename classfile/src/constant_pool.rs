use crate::descriptor;
use crate::{FieldRef, MethodRef};

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4>
#[derive(Debug)]
pub(crate) struct ConstantPool {
    storage: Vec<Constant>,
}

impl ConstantPool {
    pub fn new(storage: Vec<Constant>) -> Self {
        Self { storage }
    }

    /// Returns the constant at the (one-based) index `idx`, or `None` if out of bounds.
    /// The indices are one-based: `get(0)` returns None.
    pub fn get(&self, idx: u16) -> Option<&Constant> {
        if idx == 0 {
            None
        } else {
            self.storage.get((idx - 1) as usize)
        }
    }

    pub(crate) fn get_class(&self, index: u16) -> &str {
        let Constant::Class(ClassConstant { name_index }) = self.get(index).unwrap() else {
            panic!("expected classconst");
        };

        self.get_utf8(*name_index)
    }

    pub(crate) fn get_utf8(&self, index: u16) -> &str {
        let Constant::Utf8(s) = self.get(index).unwrap() else {
            panic!("expected utf8");
        };
        s
    }

    pub(crate) fn get_methodref(&self, cnst: MethodrefConstant, is_interface: bool) -> MethodRef {
        let MethodrefConstant {
            class_index,
            name_and_type_index,
        } = cnst;

        let classname = self.get_class(class_index);
        let Constant::NameAndType(NameAndTypeConstant { name_index, descriptor_index }) = self.get(name_and_type_index).unwrap() else {
            panic!("expected utf8");
        };
        let descriptor_string = self.get_utf8(*descriptor_index);

        MethodRef {
            is_interface,
            class: classname,
            name: self.get_utf8(*name_index),
            descriptor: descriptor::parse_method_descriptor(descriptor_string),
        }
    }

    pub(crate) fn get_fieldref(&self, index: u16) -> FieldRef {
        let Constant::Fieldref(FieldrefConstant { class_index, name_and_type_index }) = self.get(index).unwrap() else {
            panic!("expected fieldref");
        };
        let classname = self.get_class(*class_index);
        let Constant::NameAndType(NameAndTypeConstant { name_index, descriptor_index }) = self.get(*name_and_type_index).unwrap() else {
            panic!("expected utf8");
        };
        let field_name = self.get_utf8(*name_index);
        let descriptor = self.get_utf8(*descriptor_index);

        FieldRef {
            class: classname,
            name: field_name,
            descriptor: descriptor::parse_field_descriptor(descriptor),
        }
    }
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4-140>
#[derive(Debug)]
pub(crate) enum Constant {
    Class(ClassConstant),
    Methodref(MethodrefConstant),
    InterfaceMethodref(MethodrefConstant),
    NameAndType(NameAndTypeConstant),
    Fieldref(FieldrefConstant),
    /// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4.7>
    Utf8(String),
    ConstantValue(ConstantValueKind),
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7.2>
#[derive(Debug)]
pub(crate) enum ConstantValueKind {
    /// int, short, char, byte, boolean
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    /// Holds the index of the Utf8Constant
    String(u16),
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4.1>
#[derive(Debug)]
pub(crate) struct ClassConstant {
    pub name_index: u16,
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4.6>
#[derive(Debug)]
pub(crate) struct NameAndTypeConstant {
    pub name_index: u16,
    pub descriptor_index: u16,
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4.2>
#[derive(Debug, Copy, Clone)]
pub(crate) struct MethodrefConstant {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4.2>
#[derive(Debug)]
pub(crate) struct FieldrefConstant {
    pub class_index: u16,
    pub name_and_type_index: u16,
}
