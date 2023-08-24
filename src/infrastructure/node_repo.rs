use anyhow::Result;
use std::net::IpAddr;

use crate::domain::node::Node;

pub async fn find(ip: &IpAddr) -> Result<Option<Node>> {
    Ok(Some(Node::new(ip.to_owned(), "540hs0qos".to_string())))
}
