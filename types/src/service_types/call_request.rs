// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use service_types::impls::RequestHeader;
use service_types::CallMethodRequest;

#[derive(Debug, Clone, PartialEq)]
pub struct CallRequest {
    pub request_header: RequestHeader,
    pub methods_to_call: Option<Vec<CallMethodRequest>>,
}

impl MessageInfo for CallRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CallRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CallRequest> for CallRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.methods_to_call);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.methods_to_call)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let methods_to_call: Option<Vec<CallMethodRequest>> = read_array(stream)?;
        Ok(CallRequest {
            request_header,
            methods_to_call,
        })
    }
}