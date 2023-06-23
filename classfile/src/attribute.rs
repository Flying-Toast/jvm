/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7>
#[derive(Debug)]
pub enum Attribute<'cp> {
    ConstantValue(ConstantValueAttribute<'cp>),
    Code(CodeAttribute<'cp>),
    LineNumberTable(Vec<LineNumberTableEntry>),
    /// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7.10>
    SourceFile(&'cp str),
}

/// The spec defines this struct inline on the "Code" attribute: <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7.3>
#[derive(Debug)]
pub struct ExceptionHandlerDescriptor {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7.3>
#[derive(Debug)]
pub struct CodeAttribute<'cp> {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<crate::op::Op<'cp>>,
    pub exception_table: Vec<ExceptionHandlerDescriptor>,
    pub attributes: Vec<Attribute<'cp>>,
}

/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7.2>
#[derive(Debug)]
pub enum ConstantValueAttribute<'cp> {
    /// int, short, char, byte, boolean
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(&'cp str),
}

/// Spec defines this inline on "LineNumberTable": <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.7.12>
#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}
