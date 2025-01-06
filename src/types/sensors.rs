use msgpack_rpc::{message::Response, Utf8String, Value};
use crate::{GeoPoint, Pose3};

use super::vector::Vector3;

use super::quaternion::Quaternionr;

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Scene,
    DepthPlanar,
    DepthPerspective,
    DepthVis,
    DisparityNormalized,
    SurfaceNormals,
    Infrared,
    OpticalFlow,
    OpticalFlowVis,
}

impl ImageType {
    pub(crate) fn as_msgpack(&self) -> Value {
        let val = match self {
            ImageType::Scene => 0_i64,
            ImageType::DepthPlanar => 1_i64,
            ImageType::DepthPerspective => 2_i64,
            ImageType::DepthVis => 3_i64,
            ImageType::DisparityNormalized => 4_i64,
            ImageType::SurfaceNormals => 5_i64,
            ImageType::Infrared => 6_i64,
            ImageType::OpticalFlow => 7_i64,
            ImageType::OpticalFlowVis => 8_i64,
        };

        Value::Integer(val.into())
    }
}

#[derive(Debug, Clone)]
/// Binary string literal of compressed png image in presented as an vector of bytes
pub struct CompressedImage(pub Vec<u8>);

impl From<Response> for CompressedImage {
    fn from(msgpack: Response) -> Self {
        let mut pixels = vec![];

        match msgpack.result {
            Ok(res) => {
                let slice: &[u8] = res.as_slice().unwrap();
                for p in slice {
                    pixels.push(*p);
                }
            }
            Err(_) => panic!("Could not decode result from CompressedImage msgpack"),
        };

        Self(pixels)
    }
}

#[derive(Debug, Clone)]
pub struct ImageRequest {
    pub camera_name: String,
    pub image_type: ImageType,
    pub pixels_as_float: bool,
    pub compress: bool,
}

#[derive(Debug, Clone)]
pub struct ImageRequests(pub Vec<ImageRequest>);

impl ImageRequest {
    pub(crate) fn as_msgpack(&self) -> Value {
        let camera_name: Utf8String = "camera_name".into();
        let image_type: Utf8String = "image_type".into();
        let pixels_as_float: Utf8String = "pixels_as_float".into();
        let compress: Utf8String = "compress".into();

        let val = Value::Map(vec![
            (
                Value::String(camera_name),
                Value::String(self.camera_name.to_owned().into()),
            ),
            (Value::String(image_type), self.image_type.as_msgpack()),
            (Value::String(pixels_as_float), Value::Boolean(self.pixels_as_float)),
            (Value::String(compress), Value::Boolean(self.compress)),
        ]);

        let msg: Vec<(msgpack_rpc::Value, msgpack_rpc::Value)> = val.as_map().map(|x| x.to_owned()).unwrap();
        Value::Map(msg)
    }
}

impl ImageRequests {
    pub(crate) fn as_msgpack(&self) -> Value {
        let images = self.0.iter().cloned().map(|img| img.as_msgpack()).collect();
        Value::Array(images)
    }
}

pub struct ImuData {
    pub timestamp: u64,
    pub orientation: Quaternionr,
    pub angular_velocity: Vector3, // rad/s
    pub linear_acceleration: Vector3, // m/s^2
}

impl From<Response> for ImuData {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                let timestamp: u64 = payload[0].1.as_u64().unwrap();
                let orientation: Quaternionr = payload[1].1.to_owned().into();
                let angular_velocity: Vector3 = payload[2].1.to_owned().into();
                let linear_acceleration: Vector3 = payload[3].1.to_owned().into();

                Self {
                    timestamp,
                    orientation,
                    angular_velocity,
                    linear_acceleration
                }
            }
            Err(_) => panic!("Couldn't decode result from ImuData msgpack")
        }
    }
}


pub struct DistanceSensorData {
    pub timestamp: u64,
    pub distance: f32, // meters
    pub min_distance: f32, // meters
    pub max_distance: f32, // meters
    pub relative_pose: Pose3, 
}

impl From<Response> for DistanceSensorData {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                let timestamp: u64 = payload[0].1.as_u64().unwrap();
                let distance: f32 = payload[1].1.as_f64().unwrap() as f32;
                let min_distance: f32 = payload[2].1.as_f64().unwrap() as f32;
                let max_distance: f32 = payload[3].1.as_f64().unwrap() as f32;
                let relative_pose: Pose3 = payload[4].1.to_owned().into();
                Self { timestamp, distance, min_distance, max_distance, relative_pose }
            }
            Err(_) => panic!("Couldn't decode result from DistanceSensorData msgpack")
        }
    }
}


pub struct MagnetometerData {
    pub timestamp: u64,
    pub magnetic_field: Vector3,
    pub magnetic_field_covariance: f32,
}

