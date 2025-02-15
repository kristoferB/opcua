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
    localized_text::LocalizedText,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectAttributes {
    pub specified_attributes: u32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: u32,
    pub user_write_mask: u32,
    pub event_notifier: u8,
}

impl BinaryEncoder<ObjectAttributes> for ObjectAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.event_notifier.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.event_notifier.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let specified_attributes = u32::decode(stream, decoding_options)?;
        let display_name = LocalizedText::decode(stream, decoding_options)?;
        let description = LocalizedText::decode(stream, decoding_options)?;
        let write_mask = u32::decode(stream, decoding_options)?;
        let user_write_mask = u32::decode(stream, decoding_options)?;
        let event_notifier = u8::decode(stream, decoding_options)?;
        Ok(ObjectAttributes {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            event_notifier,
        })
    }
}
