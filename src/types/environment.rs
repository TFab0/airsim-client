use msgpack_rpc::{message::Response, Value};

use crate::types::{geopoint::GeoPoint, vector::Vector3};

#[derive(Debug, Clone, Copy)]
pub struct EnvironmentState {
    pub position: Vector3,
    pub geo_point: GeoPoint,
    pub gravity: Vector3,
    pub air_pressure: f32,
    pub air_temperature: f32,
    pub air_density: f32,
}


impl From<Response> for EnvironmentState {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                let position: Vector3 = payload[0].1.to_owned().into();
                let geo_point: GeoPoint = payload[1].1.to_owned().into();
                let gravity: Vector3 = payload[2].1.to_owned().into();
                let air_pressure: f32 = payload[3].1.as_f64().unwrap() as f32;
                let air_temperature: f32 = payload[4].1.as_f64().unwrap() as f32;
                let air_density: f32 = payload[5].1.as_f64().unwrap() as f32;
                Self { position, geo_point, gravity, air_pressure, air_temperature, air_density }
            }
            Err(_) => panic!("Could not decode result from EnvironmentState msgpack")
        }
    }
}
