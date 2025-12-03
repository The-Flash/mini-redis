use std::fmt::{self, Formatter};

use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(Debug)]
pub enum Error {
    Incomplete,
    /// Invalid message encoding
    Other(crate::Error),
}

impl Frame {
    pub(crate) fn array() -> Self {
        Self::Array(vec![])
    }

    pub(crate) fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Bulk(bytes));
            }
            _ => panic!("not an array frame")
        }
    }

    pub(crate) fn push_int(&mut self, value: u64) {
         match self {
             Frame::Array(vec) => {
                    vec.push(Frame::Integer(value));
             }
             _ => panic!("not an integer frame")
         }
    }

    pub(crate) fn check() {
        todo!()
    }

    pub fn parse() {
        todo!()
    }

    pub(crate) fn to_error(self) -> crate::Error {
            format!("unexpected frame: {}", self).into()
    }
}


impl fmt::Display for Frame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use std::str;
        match self {
            Frame::Simple(response) => response.fmt(f),
            Frame::Error(msg) => write!(f, "error: {}", msg),
            Frame::Integer(num) => num.fmt(f),
            Frame::Bulk(bytes) => match str::from_utf8(bytes) {
                Ok(s) => s.fmt(f),
                Err(_) => write!(f, "{:?}", bytes)
            },
            Frame::Null => "{nil}".fmt(f),
            Frame::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    part.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}
