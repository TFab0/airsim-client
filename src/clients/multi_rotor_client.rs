use async_std::net::ToSocketAddrs;
use rmp_rpc::Utf8String;
use rmpv::Value;

use crate::types::drive_train::DrivetrainType;
use crate::types::geopoint::GeoPoint;
use crate::types::pose::Position;
use crate::types::yaw_mode::YawMode;
use crate::{error::NetworkResult, NetworkError};

use super::airsim_client::AirsimClient;

pub struct MultiRotorClient {
    airsim_client: AirsimClient,
    vehicle_name: &'static str,
}

impl MultiRotorClient {
    pub async fn connect(addrs: impl ToSocketAddrs, vehicle_name: &'static str) -> NetworkResult<Self> {
        let airsim_client = AirsimClient::new(addrs, vehicle_name).await?;
        Ok(Self {
            airsim_client,
            vehicle_name,
        })
    }

    /// Reset the vehicle to its original starting state
    ///
    /// Note that you must call `enable_api_control` and `arm_disarm` again after the call to reset
    #[inline(always)]
    pub async fn reset(&self) -> NetworkResult<bool> {
        self.airsim_client.reset().await
    }

    /// If connection is established then this call will return `True` otherwise
    /// the request will be blocked until timeout (default value)
    #[inline(always)]
    pub async fn ping(&self) -> NetworkResult<bool> {
        self.airsim_client.ping().await
    }

    #[inline(always)]
    pub async fn confirm_connection(&self) -> NetworkResult<bool> {
        self.airsim_client.confirm_connection().await
    }

    /// Enables or disables API control for vehicle corresponding to vehicle_name
    ///
    /// args:
    ///     is_enabled (bool): True to enable, False to disable API control
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    #[inline(always)]
    pub async fn enable_api_control(&self, is_enabled: bool) -> NetworkResult<bool> {
        self.airsim_client
            .enable_api_control(is_enabled, Some(self.vehicle_name))
            .await
    }

    /// Returns true if API control is established.
    ///
    /// If false (which is default) then API calls would be ignored. After a successful call
    /// to `enableApiControl`, `isApiControlEnabled` should return true.
    ///
    /// args:
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    #[inline(always)]
    pub async fn is_api_control_enabled(&self, is_enabled: bool) -> NetworkResult<bool> {
        self.airsim_client
            .is_api_control_enabled(is_enabled, Some(self.vehicle_name))
            .await
    }

    /// Returns true if API control is established.
    ///
    /// If false (which is default) then API calls would be ignored. After a successful call
    /// to `enableApiControl`, `isApiControlEnabled` should return true.
    ///
    /// args:
    ///     arm (bool): True to arm, False to disarm the vehicle
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    #[inline(always)]
    pub async fn arm_disarm(&self, arm: bool) -> NetworkResult<bool> {
        self.airsim_client.arm_disarm(arm, Some(self.vehicle_name)).await
    }

    /// Takeoff vehicle to 3m above ground. Vehicle should not be moving when this API is used
    ///
    /// Args:
    ///     timeout_sec (Option<u64>): Timeout for the vehicle to reach desired altitude
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    pub async fn take_off_async(&self, timeout_sec: u64) -> NetworkResult<bool> {
        let vehicle_name: Utf8String = self.vehicle_name.into();

        self.airsim_client
            .unary_rpc(
                "takeoff".into(),
                Some(vec![Value::Integer(timeout_sec.into()), Value::String(vehicle_name)]),
            )
            .await
            .map_err(Into::into)
            .map(|response| response.result.is_ok() && response.result.unwrap().as_bool() == Some(true))
    }

    /// Get the Home location of the vehicle
    ///
    /// Args:
    ///     vehicle_name (Option<String>): Name of the vehicle to send this command to
    pub async fn get_home_geo_point(&self) -> Result<GeoPoint, NetworkError> {
        self.airsim_client.get_home_geo_point(Some(self.vehicle_name)).await
    }

    /// Send desired goal position to default PID vehicle controller
    ///
    /// Args:
    ///     position (Position): goal position of the vehicle controller
    ///     velocity (f32): desired velocity in NED frame of the vehicle
    ///     timeout_sec (32): Timeout for the vehicle to reach desired goal position
    ///     drivetrain (DrivetrainType): when ForwardOnly, vehicle rotates itself so that its front is always facing the direction of travel. If MaxDegreeOfFreedom then it doesn't do that (crab-like movement)
    ///     yaw_mode (YawMode): Specifies if vehicle should face at given angle (is_rate=False) or should be rotating around its axis at given rate (is_rate=True)
    ///     lookahead (Option<i32>): defaults to `-1`
    ///     adaptive_lookahead (Option<i32>): defaults to `0`
    pub async fn move_to_position_async(
        &self,
        position: Position,
        velocity: f32,
        timeout_sec: f32,
        drivetrain: DrivetrainType,
        yaw_mode: YawMode,
        lookahead: Option<i32>,
        adaptive_lookahead: Option<i32>,
    ) -> NetworkResult<bool> {
        let lookahead = lookahead.unwrap_or(-1) as i64;
        let adaptive_lookahead = adaptive_lookahead.unwrap_or(0) as i64;

        self.airsim_client
            .unary_rpc(
                "moveToPositionAsync".into(),
                Some(vec![
                    rmp_rpc::Value::F32(position.x),
                    rmp_rpc::Value::F32(position.y),
                    rmp_rpc::Value::F32(position.z),
                    rmp_rpc::Value::F32(velocity),
                    rmp_rpc::Value::F32(timeout_sec),
                    drivetrain.to_msgpack(),
                    yaw_mode.to_msgpack(),
                    rmp_rpc::Value::Integer(lookahead.into()),
                    rmp_rpc::Value::Integer(adaptive_lookahead.into()),
                ]),
            )
            .await
            .map_err(Into::into)
            .map(|response| {
                let x = response.result;
                println!("res: {:?}", x);
                x.is_ok()
            })
    }
}
