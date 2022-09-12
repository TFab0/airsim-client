use airsim_client::{DrivetrainType, MultiRotorClient, NetworkResult, Velocity3, YawMode, Velocity2};
use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.21.112.1:41451"; // set with env variable
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let client = MultiRotorClient::connect(address, vehicle_name).await?;

    // confirm connect
    log::info!("confirm connection");
    let res = client.confirm_connection().await?;
    log::info!("Response: {:?}", res);

    // arm drone
    log::info!("arm drone");
    client.arm_disarm(true).await?;
    log::info!("Response: {:?}", res);

    // take off
    log::info!("take off drone");
    client.take_off_async(20.0).await?;
    log::info!("take off completed");

    log::info!("move by 3D velocity in body frame");
    client
    .move_by_velocity_body_frame_async(
        Velocity3::new(-5.0, 5.0, -10.0),
        10.0,
        DrivetrainType::MaxDegreeOfFreedom,
        YawMode::new(false, 45.0),
    )
    .await?;
    log::info!("done!");

    log::info!("move by 2D velocity body frame with fixed Z");
    client
    .move_by_velocity_z_body_frame_async(
        Velocity2::new(-3.0, -3.0),
    -15.0,
        15.0,
        DrivetrainType::MaxDegreeOfFreedom,
        YawMode::new(false, 45.0),
    )
    .await?;
    log::info!("done!");

    log::info!("move by 3D velocity in world NED frame");
    client
    .move_by_velocity_async(
        Velocity3::new(-3.0, -3.0, -3.0),
    6.0,
        DrivetrainType::MaxDegreeOfFreedom,
        YawMode::new(false, -45.0),
    )
    .await?;
    log::info!("done!");

    log::info!("move by 2D velocity world NED frame with fixed Z");
    client
    .move_by_velocity_z_async(
        Velocity2::new(-3.0, -3.0),
    -10.0,
        15.0,
        DrivetrainType::MaxDegreeOfFreedom,
        YawMode::new(false, 45.0),
    )
    .await?;
    log::info!("done!");
    
    log::info!("go home");
    client.go_home_async(20.0).await?;
    log::info!("got home");

    log::info!("land drone");
    let landed = client.land_async(20.0).await?;
    log::info!("drone landed: {}", landed);

    client.arm_disarm(false).await?;
    client.enable_api_control(false).await?;
    log::info!("Mission done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
