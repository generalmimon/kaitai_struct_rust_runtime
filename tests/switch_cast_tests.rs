#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod switch_cast;
use switch_cast::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/switch_opcodes.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = SwitchCast::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(4, test.opcodes.len());

    assert_eq!(0x53, test.opcodes[0].code);
    if let SwitchCast_Opcode_Body::SwitchCast_Strval(s) =  test.opcodes[0].body.as_ref().unwrap() {
        assert_eq!("foobar", s.value);
    } else {
        panic!("expected enum SwitchCast_Opcode_Strval");
    }

    assert_eq!(0x49, test.opcodes[1].code);
    if let SwitchCast_Opcode_Body::SwitchCast_Intval(s) =  test.opcodes[1].body.as_ref().unwrap() {
        assert_eq!(66, s.value);
    } else {
        panic!("expected enum SwitchCast_Opcode_Intval");
    }

    assert_eq!(0x49, test.opcodes[2].code);
    if let SwitchCast_Opcode_Body::SwitchCast_Intval(s) =  test.opcodes[2].body.as_ref().unwrap() {
        assert_eq!(55, s.value);
    } else {
        panic!("expected enum SwitchCast_Opcode_Intval");
    }

    assert_eq!(0x53, test.opcodes[3].code);
    if let SwitchCast_Opcode_Body::SwitchCast_Strval(s) =  test.opcodes[3].body.as_ref().unwrap() {
        assert_eq!("", s.value);
    } else {
        panic!("expected enum SwitchCast_Opcode_Strval");
    }
}