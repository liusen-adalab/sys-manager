use std::net::IpAddr;

use anyhow::Result;

use crate::{domain::node::Node, infrastructure::node_repo};

pub mod nodes;

/// 往系统中加入一个节点
pub async fn add_node(ip: IpAddr, password: String) -> Result<()> {
    let node = Node::new(ip, password);
    node.install().await?;
    Ok(())
}

/// 删除一个节点，并清除其中部署的服务
pub async fn del_node(ip: IpAddr) -> Result<()> {
    let Some(node) = node_repo::find(&ip).await? else {
        return Ok(());
    };
    node.uninstall().await?;
    Ok(())
}
