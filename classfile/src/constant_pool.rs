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
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.4-140>
#[derive(Debug)]
pub(crate) enum Constant {
    Class(ClassConstant),
    Methodref(MethodrefConstant),
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
#[derive(Debug)]
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
