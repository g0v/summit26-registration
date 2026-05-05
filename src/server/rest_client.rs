use reqwest::header::{HeaderMap, HeaderValue};

use crate::models::{QrCodeDataRequest, QrCodeDataResponse};

use super::{config::VerifierApiSettings, error::AppError};

#[derive(Clone)]
pub struct VerifierApiClient {
    http: reqwest::Client,
    base_url: String,
    auth_token: String,
}

impl VerifierApiClient {
    pub fn new(settings: VerifierApiSettings) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: settings.base_url.trim_end_matches('/').to_string(),
            auth_token: settings.auth_token,
        }
    }

    pub async fn create_qrcode_data(
        &self,
        request: &QrCodeDataRequest,
    ) -> Result<QrCodeDataResponse, AppError> {
        let url = format!("{}/api/oidvp/qrcode", self.base_url);

        Ok(self
            .http
            .get(url)
            .headers(self.auth_headers()?)
            .query(request)
            .send()
            .await?
            .error_for_status()?
            .json::<QrCodeDataResponse>()
            .await?)
    }

    fn auth_headers(&self) -> Result<HeaderMap, AppError> {
        let mut headers = HeaderMap::new();
        let auth_token = HeaderValue::from_str(&self.auth_token)
            .map_err(|_| AppError::bad_gateway("invalid verifier api auth token"))?;
        headers.insert("Access-Token", auth_token);
        Ok(headers)
    }
}
