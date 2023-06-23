use crate::constant_pool::*;
use crate::*;
use std::mem;

/// Parser state for parsing .class files.
#[derive(Debug)]
pub struct Parser<'a> {
    raw: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(raw: &'a [u8]) -> Self {
        Self { raw }
    }

    /// Run the parser on the bytes it was constructed with (see `Self::new`).
    pub fn run(mut self) -> ClassFile {
        self.raw = self
            .raw
            .strip_prefix(&u32::to_be_bytes(0xCAFEBABEu32))
            .expect("bad magic");

        let minor_version = self.next_u16();
        let major_version = self.next_u16();

        let cp_cnt = self.next_u16();
        let constant_pool = self.parse_sized_table(cp_cnt - 1, Self::parse_constant);
        let constant_pool = ConstantPool::new(constant_pool);

        let access_flags = self.next_u16();
        let this_class = self.next_u16();
        let Constant::Class(this_class) = constant_pool.get(this_class).unwrap() else {
            panic!("this_class not a classinfo");
        };
        let Constant::Utf8(this_class) = constant_pool.get(this_class.name_index).unwrap() else {
            panic!();
        };
        let super_class = self.next_u16();
        let Constant::Class(super_class) = constant_pool.get(super_class).unwrap() else {
            panic!("super_class not a classinfo");
        };
        let Constant::Utf8(super_class) = constant_pool.get(super_class.name_index).unwrap() else {
            panic!();
        };

        let interfaces_count = self.next_u16();
        let interfaces = self
            .parse_sized_table(interfaces_count, Self::next_u16)
            .into_iter()
            .map(|x| match constant_pool.get(x) {
                Some(Constant::Utf8(s)) => unsafe { mem::transmute::<&'_ str, &'static str>(s) },
                _ => panic!(),
            })
            .collect();

        let fields_count = self.next_u16();
        let fields = self.parse_sized_table(fields_count, |p| p.parse_field_info(&constant_pool));

        let methods_count = self.next_u16();
        let methods =
            self.parse_sized_table(methods_count, |p| p.parse_method_info(&constant_pool));

        let attributes_count = self.next_u16();
        let attributes =
            self.parse_sized_table(attributes_count, |p| p.parse_attribute(&constant_pool));

        unsafe {
            ClassFile {
                major_version,
                minor_version,
                access_flags: super::AccessFlags::new(access_flags),
                this_class: mem::transmute::<&'_ str, &'static str>(this_class),
                super_class: mem::transmute::<&'_ str, &'static str>(super_class),
                _constant_pool: constant_pool,
                interfaces,
                fields,
                methods,
                attributes,
            }
        }
    }

    fn next_u16(&mut self) -> u16 {
        type Ret = u16;
        let (bytes, raw) = self.raw.split_at(std::mem::size_of::<Ret>());
        self.raw = raw;
        // SAFETY: split_at returns a pointer to size_of<Ret> bytes
        let bytes = unsafe { *(bytes.as_ptr() as *const [u8; std::mem::size_of::<Ret>()]) };
        Ret::from_be_bytes(bytes)
    }

    fn next_u32(&mut self) -> u32 {
        type Ret = u32;
        let (bytes, raw) = self.raw.split_at(std::mem::size_of::<Ret>());
        self.raw = raw;
        // SAFETY: split_at returns a pointer to size_of<Ret> bytes
        let bytes = unsafe { *(bytes.as_ptr() as *const [u8; std::mem::size_of::<Ret>()]) };
        Ret::from_be_bytes(bytes)
    }

    fn parse_sized_table<Len, F, T>(&mut self, len: Len, parse_fn: F) -> Vec<T>
    where
        Len: Copy + Into<usize>,
        F: Fn(&mut Self) -> T,
    {
        let mut v = Vec::with_capacity(len.into());
        for _ in 0..len.into() {
            v.push(parse_fn(self));
        }
        v
    }

    fn parse_constant(&mut self) -> Constant {
        let tag = self.raw[0];
        self.raw = &self.raw[1..];

        use Constant::*;

        match tag {
            7 => Class(self.parse_class_constant()),
            9 => Fieldref(self.parse_fieldref_constant()),
            10 => Methodref(self.parse_methodref_constant()),
            12 => NameAndType(self.parse_name_and_type_constant()),
            1 => Utf8(self.parse_utf8_constant()),
            3 => {
                let (bytes, raw) = self.raw.split_at(4);
                self.raw = raw;
                ConstantValue(ConstantValueKind::Integer(i32::from_be_bytes(
                    bytes.try_into().unwrap(),
                )))
            }
            4 => {
                let (bytes, raw) = self.raw.split_at(4);
                self.raw = raw;
                ConstantValue(ConstantValueKind::Float(f32::from_be_bytes(
                    bytes.try_into().unwrap(),
                )))
            }
            5 => {
                let (bytes, raw) = self.raw.split_at(8);
                self.raw = raw;
                ConstantValue(ConstantValueKind::Long(i64::from_be_bytes(
                    bytes.try_into().unwrap(),
                )))
            }
            6 => {
                let (bytes, raw) = self.raw.split_at(8);
                self.raw = raw;
                ConstantValue(ConstantValueKind::Double(f64::from_be_bytes(
                    bytes.try_into().unwrap(),
                )))
            }
            8 => ConstantValue(ConstantValueKind::String(self.next_u16())),
            x => panic!("unknown constant tag {x}"),
        }
    }

    fn parse_class_constant(&mut self) -> ClassConstant {
        let name_index = self.next_u16();

        ClassConstant { name_index }
    }

    fn parse_methodref_constant(&mut self) -> MethodrefConstant {
        let class_index = self.next_u16();
        let name_and_type_index = self.next_u16();

        MethodrefConstant {
            class_index,
            name_and_type_index,
        }
    }

    fn parse_fieldref_constant(&mut self) -> FieldrefConstant {
        let class_index = self.next_u16();
        let name_and_type_index = self.next_u16();

        FieldrefConstant {
            class_index,
            name_and_type_index,
        }
    }

    fn parse_name_and_type_constant(&mut self) -> NameAndTypeConstant {
        let name_index = self.next_u16();
        let descriptor_index = self.next_u16();

        NameAndTypeConstant {
            name_index,
            descriptor_index,
        }
    }

    fn parse_utf8_constant(&mut self) -> String {
        let len = self.next_u16();
        let (bytes, raw) = self.raw.split_at(len as usize);
        self.raw = raw;

        String::from_utf8(Vec::from(bytes)).unwrap()
    }

    fn parse_field_info(&mut self, cp: &ConstantPool) -> FieldInfo<'static> {
        let access_flags = self.next_u16();

        let name_index = self.next_u16();
        let Constant::Utf8(name_string) = cp.get(name_index).unwrap() else {
            panic!("Invalid class file: expected name_index to point to a string");
        };

        let descriptor_index = self.next_u16();
        let Constant::Utf8(descriptor_string) = cp.get(descriptor_index).unwrap() else {
            panic!("Invalid class file: expected descriptor_index to point to a string");
        };

        let attribute_count = self.next_u16();
        let attributes = self.parse_sized_table(attribute_count, |p| p.parse_attribute(cp));

        FieldInfo {
            access_flags: AccessFlags::new(access_flags),
            name: unsafe { mem::transmute::<&'_ str, &'static str>(name_string) },
            descriptor: unsafe {
                mem::transmute::<FieldDescriptor<'_>, FieldDescriptor<'static>>(
                    crate::descriptor::parse_field_descriptor(descriptor_string),
                )
            },
            attributes,
        }
    }

    fn parse_method_info(&mut self, cp: &ConstantPool) -> MethodInfo<'static> {
        let access_flags = self.next_u16();
        let name_index = self.next_u16();
        let Constant::Utf8(name_string) = cp.get(name_index).unwrap() else {
            panic!("name wasn't a utf8");
        };
        let descriptor_index = self.next_u16();
        let Constant::Utf8(descriptor_string) = cp.get(descriptor_index).unwrap() else {
            panic!("descriptor wasn't a utf8");
        };
        let descriptor = crate::descriptor::parse_method_descriptor(descriptor_string);

        let attributes_count = self.next_u16();
        let attributes = self.parse_sized_table(attributes_count, |p| p.parse_attribute(cp));

        unsafe {
            MethodInfo {
                access_flags: AccessFlags::new(access_flags),
                name: mem::transmute::<&'_ str, &'static str>(name_string),
                descriptor: mem::transmute::<MethodDescriptor<'_>, MethodDescriptor<'static>>(
                    descriptor,
                ),
                attributes,
            }
        }
    }

    fn parse_attribute(&mut self, cp: &ConstantPool) -> Attribute<'static> {
        let attribute_name_index = self.next_u16();
        let attribute_length = self.next_u32() as usize;
        let len_before_attribute = self.raw.len();

        let Constant::Utf8(attribute_name) = cp.get(attribute_name_index).unwrap() else {
            panic!("Attribute name wasn't a utf8constant");
        };

        let attribute = match attribute_name.as_ref() {
            "SourceFile" => {
                let sourcefile_index = self.next_u16();
                let Constant::Utf8(sourcefile_string) = cp.get(sourcefile_index).unwrap() else {
                    panic!("sourcefile wasn't a utf8constant");
                };
                Attribute::SourceFile(sourcefile_string)
            }
            "LineNumberTable" => {
                let len = self.next_u16();
                let entries = self.parse_sized_table(len, |p| {
                    let start_pc = p.next_u16();
                    let line_number = p.next_u16();
                    LineNumberTableEntry {
                        start_pc,
                        line_number,
                    }
                });

                Attribute::LineNumberTable(entries)
            }
            "ConstantValue" => {
                let cv_idx = self.next_u16();
                let Constant::ConstantValue(value) = cp.get(cv_idx).unwrap() else {
                    panic!();
                };

                let att = match value {
                    ConstantValueKind::Integer(x) => ConstantValueAttribute::Integer(*x),
                    ConstantValueKind::Float(x) => ConstantValueAttribute::Float(*x),
                    ConstantValueKind::Long(x) => ConstantValueAttribute::Long(*x),
                    ConstantValueKind::Double(x) => ConstantValueAttribute::Double(*x),
                    ConstantValueKind::String(string_idx) => {
                        let Constant::Utf8(s) = cp.get(*string_idx).unwrap() else {
                            panic!("String ConstantValue was not a string");
                        };

                        ConstantValueAttribute::String(&s)
                    }
                };

                Attribute::ConstantValue(att)
            }
            "Code" => {
                let max_stack = self.next_u16();
                let max_locals = self.next_u16();
                let code_length = self.next_u32();
                let (code, raw) = self.raw.split_at(code_length as usize);
                self.raw = raw;

                let exception_table_length = self.next_u16();
                let exception_table = self.parse_sized_table(exception_table_length, |p| {
                    let start_pc = p.next_u16();
                    let end_pc = p.next_u16();
                    let handler_pc = p.next_u16();
                    let catch_type = p.next_u16();
                    ExceptionHandlerDescriptor {
                        start_pc,
                        end_pc,
                        handler_pc,
                        catch_type,
                    }
                });

                let attributes_count = self.next_u16();
                let attributes =
                    self.parse_sized_table(attributes_count, |p| p.parse_attribute(cp));

                Attribute::Code(CodeAttribute {
                    max_stack,
                    max_locals,
                    code: crate::op::parse_ops_from_code_bytes(code, cp),
                    exception_table,
                    attributes,
                })
            }
            c => panic!("Bad attribute name: {c:?}"),
        };

        debug_assert!(len_before_attribute - self.raw.len() == attribute_length);

        unsafe { mem::transmute::<Attribute<'_>, Attribute<'static>>(attribute) }
    }
}
