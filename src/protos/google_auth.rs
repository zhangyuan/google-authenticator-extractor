// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google_auth.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct MigrationPayload {
    // message fields
    pub otp_parameters: ::protobuf::RepeatedField<MigrationPayload_OtpParameters>,
    pub version: i32,
    pub batch_size: i32,
    pub batch_index: i32,
    pub batch_id: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MigrationPayload {
    fn default() -> &'a MigrationPayload {
        <MigrationPayload as ::protobuf::Message>::default_instance()
    }
}

impl MigrationPayload {
    pub fn new() -> MigrationPayload {
        ::std::default::Default::default()
    }

    // repeated .MigrationPayload.OtpParameters otp_parameters = 1;


    pub fn get_otp_parameters(&self) -> &[MigrationPayload_OtpParameters] {
        &self.otp_parameters
    }
    pub fn clear_otp_parameters(&mut self) {
        self.otp_parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_otp_parameters(&mut self, v: ::protobuf::RepeatedField<MigrationPayload_OtpParameters>) {
        self.otp_parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_otp_parameters(&mut self) -> &mut ::protobuf::RepeatedField<MigrationPayload_OtpParameters> {
        &mut self.otp_parameters
    }

    // Take field
    pub fn take_otp_parameters(&mut self) -> ::protobuf::RepeatedField<MigrationPayload_OtpParameters> {
        ::std::mem::replace(&mut self.otp_parameters, ::protobuf::RepeatedField::new())
    }

    // int32 version = 2;


    pub fn get_version(&self) -> i32 {
        self.version
    }
    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = v;
    }

    // int32 batch_size = 3;


    pub fn get_batch_size(&self) -> i32 {
        self.batch_size
    }
    pub fn clear_batch_size(&mut self) {
        self.batch_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_batch_size(&mut self, v: i32) {
        self.batch_size = v;
    }

    // int32 batch_index = 4;


    pub fn get_batch_index(&self) -> i32 {
        self.batch_index
    }
    pub fn clear_batch_index(&mut self) {
        self.batch_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_batch_index(&mut self, v: i32) {
        self.batch_index = v;
    }

    // int32 batch_id = 5;


    pub fn get_batch_id(&self) -> i32 {
        self.batch_id
    }
    pub fn clear_batch_id(&mut self) {
        self.batch_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_batch_id(&mut self, v: i32) {
        self.batch_id = v;
    }
}

impl ::protobuf::Message for MigrationPayload {
    fn is_initialized(&self) -> bool {
        for v in &self.otp_parameters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.otp_parameters)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.batch_size = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.batch_index = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.batch_id = tmp;
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
        for value in &self.otp_parameters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.batch_size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.batch_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.batch_index != 0 {
            my_size += ::protobuf::rt::value_size(4, self.batch_index, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.batch_id != 0 {
            my_size += ::protobuf::rt::value_size(5, self.batch_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.otp_parameters {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.version != 0 {
            os.write_int32(2, self.version)?;
        }
        if self.batch_size != 0 {
            os.write_int32(3, self.batch_size)?;
        }
        if self.batch_index != 0 {
            os.write_int32(4, self.batch_index)?;
        }
        if self.batch_id != 0 {
            os.write_int32(5, self.batch_id)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MigrationPayload {
        MigrationPayload::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MigrationPayload_OtpParameters>>(
                "otp_parameters",
                |m: &MigrationPayload| { &m.otp_parameters },
                |m: &mut MigrationPayload| { &mut m.otp_parameters },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "version",
                |m: &MigrationPayload| { &m.version },
                |m: &mut MigrationPayload| { &mut m.version },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "batch_size",
                |m: &MigrationPayload| { &m.batch_size },
                |m: &mut MigrationPayload| { &mut m.batch_size },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "batch_index",
                |m: &MigrationPayload| { &m.batch_index },
                |m: &mut MigrationPayload| { &mut m.batch_index },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "batch_id",
                |m: &MigrationPayload| { &m.batch_id },
                |m: &mut MigrationPayload| { &mut m.batch_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MigrationPayload>(
                "MigrationPayload",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MigrationPayload {
        static instance: ::protobuf::rt::LazyV2<MigrationPayload> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MigrationPayload::new)
    }
}

impl ::protobuf::Clear for MigrationPayload {
    fn clear(&mut self) {
        self.otp_parameters.clear();
        self.version = 0;
        self.batch_size = 0;
        self.batch_index = 0;
        self.batch_id = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MigrationPayload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MigrationPayload {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MigrationPayload_OtpParameters {
    // message fields
    pub secret: ::std::vec::Vec<u8>,
    pub name: ::std::string::String,
    pub issuer: ::std::string::String,
    pub algorithm: MigrationPayload_Algorithm,
    pub digits: i32,
    pub field_type: MigrationPayload_OtpType,
    pub counter: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MigrationPayload_OtpParameters {
    fn default() -> &'a MigrationPayload_OtpParameters {
        <MigrationPayload_OtpParameters as ::protobuf::Message>::default_instance()
    }
}

impl MigrationPayload_OtpParameters {
    pub fn new() -> MigrationPayload_OtpParameters {
        ::std::default::Default::default()
    }

    // bytes secret = 1;


    pub fn get_secret(&self) -> &[u8] {
        &self.secret
    }
    pub fn clear_secret(&mut self) {
        self.secret.clear();
    }

    // Param is passed by value, moved
    pub fn set_secret(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.secret
    }

    // Take field
    pub fn take_secret(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.secret, ::std::vec::Vec::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string issuer = 3;


    pub fn get_issuer(&self) -> &str {
        &self.issuer
    }
    pub fn clear_issuer(&mut self) {
        self.issuer.clear();
    }

    // Param is passed by value, moved
    pub fn set_issuer(&mut self, v: ::std::string::String) {
        self.issuer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_issuer(&mut self) -> &mut ::std::string::String {
        &mut self.issuer
    }

    // Take field
    pub fn take_issuer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.issuer, ::std::string::String::new())
    }

    // .MigrationPayload.Algorithm algorithm = 4;


    pub fn get_algorithm(&self) -> MigrationPayload_Algorithm {
        self.algorithm
    }
    pub fn clear_algorithm(&mut self) {
        self.algorithm = MigrationPayload_Algorithm::ALGO_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_algorithm(&mut self, v: MigrationPayload_Algorithm) {
        self.algorithm = v;
    }

    // int32 digits = 5;


    pub fn get_digits(&self) -> i32 {
        self.digits
    }
    pub fn clear_digits(&mut self) {
        self.digits = 0;
    }

    // Param is passed by value, moved
    pub fn set_digits(&mut self, v: i32) {
        self.digits = v;
    }

    // .MigrationPayload.OtpType type = 6;


    pub fn get_field_type(&self) -> MigrationPayload_OtpType {
        self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = MigrationPayload_OtpType::OTP_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MigrationPayload_OtpType) {
        self.field_type = v;
    }

    // int64 counter = 7;


    pub fn get_counter(&self) -> i64 {
        self.counter
    }
    pub fn clear_counter(&mut self) {
        self.counter = 0;
    }

    // Param is passed by value, moved
    pub fn set_counter(&mut self, v: i64) {
        self.counter = v;
    }
}

impl ::protobuf::Message for MigrationPayload_OtpParameters {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.secret)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.issuer)?;
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.algorithm, 4, &mut self.unknown_fields)?
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.digits = tmp;
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 6, &mut self.unknown_fields)?
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.counter = tmp;
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
        if !self.secret.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.secret);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.issuer.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.issuer);
        }
        if self.algorithm != MigrationPayload_Algorithm::ALGO_INVALID {
            my_size += ::protobuf::rt::enum_size(4, self.algorithm);
        }
        if self.digits != 0 {
            my_size += ::protobuf::rt::value_size(5, self.digits, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.field_type != MigrationPayload_OtpType::OTP_INVALID {
            my_size += ::protobuf::rt::enum_size(6, self.field_type);
        }
        if self.counter != 0 {
            my_size += ::protobuf::rt::value_size(7, self.counter, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.secret.is_empty() {
            os.write_bytes(1, &self.secret)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.issuer.is_empty() {
            os.write_string(3, &self.issuer)?;
        }
        if self.algorithm != MigrationPayload_Algorithm::ALGO_INVALID {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.algorithm))?;
        }
        if self.digits != 0 {
            os.write_int32(5, self.digits)?;
        }
        if self.field_type != MigrationPayload_OtpType::OTP_INVALID {
            os.write_enum(6, ::protobuf::ProtobufEnum::value(&self.field_type))?;
        }
        if self.counter != 0 {
            os.write_int64(7, self.counter)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MigrationPayload_OtpParameters {
        MigrationPayload_OtpParameters::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "secret",
                |m: &MigrationPayload_OtpParameters| { &m.secret },
                |m: &mut MigrationPayload_OtpParameters| { &mut m.secret },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &MigrationPayload_OtpParameters| { &m.name },
                |m: &mut MigrationPayload_OtpParameters| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "issuer",
                |m: &MigrationPayload_OtpParameters| { &m.issuer },
                |m: &mut MigrationPayload_OtpParameters| { &mut m.issuer },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MigrationPayload_Algorithm>>(
                "algorithm",
                |m: &MigrationPayload_OtpParameters| { &m.algorithm },
                |m: &mut MigrationPayload_OtpParameters| { &mut m.algorithm },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "digits",
                |m: &MigrationPayload_OtpParameters| { &m.digits },
                |m: &mut MigrationPayload_OtpParameters| { &mut m.digits },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MigrationPayload_OtpType>>(
                "type",
                |m: &MigrationPayload_OtpParameters| { &m.field_type },
                |m: &mut MigrationPayload_OtpParameters| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "counter",
                |m: &MigrationPayload_OtpParameters| { &m.counter },
                |m: &mut MigrationPayload_OtpParameters| { &mut m.counter },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MigrationPayload_OtpParameters>(
                "MigrationPayload.OtpParameters",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MigrationPayload_OtpParameters {
        static instance: ::protobuf::rt::LazyV2<MigrationPayload_OtpParameters> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MigrationPayload_OtpParameters::new)
    }
}

impl ::protobuf::Clear for MigrationPayload_OtpParameters {
    fn clear(&mut self) {
        self.secret.clear();
        self.name.clear();
        self.issuer.clear();
        self.algorithm = MigrationPayload_Algorithm::ALGO_INVALID;
        self.digits = 0;
        self.field_type = MigrationPayload_OtpType::OTP_INVALID;
        self.counter = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MigrationPayload_OtpParameters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MigrationPayload_OtpParameters {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MigrationPayload_Algorithm {
    ALGO_INVALID = 0,
    ALGO_SHA1 = 1,
}

impl ::protobuf::ProtobufEnum for MigrationPayload_Algorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MigrationPayload_Algorithm> {
        match value {
            0 => ::std::option::Option::Some(MigrationPayload_Algorithm::ALGO_INVALID),
            1 => ::std::option::Option::Some(MigrationPayload_Algorithm::ALGO_SHA1),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MigrationPayload_Algorithm] = &[
            MigrationPayload_Algorithm::ALGO_INVALID,
            MigrationPayload_Algorithm::ALGO_SHA1,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<MigrationPayload_Algorithm>("MigrationPayload.Algorithm", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for MigrationPayload_Algorithm {
}

impl ::std::default::Default for MigrationPayload_Algorithm {
    fn default() -> Self {
        MigrationPayload_Algorithm::ALGO_INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for MigrationPayload_Algorithm {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MigrationPayload_OtpType {
    OTP_INVALID = 0,
    OTP_HOTP = 1,
    OTP_TOTP = 2,
}

impl ::protobuf::ProtobufEnum for MigrationPayload_OtpType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MigrationPayload_OtpType> {
        match value {
            0 => ::std::option::Option::Some(MigrationPayload_OtpType::OTP_INVALID),
            1 => ::std::option::Option::Some(MigrationPayload_OtpType::OTP_HOTP),
            2 => ::std::option::Option::Some(MigrationPayload_OtpType::OTP_TOTP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MigrationPayload_OtpType] = &[
            MigrationPayload_OtpType::OTP_INVALID,
            MigrationPayload_OtpType::OTP_HOTP,
            MigrationPayload_OtpType::OTP_TOTP,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<MigrationPayload_OtpType>("MigrationPayload.OtpType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for MigrationPayload_OtpType {
}

impl ::std::default::Default for MigrationPayload_OtpType {
    fn default() -> Self {
        MigrationPayload_OtpType::OTP_INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for MigrationPayload_OtpType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11google_auth.proto\"\xa7\x04\n\x10MigrationPayload\x12F\n\x0eotp_pa\
    rameters\x18\x01\x20\x03(\x0b2\x1f.MigrationPayload.OtpParametersR\rotpP\
    arameters\x12\x18\n\x07version\x18\x02\x20\x01(\x05R\x07version\x12\x1d\
    \n\nbatch_size\x18\x03\x20\x01(\x05R\tbatchSize\x12\x1f\n\x0bbatch_index\
    \x18\x04\x20\x01(\x05R\nbatchIndex\x12\x19\n\x08batch_id\x18\x05\x20\x01\
    (\x05R\x07batchId\x1a\xef\x01\n\rOtpParameters\x12\x16\n\x06secret\x18\
    \x01\x20\x01(\x0cR\x06secret\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04na\
    me\x12\x16\n\x06issuer\x18\x03\x20\x01(\tR\x06issuer\x129\n\talgorithm\
    \x18\x04\x20\x01(\x0e2\x1b.MigrationPayload.AlgorithmR\talgorithm\x12\
    \x16\n\x06digits\x18\x05\x20\x01(\x05R\x06digits\x12-\n\x04type\x18\x06\
    \x20\x01(\x0e2\x19.MigrationPayload.OtpTypeR\x04type\x12\x18\n\x07counte\
    r\x18\x07\x20\x01(\x03R\x07counter\",\n\tAlgorithm\x12\x10\n\x0cALGO_INV\
    ALID\x10\0\x12\r\n\tALGO_SHA1\x10\x01\"6\n\x07OtpType\x12\x0f\n\x0bOTP_I\
    NVALID\x10\0\x12\x0c\n\x08OTP_HOTP\x10\x01\x12\x0c\n\x08OTP_TOTP\x10\x02\
    b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
