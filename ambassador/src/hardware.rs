use std::time::Duration;

use anyhow::Result;
use futures::SinkExt;
use protocol::monitor::{MachineReport, MonitorMessage};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[allow(dead_code)]
pub struct Client {}

impl Client {
    #[allow(dead_code)]
    pub async fn connect() -> Result<()> {
        let addr = "127.0.0.1:8991";

        let stream = TcpStream::connect(addr).await?;
        let mut framed = Framed::new(stream, protocol::monitor::Codec::new());

        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let msg = MonitorMessage::MachineReport(MachineReport {
                cpu_usage: 20.0,
                memory_usage: 20.1,
            });
            framed.send(msg).await?;
        }
    }
}
