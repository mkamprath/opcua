// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EventFilter {
    pub select_clauses: Option<Vec<SimpleAttributeOperand>>,
    pub where_clause: ContentFilter,
}

impl BinaryEncoder<EventFilter> for EventFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.select_clauses);
        size += self.where_clause.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.select_clauses)?;
        size += self.where_clause.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let select_clauses: Option<Vec<SimpleAttributeOperand>> = read_array(stream)?;
        let where_clause = ContentFilter::decode(stream)?;
        Ok(EventFilter {
            select_clauses: select_clauses,
            where_clause: where_clause,
        })
    }
}