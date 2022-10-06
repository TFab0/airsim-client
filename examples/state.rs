use airsim_client::{MultiRotorClient, NetworkResult};
use async_std::task;

async fn connect_drone() -> NetworkResult<()> {
    let address = "172.28.192.1:41451";
    let vehicle_name = "";

    log::info!("Start!");

    // connect
    log::info!("connect");
    let client = MultiRotorClient::connect(address, vehicle_name).await?;

    // confirm connect
    log::info!("confirm connection");
    let res = client.confirm_connection().await?;
    log::info!("Response: {:?}", res);

    // get state
    log::info!("get st  ate");
    let state = client.get_multirotor_state().await?;
    log::info!("Response: {:?}", state);

    log::info!("Done!");
    Ok(())
}

fn main() -> NetworkResult<()> {
    env_logger::init();
    task::block_on(connect_drone())
}
