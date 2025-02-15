// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    string::UAString,
    variant::Variant,
    service_types::StructureDescription,
    service_types::EnumDescription,
    service_types::SimpleTypeDescription,
    service_types::KeyValuePair,
};

#[derive(Debug, Clone, PartialEq)]
pub struct UABinaryFileDataType {
    pub namespaces: Option<Vec<UAString>>,
    pub structure_data_types: Option<Vec<StructureDescription>>,
    pub enum_data_types: Option<Vec<EnumDescription>>,
    pub simple_data_types: Option<Vec<SimpleTypeDescription>>,
    pub schema_location: UAString,
    pub file_header: Option<Vec<KeyValuePair>>,
    pub body: Variant,
}

impl BinaryEncoder<UABinaryFileDataType> for UABinaryFileDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.namespaces);
        size += byte_len_array(&self.structure_data_types);
        size += byte_len_array(&self.enum_data_types);
        size += byte_len_array(&self.simple_data_types);
        size += self.schema_location.byte_len();
        size += byte_len_array(&self.file_header);
        size += self.body.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.namespaces)?;
        size += write_array(stream, &self.structure_data_types)?;
        size += write_array(stream, &self.enum_data_types)?;
        size += write_array(stream, &self.simple_data_types)?;
        size += self.schema_location.encode(stream)?;
        size += write_array(stream, &self.file_header)?;
        size += self.body.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let namespaces: Option<Vec<UAString>> = read_array(stream, decoding_options)?;
        let structure_data_types: Option<Vec<StructureDescription>> = read_array(stream, decoding_options)?;
        let enum_data_types: Option<Vec<EnumDescription>> = read_array(stream, decoding_options)?;
        let simple_data_types: Option<Vec<SimpleTypeDescription>> = read_array(stream, decoding_options)?;
        let schema_location = UAString::decode(stream, decoding_options)?;
        let file_header: Option<Vec<KeyValuePair>> = read_array(stream, decoding_options)?;
        let body = Variant::decode(stream, decoding_options)?;
        Ok(UABinaryFileDataType {
            namespaces,
            structure_data_types,
            enum_data_types,
            simple_data_types,
            schema_location,
            file_header,
            body,
        })
    }
}
