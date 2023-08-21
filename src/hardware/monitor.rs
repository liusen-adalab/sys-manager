use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::OnceLock,
};

use anyhow::Result;
use futures::StreamExt;
use protocol::monitor::{self, MachineReport};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{oneshot, RwLock},
};
use tokio_util::codec::Framed;
use tracing::{error, info};

use crate::log_if_err;

static MANAGER: OnceLock<WorkerManager> = OnceLock::new();

pub fn manager() -> &'static WorkerManager {
    MANAGER.get_or_init(|| WorkerManager {
        workers: Default::default(),
    })
}

type Channel = Framed<TcpStream, protocol::monitor::Codec>;

pub struct WorkerManager {
    workers: RwLock<HashMap<IpAddr, WrokerHandler>>,
}

impl WorkerManager {
    pub fn start_server() -> Result<()> {
        tokio::spawn(Self::server_inner());
        Ok(())
    }

    async fn server_inner() -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:8991").await?;
        loop {
            let (stream, addr) = listener.accept().await?;
            let handler = Worker::new(stream, addr);
            let mut lock = manager().workers.write().await;
            lock.insert(addr.ip(), handler);
        }
    }
}

#[allow(dead_code)]
struct WrokerHandler {
    hander: oneshot::Sender<WorkerCommand>,
}

#[allow(dead_code)]
enum WorkerCommand {
    Exit,
}

#[allow(dead_code)]
struct Worker {
    ip: IpAddr,
    commander: oneshot::Receiver<WorkerCommand>,
    stream: Channel,
}

impl Worker {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> WrokerHandler {
        let (tx, rx) = oneshot::channel();
        Self {
            ip: addr.ip(),
            commander: rx,
            stream: Framed::new(stream, protocol::monitor::Codec::new()),
        }
        .run();
        WrokerHandler { hander: tx }
    }

    pub fn run(mut self) {
        tokio::spawn(async move {
            log_if_err!(self.run_inner().await);
        });
    }

    async fn run_inner(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                content = self.stream.next() => {
                    match content{
                        Some(Ok(req)) => {
                            self.handle_client_req(req).await?;
                        }
                        Some(Err(err)) => {
                            error!(?err, "failed to read client tcp stream");
                            break;
                        }
                        None => {
                            info!("remote worker exited");
                            break
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_client_req(&self, msg: monitor::MonitorMessage) -> Result<()> {
        match msg {
            monitor::MonitorMessage::HeartBeat => {}
            monitor::MonitorMessage::MachineReport(MachineReport {
                cpu_usage,
                memory_usage,
            }) => {
                info!(ip=%self.ip, %cpu_usage, ?memory_usage);
            }
        }
        Ok(())
    }
}
