use log::{debug, error};

use actix_web::client::Client;
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorResponse, ServiceError};

#[derive(Debug, Deserialize, Serialize)]
pub struct Userinfo {
    pub sub: String,
}

pub async fn fetch(token: &str) -> Result<Userinfo, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let url = &format!("{}{}", authority.as_str(), "userinfo");
    debug!("url = {:?}", url);

    let userinfo_result = Client::default().get(url).bearer_auth(token).send().await;
    if userinfo_result.is_err() {
        let msg = "Failed to fetch userinfo";
        error!("{}", msg);
        return Err(ServiceError::InternalServerError(ErrorResponse::new(msg)));
    }

    let mut userinfo_response = userinfo_result.unwrap();
    if !userinfo_response.status().is_success() {
        let msg = "Failed to fetch userinfo";
        error!("{}", msg);
        return Err(ServiceError::InternalServerError(ErrorResponse::new(msg)));
    }

    let userinfo = userinfo_response.json::<Userinfo>().await.unwrap();
    return Ok(userinfo);
}