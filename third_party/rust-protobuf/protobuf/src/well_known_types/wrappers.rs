// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct DoubleValue {
    // message fields
    pub value: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoubleValue {}

impl DoubleValue {
    pub fn new() -> DoubleValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoubleValue {
        static mut instance: ::protobuf::lazy::Lazy<DoubleValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoubleValue,
        };
        unsafe {
            instance.get(DoubleValue::new)
        }
    }

    // double value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0.;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &f64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut f64 {
        &mut self.value
    }
}

impl ::protobuf::Message for DoubleValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.value = tmp;
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
        if self.value != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0. {
            os.write_double(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DoubleValue {
    fn new() -> DoubleValue {
        DoubleValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoubleValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "value",
                    DoubleValue::get_value_for_reflect,
                    DoubleValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoubleValue>(
                    "DoubleValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoubleValue {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoubleValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoubleValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FloatValue {
    // message fields
    pub value: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FloatValue {}

impl FloatValue {
    pub fn new() -> FloatValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FloatValue {
        static mut instance: ::protobuf::lazy::Lazy<FloatValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FloatValue,
        };
        unsafe {
            instance.get(FloatValue::new)
        }
    }

    // float value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0.;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f32) {
        self.value = v;
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &f32 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut f32 {
        &mut self.value
    }
}

impl ::protobuf::Message for FloatValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.value = tmp;
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
        if self.value != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0. {
            os.write_float(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FloatValue {
    fn new() -> FloatValue {
        FloatValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<FloatValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "value",
                    FloatValue::get_value_for_reflect,
                    FloatValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FloatValue>(
                    "FloatValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FloatValue {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FloatValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FloatValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Int64Value {
    // message fields
    pub value: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Int64Value {}

impl Int64Value {
    pub fn new() -> Int64Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Int64Value {
        static mut instance: ::protobuf::lazy::Lazy<Int64Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Int64Value,
        };
        unsafe {
            instance.get(Int64Value::new)
        }
    }

    // int64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = v;
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &i64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.value
    }
}

impl ::protobuf::Message for Int64Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.value = tmp;
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
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(1, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0 {
            os.write_int64(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Int64Value {
    fn new() -> Int64Value {
        Int64Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<Int64Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    Int64Value::get_value_for_reflect,
                    Int64Value::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Int64Value>(
                    "Int64Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Int64Value {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Int64Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Int64Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UInt64Value {
    // message fields
    pub value: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UInt64Value {}

impl UInt64Value {
    pub fn new() -> UInt64Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UInt64Value {
        static mut instance: ::protobuf::lazy::Lazy<UInt64Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UInt64Value,
        };
        unsafe {
            instance.get(UInt64Value::new)
        }
    }

    // uint64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u64) {
        self.value = v;
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &u64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut u64 {
        &mut self.value
    }
}

impl ::protobuf::Message for UInt64Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.value = tmp;
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
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(1, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0 {
            os.write_uint64(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UInt64Value {
    fn new() -> UInt64Value {
        UInt64Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<UInt64Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "value",
                    UInt64Value::get_value_for_reflect,
                    UInt64Value::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UInt64Value>(
                    "UInt64Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UInt64Value {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UInt64Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UInt64Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Int32Value {
    // message fields
    pub value: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Int32Value {}

impl Int32Value {
    pub fn new() -> Int32Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Int32Value {
        static mut instance: ::protobuf::lazy::Lazy<Int32Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Int32Value,
        };
        unsafe {
            instance.get(Int32Value::new)
        }
    }

    // int32 value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = v;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &i32 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut i32 {
        &mut self.value
    }
}

impl ::protobuf::Message for Int32Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.value = tmp;
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
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(1, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0 {
            os.write_int32(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Int32Value {
    fn new() -> Int32Value {
        Int32Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<Int32Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "value",
                    Int32Value::get_value_for_reflect,
                    Int32Value::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Int32Value>(
                    "Int32Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Int32Value {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Int32Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Int32Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UInt32Value {
    // message fields
    pub value: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UInt32Value {}

impl UInt32Value {
    pub fn new() -> UInt32Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UInt32Value {
        static mut instance: ::protobuf::lazy::Lazy<UInt32Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UInt32Value,
        };
        unsafe {
            instance.get(UInt32Value::new)
        }
    }

    // uint32 value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u32) {
        self.value = v;
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &u32 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut u32 {
        &mut self.value
    }
}

impl ::protobuf::Message for UInt32Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.value = tmp;
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
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(1, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0 {
            os.write_uint32(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UInt32Value {
    fn new() -> UInt32Value {
        UInt32Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<UInt32Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "value",
                    UInt32Value::get_value_for_reflect,
                    UInt32Value::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UInt32Value>(
                    "UInt32Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UInt32Value {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UInt32Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UInt32Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BoolValue {
    // message fields
    pub value: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BoolValue {}

impl BoolValue {
    pub fn new() -> BoolValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BoolValue {
        static mut instance: ::protobuf::lazy::Lazy<BoolValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BoolValue,
        };
        unsafe {
            instance.get(BoolValue::new)
        }
    }

    // bool value = 1;

    pub fn clear_value(&mut self) {
        self.value = false;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: bool) {
        self.value = v;
    }

    pub fn get_value(&self) -> bool {
        self.value
    }

    fn get_value_for_reflect(&self) -> &bool {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl ::protobuf::Message for BoolValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.value = tmp;
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
        if self.value != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != false {
            os.write_bool(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BoolValue {
    fn new() -> BoolValue {
        BoolValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<BoolValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "value",
                    BoolValue::get_value_for_reflect,
                    BoolValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BoolValue>(
                    "BoolValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BoolValue {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BoolValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BoolValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StringValue {
    // message fields
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StringValue {}

impl StringValue {
    pub fn new() -> StringValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringValue {
        static mut instance: ::protobuf::lazy::Lazy<StringValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringValue,
        };
        unsafe {
            instance.get(StringValue::new)
        }
    }

    // string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for StringValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_string(1, &self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StringValue {
    fn new() -> StringValue {
        StringValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<StringValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    StringValue::get_value_for_reflect,
                    StringValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StringValue>(
                    "StringValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StringValue {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StringValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StringValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BytesValue {
    // message fields
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BytesValue {}

impl BytesValue {
    pub fn new() -> BytesValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BytesValue {
        static mut instance: ::protobuf::lazy::Lazy<BytesValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BytesValue,
        };
        unsafe {
            instance.get(BytesValue::new)
        }
    }

    // bytes value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for BytesValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_bytes(1, &self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BytesValue {
    fn new() -> BytesValue {
        BytesValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<BytesValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    BytesValue::get_value_for_reflect,
                    BytesValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BytesValue>(
                    "BytesValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BytesValue {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BytesValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BytesValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egoogle/protobuf/wrappers.proto\x12\x0fgoogle.protobuf\"#\n\x0bDoub\
    leValue\x12\x14\n\x05value\x18\x01\x20\x01(\x01R\x05value\"\"\n\nFloatVa\
    lue\x12\x14\n\x05value\x18\x01\x20\x01(\x02R\x05value\"\"\n\nInt64Value\
    \x12\x14\n\x05value\x18\x01\x20\x01(\x03R\x05value\"#\n\x0bUInt64Value\
    \x12\x14\n\x05value\x18\x01\x20\x01(\x04R\x05value\"\"\n\nInt32Value\x12\
    \x14\n\x05value\x18\x01\x20\x01(\x05R\x05value\"#\n\x0bUInt32Value\x12\
    \x14\n\x05value\x18\x01\x20\x01(\rR\x05value\"!\n\tBoolValue\x12\x14\n\
    \x05value\x18\x01\x20\x01(\x08R\x05value\"#\n\x0bStringValue\x12\x14\n\
    \x05value\x18\x01\x20\x01(\tR\x05value\"\"\n\nBytesValue\x12\x14\n\x05va\
    lue\x18\x01\x20\x01(\x0cR\x05valueB|\n\x13com.google.protobufB\rWrappers\
    ProtoP\x01Z*github.com/golang/protobuf/ptypes/wrappers\xf8\x01\x01\xa2\
    \x02\x03GPB\xaa\x02\x1eGoogle.Protobuf.WellKnownTypesJ\xe0!\n\x06\x12\
    \x04#\0u\x01\n\xc3\x0e\n\x01\x0c\x12\x03#\0\x122\xc1\x0c\x20Protocol\x20\
    Buffers\x20-\x20Google's\x20data\x20interchange\x20format\n\x20Copyright\
    \x202008\x20Google\x20Inc.\x20\x20All\x20rights\x20reserved.\n\x20https:\
    //developers.google.com/protocol-buffers/\n\n\x20Redistribution\x20and\
    \x20use\x20in\x20source\x20and\x20binary\x20forms,\x20with\x20or\x20with\
    out\n\x20modification,\x20are\x20permitted\x20provided\x20that\x20the\
    \x20following\x20conditions\x20are\n\x20met:\n\n\x20\x20\x20\x20\x20*\
    \x20Redistributions\x20of\x20source\x20code\x20must\x20retain\x20the\x20\
    above\x20copyright\n\x20notice,\x20this\x20list\x20of\x20conditions\x20a\
    nd\x20the\x20following\x20disclaimer.\n\x20\x20\x20\x20\x20*\x20Redistri\
    butions\x20in\x20binary\x20form\x20must\x20reproduce\x20the\x20above\n\
    \x20copyright\x20notice,\x20this\x20list\x20of\x20conditions\x20and\x20t\
    he\x20following\x20disclaimer\n\x20in\x20the\x20documentation\x20and/or\
    \x20other\x20materials\x20provided\x20with\x20the\n\x20distribution.\n\
    \x20\x20\x20\x20\x20*\x20Neither\x20the\x20name\x20of\x20Google\x20Inc.\
    \x20nor\x20the\x20names\x20of\x20its\n\x20contributors\x20may\x20be\x20u\
    sed\x20to\x20endorse\x20or\x20promote\x20products\x20derived\x20from\n\
    \x20this\x20software\x20without\x20specific\x20prior\x20written\x20permi\
    ssion.\n\n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\x20COPYRIG\
    HT\x20HOLDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20\
    EXPRESS\x20OR\x20IMPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\x20NOT\n\
    \x20LIMITED\x20TO,\x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MERCHANTABIL\
    ITY\x20AND\x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\x20ARE\x20D\
    ISCLAIMED.\x20IN\x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\
    \x20OR\x20CONTRIBUTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIRECT,\x20INDIR\
    ECT,\x20INCIDENTAL,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\
    \x20DAMAGES\x20(INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20PROCUREM\
    ENT\x20OF\x20SUBSTITUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE\
    ,\n\x20DATA,\x20OR\x20PROFITS;\x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOW\
    EVER\x20CAUSED\x20AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WH\
    ETHER\x20IN\x20CONTRACT,\x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INC\
    LUDING\x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\
    \x20OUT\x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\
    \x20ADVISED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n2\xf4\
    \x01\x20Wrappers\x20for\x20primitive\x20(non-message)\x20types.\x20These\
    \x20types\x20are\x20useful\n\x20for\x20embedding\x20primitives\x20in\x20\
    the\x20`google.protobuf.Any`\x20type\x20and\x20for\x20places\n\x20where\
    \x20we\x20need\x20to\x20distinguish\x20between\x20the\x20absence\x20of\
    \x20a\x20primitive\n\x20typed\x20field\x20and\x20its\x20default\x20value\
    .\n\n\x08\n\x01\x02\x12\x03%\x08\x17\n\x08\n\x01\x08\x12\x03'\0;\n\x0b\n\
    \x04\x08\xe7\x07\0\x12\x03'\0;\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03'\
    \x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03'\x07\x17\n\x0e\n\x07\x08\
    \xe7\x07\0\x02\0\x01\x12\x03'\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\
    \x03'\x1a:\n\x08\n\x01\x08\x12\x03(\0\x1f\n\x0b\n\x04\x08\xe7\x07\x01\
    \x12\x03(\0\x1f\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03(\x07\x17\n\r\n\
    \x06\x08\xe7\x07\x01\x02\0\x12\x03(\x07\x17\n\x0e\n\x07\x08\xe7\x07\x01\
    \x02\0\x01\x12\x03(\x07\x17\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03(\x1a\
    \x1e\n\x08\n\x01\x08\x12\x03)\0A\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03)\0A\
    \n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03)\x07\x11\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03)\x07\x11\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03)\x07\x11\n\x0c\n\x05\x08\xe7\x07\x02\x07\x12\x03)\x14@\n\x08\n\x01\
    \x08\x12\x03*\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03*\0,\n\x0c\n\x05\x08\
    \xe7\x07\x03\x02\x12\x03*\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\
    \x03*\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03*\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03*\x16+\n\x08\n\x01\x08\x12\x03+\0.\
    \n\x0b\n\x04\x08\xe7\x07\x04\x12\x03+\0.\n\x0c\n\x05\x08\xe7\x07\x04\x02\
    \x12\x03+\x07\x1b\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03+\x07\x1b\n\x0e\
    \n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03+\x07\x1b\n\x0c\n\x05\x08\xe7\
    \x07\x04\x07\x12\x03+\x1e-\n\x08\n\x01\x08\x12\x03,\0\"\n\x0b\n\x04\x08\
    \xe7\x07\x05\x12\x03,\0\"\n\x0c\n\x05\x08\xe7\x07\x05\x02\x12\x03,\x07\
    \x1a\n\r\n\x06\x08\xe7\x07\x05\x02\0\x12\x03,\x07\x1a\n\x0e\n\x07\x08\
    \xe7\x07\x05\x02\0\x01\x12\x03,\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x05\x03\
    \x12\x03,\x1d!\n\x08\n\x01\x08\x12\x03-\0!\n\x0b\n\x04\x08\xe7\x07\x06\
    \x12\x03-\0!\n\x0c\n\x05\x08\xe7\x07\x06\x02\x12\x03-\x07\x18\n\r\n\x06\
    \x08\xe7\x07\x06\x02\0\x12\x03-\x07\x18\n\x0e\n\x07\x08\xe7\x07\x06\x02\
    \0\x01\x12\x03-\x07\x18\n\x0c\n\x05\x08\xe7\x07\x06\x07\x12\x03-\x1b\x20\
    \ng\n\x02\x04\0\x12\x042\05\x01\x1a[\x20Wrapper\x20message\x20for\x20`do\
    uble`.\n\n\x20The\x20JSON\x20representation\x20for\x20`DoubleValue`\x20i\
    s\x20JSON\x20number.\n\n\n\n\x03\x04\0\x01\x12\x032\x08\x13\n\x20\n\x04\
    \x04\0\x02\0\x12\x034\x02\x13\x1a\x13\x20The\x20double\x20value.\n\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x044\x022\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x034\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x034\t\x0e\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x034\x11\x12\ne\n\x02\x04\x01\x12\x04:\0=\x01\x1aY\x20W\
    rapper\x20message\x20for\x20`float`.\n\n\x20The\x20JSON\x20representatio\
    n\x20for\x20`FloatValue`\x20is\x20JSON\x20number.\n\n\n\n\x03\x04\x01\
    \x01\x12\x03:\x08\x12\n\x1f\n\x04\x04\x01\x02\0\x12\x03<\x02\x12\x1a\x12\
    \x20The\x20float\x20value.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04<\x02:\
    \x14\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03<\x02\x07\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03<\x08\r\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03<\x10\x11\
    \ne\n\x02\x04\x02\x12\x04B\0E\x01\x1aY\x20Wrapper\x20message\x20for\x20`\
    int64`.\n\n\x20The\x20JSON\x20representation\x20for\x20`Int64Value`\x20i\
    s\x20JSON\x20string.\n\n\n\n\x03\x04\x02\x01\x12\x03B\x08\x12\n\x1f\n\
    \x04\x04\x02\x02\0\x12\x03D\x02\x12\x1a\x12\x20The\x20int64\x20value.\n\
    \n\r\n\x05\x04\x02\x02\0\x04\x12\x04D\x02B\x14\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03D\x02\x07\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03D\x08\r\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03D\x10\x11\ng\n\x02\x04\x03\x12\x04J\0M\
    \x01\x1a[\x20Wrapper\x20message\x20for\x20`uint64`.\n\n\x20The\x20JSON\
    \x20representation\x20for\x20`UInt64Value`\x20is\x20JSON\x20string.\n\n\
    \n\n\x03\x04\x03\x01\x12\x03J\x08\x13\n\x20\n\x04\x04\x03\x02\0\x12\x03L\
    \x02\x13\x1a\x13\x20The\x20uint64\x20value.\n\n\r\n\x05\x04\x03\x02\0\
    \x04\x12\x04L\x02J\x15\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03L\x02\x08\n\
    \x0c\n\x05\x04\x03\x02\0\x01\x12\x03L\t\x0e\n\x0c\n\x05\x04\x03\x02\0\
    \x03\x12\x03L\x11\x12\ne\n\x02\x04\x04\x12\x04R\0U\x01\x1aY\x20Wrapper\
    \x20message\x20for\x20`int32`.\n\n\x20The\x20JSON\x20representation\x20f\
    or\x20`Int32Value`\x20is\x20JSON\x20number.\n\n\n\n\x03\x04\x04\x01\x12\
    \x03R\x08\x12\n\x1f\n\x04\x04\x04\x02\0\x12\x03T\x02\x12\x1a\x12\x20The\
    \x20int32\x20value.\n\n\r\n\x05\x04\x04\x02\0\x04\x12\x04T\x02R\x14\n\
    \x0c\n\x05\x04\x04\x02\0\x05\x12\x03T\x02\x07\n\x0c\n\x05\x04\x04\x02\0\
    \x01\x12\x03T\x08\r\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03T\x10\x11\ng\n\
    \x02\x04\x05\x12\x04Z\0]\x01\x1a[\x20Wrapper\x20message\x20for\x20`uint3\
    2`.\n\n\x20The\x20JSON\x20representation\x20for\x20`UInt32Value`\x20is\
    \x20JSON\x20number.\n\n\n\n\x03\x04\x05\x01\x12\x03Z\x08\x13\n\x20\n\x04\
    \x04\x05\x02\0\x12\x03\\\x02\x13\x1a\x13\x20The\x20uint32\x20value.\n\n\
    \r\n\x05\x04\x05\x02\0\x04\x12\x04\\\x02Z\x15\n\x0c\n\x05\x04\x05\x02\0\
    \x05\x12\x03\\\x02\x08\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03\\\t\x0e\n\
    \x0c\n\x05\x04\x05\x02\0\x03\x12\x03\\\x11\x12\no\n\x02\x04\x06\x12\x04b\
    \0e\x01\x1ac\x20Wrapper\x20message\x20for\x20`bool`.\n\n\x20The\x20JSON\
    \x20representation\x20for\x20`BoolValue`\x20is\x20JSON\x20`true`\x20and\
    \x20`false`.\n\n\n\n\x03\x04\x06\x01\x12\x03b\x08\x11\n\x1e\n\x04\x04\
    \x06\x02\0\x12\x03d\x02\x11\x1a\x11\x20The\x20bool\x20value.\n\n\r\n\x05\
    \x04\x06\x02\0\x04\x12\x04d\x02b\x13\n\x0c\n\x05\x04\x06\x02\0\x05\x12\
    \x03d\x02\x06\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03d\x07\x0c\n\x0c\n\x05\
    \x04\x06\x02\0\x03\x12\x03d\x0f\x10\ng\n\x02\x04\x07\x12\x04j\0m\x01\x1a\
    [\x20Wrapper\x20message\x20for\x20`string`.\n\n\x20The\x20JSON\x20repres\
    entation\x20for\x20`StringValue`\x20is\x20JSON\x20string.\n\n\n\n\x03\
    \x04\x07\x01\x12\x03j\x08\x13\n\x20\n\x04\x04\x07\x02\0\x12\x03l\x02\x13\
    \x1a\x13\x20The\x20string\x20value.\n\n\r\n\x05\x04\x07\x02\0\x04\x12\
    \x04l\x02j\x15\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03l\x02\x08\n\x0c\n\
    \x05\x04\x07\x02\0\x01\x12\x03l\t\x0e\n\x0c\n\x05\x04\x07\x02\0\x03\x12\
    \x03l\x11\x12\ne\n\x02\x04\x08\x12\x04r\0u\x01\x1aY\x20Wrapper\x20messag\
    e\x20for\x20`bytes`.\n\n\x20The\x20JSON\x20representation\x20for\x20`Byt\
    esValue`\x20is\x20JSON\x20string.\n\n\n\n\x03\x04\x08\x01\x12\x03r\x08\
    \x12\n\x1f\n\x04\x04\x08\x02\0\x12\x03t\x02\x12\x1a\x12\x20The\x20bytes\
    \x20value.\n\n\r\n\x05\x04\x08\x02\0\x04\x12\x04t\x02r\x14\n\x0c\n\x05\
    \x04\x08\x02\0\x05\x12\x03t\x02\x07\n\x0c\n\x05\x04\x08\x02\0\x01\x12\
    \x03t\x08\r\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03t\x10\x11b\x06proto3\
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
