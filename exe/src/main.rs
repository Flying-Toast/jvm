use classfile::ClassFile;
use std::io::Read;

fn main() {
    let mut data = Vec::new();
    std::fs::File::open(
        std::env::args()
            .skip(1)
            .next()
            .unwrap_or("Foo.class".into()),
    )
    .unwrap()
    .read_to_end(&mut data)
    .unwrap();

    let fooclass = ClassFile::parse_from_bytes(&data);
    println!("{fooclass:#?}");
}
