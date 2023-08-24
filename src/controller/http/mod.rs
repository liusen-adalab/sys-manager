use actix_web::{
    body::BoxBody,
    http::StatusCode,
    web::{self, Json, ServiceConfig},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::Serialize;
use tracing::{error, warn};

pub mod node;

type Result<T, E = ApiError> = std::result::Result<T, E>;
pub type JsonResponse<T> = Result<Json<ApiResponse<T>>>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    pub status: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Result<Json<Self>> {
        Ok(Json(Self {
            status: 0,
            err_msg: None,
            data: Some(data),
        }))
    }
}

#[repr(u8)]
#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "[debug only] server internal err: {source:#?}")]
    Internal { source: anyhow::Error } = 1,

    #[display(fmt = "bad requeset. code = {_0}, err = {_1}")]
    BusinessError(u32, String),
}

impl From<anyhow::Error> for ApiError {
    #[track_caller]
    fn from(value: anyhow::Error) -> Self {
        Self::Internal { source: value }
    }
}

impl ApiError {
    fn code(&self) -> u32 {
        match self {
            ApiError::Internal { .. } => 1,
            ApiError::BusinessError(code, _) => *code,
        }
    }
}

pub trait ToAnyhow<T, E> {
    fn to_anyhow(self) -> anyhow::Result<T>;
}

impl<T, E> ToAnyhow<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn to_anyhow(self) -> anyhow::Result<T> {
        Ok(self?)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ApiError::Internal { .. } => {
                error!(err = %self, "server internal err");
            }
            _ => {
                warn!(event = %self, "received bad request")
            }
        }
        let resp = ApiResponse::<()> {
            status: self.code(),
            err_msg: Some(self.to_string()),
            data: None,
        };
        HttpResponse::build(self.status_code()).json(resp)
    }
}

pub fn config_endpoints(conf: &mut ServiceConfig) {
    conf.service(
        web::scope("/api/node")
            .route("/add", web::post().to(node::add_node))
            .route("/del", web::post().to(node::del_node)),
    );
}
