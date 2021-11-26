use bytes::Bytes;
use http::StatusCode;

use crate::{CommandResponse, Hget, Hgetall, command_request::RequestData, error::KvError};

use self::abi::{
    command_request::{self},
    value, CommandRequest, Hset, Kvpair, Value,
};

pub mod abi;

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, val: Value) -> Self {
        CommandRequest {
            request_data: Some(command_request::RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key.into(), val)),
            })),
        }
    }
    /// 创建 HGETALL 命令
    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    /// 创建 HSET 命令
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, val: Value) -> Kvpair {
        Kvpair {
            key: key.into(),
            value: Some(val),
        }
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value {
            value: Some(value::Value::String(s)),
        }
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value {
            value: Some(value::Value::String(s.into())),
        }
    }
}
/// 从 i64转换成 Value
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i)),
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self{
            status: StatusCode::OK.as_u16()  as _,
            values: vec![v],
            .. Default::default()
        }
    }
}

impl From<Vec<Value>> for CommandResponse {
    fn from(values: Vec<Value>) -> Self {
        Self{
            status: StatusCode::OK.as_u16()  as _,
            values: values,
            .. Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }

        result
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(pairs: Vec<Kvpair>) -> Self {
        Self{
            status: StatusCode::OK.as_u16()  as _,
            pairs: pairs,
            .. Default::default()
        }
    }
}

impl<const N: usize> From<&[u8; N]> for Value {
    fn from(buf: &[u8; N]) -> Self {
        Bytes::copy_from_slice(&buf[..]).into()
    }
}

impl From<Bytes> for Value {
    fn from(buf: Bytes) -> Self {
        Self {
            value: Some(value::Value::Binary(buf)),
        }
    }
}