use rmp_rpc::{Value};

use crate::Vector3;

pub struct Path(pub Vec<Vector3>);

impl Path {
    pub fn to_msgpack(&self) -> Value {
        let v3_msgpack = self.0.to_owned().into_iter().map(|v3| v3.to_msgpack()).collect();
        Value::Array(v3_msgpack)
    }
}