impl From<Response> for MagnetometerData {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                let timestamp: u64 = payload[0].1.as_u64().unwrap();
                let magnetic_field: Vector3 = payload[1].1.to_owned().into();
                // let magnetic_field_covariance: f32 = payload[2].1.as_f64().unwrap() as f32;
                Self { timestamp, magnetic_field, magnetic_field_covariance: 0.0 }
            }
            Err(_) => panic!("Couldn't decode result from MagnetometerData msgpack")
        }
    }
}

pub struct BarometerData {
    pub timestamp: u64,
    pub altitude: f32,
    pub pressure: f32,
    pub qnh: f32,
}

impl From<Response> for BarometerData {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                let timestamp: u64 = payload[0].1.as_u64().unwrap();
                let pressure: f32 = payload[1].1.as_f64().unwrap() as f32;
                let altitude: f32 = payload[2].1.as_f64().unwrap() as f32;
                let qnh: f32 = payload[3].1.as_f64().unwrap() as f32;
                Self { timestamp, altitude, pressure, qnh }
            }
            Err(_) => panic!("Couldn't decode result from BarometerData msgpack")
        }
    }
}

pub struct GpsData {
    pub timestamp: u64,
    pub gnss_report: GnssReport,
    pub is_valid: bool,
}

impl From<Response> for GpsData {
    fn from(msgpack: Response) -> Self {
        match msgpack.result {
            Ok(res) => {
                let payload: &Vec<(Value, Value)> = res.as_map().unwrap();
                let timestamp: u64 = payload[0].1.as_u64().unwrap();
                let gnss_report: GnssReport = payload[1].1.to_owned().into();
                let is_valid: bool = payload[2].1.as_bool().unwrap();
                Self { timestamp, gnss_report, is_valid }
            }
            Err(e) => {
                println!("Error decoding Response for GpsData: {:?}", e);
                panic!("Couldn't decode result from GpsData msgpack")
            }
        }
    }
}

pub struct GnssReport {
    pub geo_point: GeoPoint,
    pub eph: f32,
    pub epv: f32,
    pub velocity: Vector3,
    pub fix_type: GnssFixType,
    pub time_utc: u64,
}
impl From<Value> for GnssReport {
    fn from(msgpack: Value) -> Self {
        let payload: &Vec<(Value, Value)> = msgpack.as_map().unwrap();
        let geo_point: GeoPoint = payload[0].1.to_owned().into();
        let eph: f32 = payload[1].1.as_f64().unwrap() as f32;
        let epv: f32 = payload[2].1.as_f64().unwrap() as f32;
        let velocity: Vector3 = payload[3].1.to_owned().into();
        let fix_type: GnssFixType = match payload[4].1.as_u64().unwrap() {
            0 => GnssFixType::GnssFixNoFix,
            1 => GnssFixType::GnssFixTimeOnly, 
            2 => GnssFixType::GnssFix2DFix,
            3 => GnssFixType::GnssFix3DFix,
            _ => panic!("Invalid GNSS fix type")
        };
        let time_utc: u64 = payload[5].1.as_u64().unwrap();
        Self { geo_point, eph, epv, velocity, fix_type, time_utc }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GnssFixType {
    GnssFixNoFix = 0,
    GnssFixTimeOnly = 1,
    GnssFix2DFix = 2,
    GnssFix3DFix = 3,
}

/*
        enum GnssFixType : unsigned char
        {
            GNSS_FIX_NO_FIX = 0,
            GNSS_FIX_TIME_ONLY = 1,
            GNSS_FIX_2D_FIX = 2,
            GNSS_FIX_3D_FIX = 3
        }; */

/*
struct GnssReport
        {
            GeoPoint geo_point;
            msr::airlib::real_T eph = 0.0, epv = 0.0;
            Vector3r velocity;
            msr::airlib::GpsBase::GnssFixType fix_type;
            uint64_t time_utc = 0;

            MSGPACK_DEFINE_MAP(geo_point, eph, epv, velocity, fix_type, time_utc);

            GnssReport()
            {
            }

            GnssReport(const msr::airlib::GpsBase::GnssReport& s)
            {
                geo_point = s.geo_point;
                eph = s.eph;
                epv = s.epv;
                velocity = s.velocity;
                fix_type = s.fix_type;
                time_utc = s.time_utc;
            }

            msr::airlib::GpsBase::GnssReport to() const
            {
                msr::airlib::GpsBase::GnssReport d;

                d.geo_point = geo_point.to();
                d.eph = eph;
                d.epv = epv;
                d.velocity = velocity.to();
                d.fix_type = fix_type;
                d.time_utc = time_utc;

                return d;
            }
        }; */