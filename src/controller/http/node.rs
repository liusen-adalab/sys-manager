use std::net::IpAddr;

use actix_web::web::Json;
use serde::Deserialize;

use crate::application;

use super::{ApiResponse, JsonResponse};

#[derive(Deserialize)]
pub struct AddNodeParams {
    ip: IpAddr,
    password: String,
}

/// 往系统中加入一个节点
pub async fn add_node(params: Json<AddNodeParams>) -> JsonResponse<()> {
    let AddNodeParams { ip, password } = params.into_inner();
    application::add_node(ip, password).await?;
    ApiResponse::ok(())
}

#[derive(Deserialize)]
pub struct DelNodeParams {
    ip: IpAddr,
}

/// 删除一个节点，并清除其中部署的服务
pub async fn del_node(params: Json<DelNodeParams>) -> JsonResponse<()> {
    let DelNodeParams { ip } = params.into_inner();
    application::del_node(ip).await?;
    ApiResponse::ok(())
}
