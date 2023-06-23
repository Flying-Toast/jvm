/// Non-recursive part of a `FieldDescriptor`.
///
/// It is a union of "BaseType" and "ObjectType"
/// as defined in <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.3.2>.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BasicFieldType<'cp> {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    ClassInstance(&'cp str),
    Short,
    Boolean,
}

/// A field descriptor.
/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.3.2>
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FieldDescriptor<'cp> {
    /// BaseType | ObjectType
    Basic(BasicFieldType<'cp>),
    /// ArrayType
    Arr(
        /// Dimension
        std::num::NonZeroU8,
        BasicFieldType<'cp>,
    ),
}

/// Parses a `FieldDescriptor` out of the given field descriptor in `string`.
/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.3.2>
pub(crate) fn parse_field_descriptor(string: &str) -> FieldDescriptor {
    parse_field_descriptor_rest(string).0
}

fn parse_field_descriptor_rest(string: &str) -> (FieldDescriptor, &str) {
    let mut chars = string.chars();

    use BasicFieldType::*;
    use FieldDescriptor::*;
    let desc = match chars.next().unwrap() {
        'B' => Basic(Byte),
        'C' => Basic(Char),
        'D' => Basic(Double),
        'F' => Basic(Float),
        'I' => Basic(Int),
        'J' => Basic(Long),
        'L' => {
            let before = chars.as_str();
            loop {
                match chars.next() {
                    None => panic!("Reached end of string while looking for ';'"),
                    Some(';') => break,
                    Some(_) => {}
                }
            }
            let len_diff = before.len() - chars.as_str().len();

            Basic(ClassInstance(&before[..len_diff - 1]))
        }
        'S' => Basic(Short),
        'Z' => Basic(Boolean),
        '[' => {
            let mut dim = 1;
            while let Some('[') = chars.as_str().chars().next() {
                chars.next();
                dim += 1;
            }

            let (Basic(descriptor), rest) = parse_field_descriptor_rest(chars.as_str()) else {
                // We parsed out the '[' characters already, so the next parsed descriptor
                // wont be an `Arr`.
                unreachable!();
            };
            chars = rest.chars();
            Arr(std::num::NonZeroU8::new(dim).unwrap(), descriptor)
        }
        x => panic!("Unknown field spec {x}"),
    };

    (desc, chars.as_str())
}

/// See <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.3.3>.
#[derive(Debug, Eq, PartialEq)]
pub enum ReturnDescriptor<'cp> {
    Void,
    NonVoid(FieldDescriptor<'cp>),
}

/// A method descriptor.
/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.3.3>
#[derive(Debug, Eq, PartialEq)]
pub struct MethodDescriptor<'cp> {
    pub parameters: Vec<FieldDescriptor<'cp>>,
    pub return_descriptor: ReturnDescriptor<'cp>,
}

/// Parses a `MethodDescriptor` from the specified string.
/// <https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html#jvms-4.3.3>
pub(crate) fn parse_method_descriptor(string: &str) -> MethodDescriptor {
    let mut chars = string.chars();
    assert!(chars.next().unwrap() == '(');

    let mut parameters = Vec::new();
    loop {
        if chars.as_str().chars().next().unwrap() == ')' {
            chars.next();
            break;
        } else {
            let (desc, rest) = parse_field_descriptor_rest(chars.as_str());
            chars = rest.chars();
            parameters.push(desc);
        }
    }
    let return_descriptor = if chars.as_str().chars().next().unwrap() == 'V' {
        ReturnDescriptor::Void
    } else {
        ReturnDescriptor::NonVoid(parse_field_descriptor(chars.as_str()))
    };

    MethodDescriptor {
        parameters,
        return_descriptor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BasicFieldType::*;
    use FieldDescriptor::*;

    #[test]
    fn test_method_descriptor_parsing() {
        assert_eq!(
            MethodDescriptor {
                parameters: vec![
                    Basic(Int),
                    Basic(Double),
                    Basic(ClassInstance("java/lang/Thread"))
                ],
                return_descriptor: ReturnDescriptor::NonVoid(FieldDescriptor::Basic(
                    ClassInstance("java/lang/Object")
                ))
            },
            parse_method_descriptor("(IDLjava/lang/Thread;)Ljava/lang/Object;")
        );

        assert_eq!(
            MethodDescriptor {
                parameters: Vec::new(),
                return_descriptor: ReturnDescriptor::Void
            },
            parse_method_descriptor("()V")
        );

        assert_eq!(
            MethodDescriptor {
                parameters: vec![Basic(Float)],
                return_descriptor: ReturnDescriptor::Void
            },
            parse_method_descriptor("(F)V")
        );
    }

    #[test]
    fn test_field_descriptor_parsing() {
        fn nz(n: u8) -> std::num::NonZeroU8 {
            std::num::NonZeroU8::new(n).unwrap()
        }

        assert_eq!((Basic(Byte), ""), parse_field_descriptor_rest("B"));

        assert_eq!(
            (Basic(ClassInstance("thing/other/FooBar")), ""),
            parse_field_descriptor_rest("Lthing/other/FooBar;")
        );

        assert_eq!((Arr(nz(1), Short), "Z"), parse_field_descriptor_rest("[SZ"));

        assert_eq!(
            (Arr(nz(3), ClassInstance("Something")), "[BSZLHello;"),
            parse_field_descriptor_rest("[[[LSomething;[BSZLHello;")
        );
    }
}
