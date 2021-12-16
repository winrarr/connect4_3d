use std::net::{IpAddr, Ipv6Addr};

use service::WorldClient;
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
pub async fn run(addr: (IpAddr, u16)) -> anyhow::Result<()> {
    let transport = tarpc::serde_transport::tcp::connect(&addr, Json::default);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    let client = WorldClient::new(client::Config::default(), transport.await?).spawn();

    let place = client.place(context::current(), 1, 2, 3).await?;
    println!("client: {}", place);

    Ok(())
}
