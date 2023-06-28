use classfile::*;
use gc::*;
use std::collections::HashMap;
use std::mem;

#[derive(Debug)]
enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    Float(f32),
    Double(f64),
    Bool(bool),
    //ReturnAddress(?????)
}

// TODO: fix gc_derive to work for enums
unsafe impl Trace for Value {
    fn trace(&self, m: &mut Marker) {}
}

#[derive(Debug, Trace)]
struct Frame {
    /// Long/Double takes **two slots** in this
    locals: Vec<Value>,
    /// Long/Double takes **one slot** in this
    operand_stack: Vec<Value>,
}

impl Frame {
    fn from_method(class: &str, meth: &MethodInfo) -> Frame {
        let Some(code) = &meth.code else {
            panic!("{class}.{}() doesn't have code!", meth.name);
        };

        Frame {
            locals: Vec::with_capacity(code.max_locals.into()),
            operand_stack: Vec::with_capacity(code.max_stack.into()),
        }
    }
}

#[derive(Debug)]
pub struct Vm {
    classes: HashMap<&'static str, ClassFile>,
    heap: Heap,
    frame_stack: Vec<Frame>,
}

impl Vm {
    pub fn from_init_class(main_class: ClassFile) -> Self {
        let mut it = Self {
            heap: Heap::new(),
            frame_stack: Vec::new(),
            classes: HashMap::new(),
        };

        it.classes.insert(
            unsafe { mem::transmute::<&'_ str, &'static str>(main_class.this_class()) },
            main_class,
        );

        let main_class = it.classes.iter().next().map(|(_, v)| v).unwrap();

        let main_method = main_class
            .methods()
            .iter()
            .find(|x| x.name == "<init>")
            .expect("main_class doesn't have <init>");

        it.frame_stack
            .push(Frame::from_method(main_class.this_class(), main_method));

        it
    }
}
