// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `aas.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct Msg {
    // message fields
    pub msg_type: Msg_MsgType,
    pub msg_bytes: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Msg {
    fn default() -> &'a Msg {
        <Msg as ::protobuf::Message>::default_instance()
    }
}

impl Msg {
    pub fn new() -> Msg {
        ::std::default::Default::default()
    }

    // .aas.Msg.MsgType msg_type = 1;


    pub fn get_msg_type(&self) -> Msg_MsgType {
        self.msg_type
    }
    pub fn clear_msg_type(&mut self) {
        self.msg_type = Msg_MsgType::MSG_UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: Msg_MsgType) {
        self.msg_type = v;
    }

    // bytes msg_bytes = 2;


    pub fn get_msg_bytes(&self) -> &[u8] {
        &self.msg_bytes
    }
    pub fn clear_msg_bytes(&mut self) {
        self.msg_bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg_bytes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.msg_bytes
    }

    // Take field
    pub fn take_msg_bytes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.msg_bytes, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Msg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.msg_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.msg_bytes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.msg_type != Msg_MsgType::MSG_UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.msg_type);
        }
        if !self.msg_bytes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.msg_bytes);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.msg_type != Msg_MsgType::MSG_UNKNOWN {
            os.write_enum(1, self.msg_type.value())?;
        }
        if !self.msg_bytes.is_empty() {
            os.write_bytes(2, &self.msg_bytes)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Msg {
        Msg::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Msg_MsgType>>(
                    "msg_type",
                    |m: &Msg| { &m.msg_type },
                    |m: &mut Msg| { &mut m.msg_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msg_bytes",
                    |m: &Msg| { &m.msg_bytes },
                    |m: &mut Msg| { &mut m.msg_bytes },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Msg>(
                    "Msg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Msg {
        static mut instance: ::protobuf::lazy::Lazy<Msg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Msg,
        };
        unsafe {
            instance.get(Msg::new)
        }
    }
}

impl ::protobuf::Clear for Msg {
    fn clear(&mut self) {
        self.msg_type = Msg_MsgType::MSG_UNKNOWN;
        self.msg_bytes.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Msg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Msg_MsgType {
    MSG_UNKNOWN = 0,
    SGX_RA_MSG0 = 1,
    SGX_RA_MSG1 = 2,
    SGX_RA_MSG2 = 3,
    SGX_RA_MSG3 = 4,
    SGX_RA_MSG0_REPLY = 5,
    SGX_RA_MSG3_REPLY = 6,
    AAS_RA_REG_REQUEST = 7,
    AAS_RA_REG_REPORT = 8,
}

impl ::protobuf::ProtobufEnum for Msg_MsgType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Msg_MsgType> {
        match value {
            0 => ::std::option::Option::Some(Msg_MsgType::MSG_UNKNOWN),
            1 => ::std::option::Option::Some(Msg_MsgType::SGX_RA_MSG0),
            2 => ::std::option::Option::Some(Msg_MsgType::SGX_RA_MSG1),
            3 => ::std::option::Option::Some(Msg_MsgType::SGX_RA_MSG2),
            4 => ::std::option::Option::Some(Msg_MsgType::SGX_RA_MSG3),
            5 => ::std::option::Option::Some(Msg_MsgType::SGX_RA_MSG0_REPLY),
            6 => ::std::option::Option::Some(Msg_MsgType::SGX_RA_MSG3_REPLY),
            7 => ::std::option::Option::Some(Msg_MsgType::AAS_RA_REG_REQUEST),
            8 => ::std::option::Option::Some(Msg_MsgType::AAS_RA_REG_REPORT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Msg_MsgType] = &[
            Msg_MsgType::MSG_UNKNOWN,
            Msg_MsgType::SGX_RA_MSG0,
            Msg_MsgType::SGX_RA_MSG1,
            Msg_MsgType::SGX_RA_MSG2,
            Msg_MsgType::SGX_RA_MSG3,
            Msg_MsgType::SGX_RA_MSG0_REPLY,
            Msg_MsgType::SGX_RA_MSG3_REPLY,
            Msg_MsgType::AAS_RA_REG_REQUEST,
            Msg_MsgType::AAS_RA_REG_REPORT,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Msg_MsgType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Msg_MsgType {
}

impl ::std::default::Default for Msg_MsgType {
    fn default() -> Self {
        Msg_MsgType::MSG_UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for Msg_MsgType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\taas.proto\x12\x03aas\"\x8d\x02\n\x03Msg\x12+\n\x08msg_type\x18\x01\
    \x20\x01(\x0e2\x10.aas.Msg.MsgTypeR\x07msgType\x12\x1b\n\tmsg_bytes\x18\
    \x02\x20\x01(\x0cR\x08msgBytes\"\xbb\x01\n\x07MsgType\x12\x0f\n\x0bMSG_U\
    NKNOWN\x10\0\x12\x0f\n\x0bSGX_RA_MSG0\x10\x01\x12\x0f\n\x0bSGX_RA_MSG1\
    \x10\x02\x12\x0f\n\x0bSGX_RA_MSG2\x10\x03\x12\x0f\n\x0bSGX_RA_MSG3\x10\
    \x04\x12\x15\n\x11SGX_RA_MSG0_REPLY\x10\x05\x12\x15\n\x11SGX_RA_MSG3_REP\
    LY\x10\x06\x12\x16\n\x12AAS_RA_REG_REQUEST\x10\x07\x12\x15\n\x11AAS_RA_R\
    EG_REPORT\x10\x0826\n\tAasServer\x12)\n\rremote_attest\x12\x08.aas.Msg\
    \x1a\x08.aas.Msg\"\0(\x010\x01b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}