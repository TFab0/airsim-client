use msgpack_rpc::Value;
use nalgebra::Quaternion;

#[derive(Debug, Clone)]
pub struct Quaternionr(pub Quaternion<f32>);

impl From<Value> for Quaternionr {
    fn from(msgpack: Value) -> Self {
        let mut points = vec![];
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();
        for (_, v) in payload {
            let p: f32 = v.as_f64().unwrap() as f32;
            points.push(p);
        }

        Self(Quaternion::new(points[0], points[1], points[2], points[3]))
    }
}
