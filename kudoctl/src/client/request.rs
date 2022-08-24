use crate::config::Config;
use reqwest::header;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

// Represent the error returned by the controller when a request fails
#[derive(Deserialize)]
struct ErrorResponse {
    pub error: String,
}

// Error returned by this module when an endpoint returns an error.
#[derive(Debug)]
pub struct ErrStatusCode {
    pub error: String,
    pub status: u16,
}

#[derive(Debug)]
pub enum RequestError {
    ErrStatusCode(ErrStatusCode),
    ReqwestError(reqwest::Error),
    ParseError(url::ParseError),
}

impl std::error::Error for RequestError {}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::ErrStatusCode(err) => {
                write!(
                    f,
                    "Error status code : {}. Message: {}",
                    err.status, err.error
                )
            }
            RequestError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            RequestError::ParseError(err) => write!(f, "Url parse error: {}", err),
        }
    }
}

// Wrapper around reqwest to make requests to the controller.
pub struct Client {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl Client {
    // Create a new client with the given config.
    pub fn new(config: &Config) -> anyhow::Result<Client> {
        let base_url = reqwest::Url::parse(&config.controller_url)?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        Ok(Client {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
            base_url,
        })
    }

    // Send a request to the controller.
    async fn send_request<U: Serialize>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        body: Option<&U>,
    ) -> Result<Response, RequestError> {
        let url = self
            .base_url
            .join(endpoint)
            .map_err(RequestError::ParseError)?;
        let mut request = (*self).client.request(method, url);

        if let Some(body) = body {
            request = request.json(body);
        }

        request.send().await.map_err(RequestError::ReqwestError)
    }

    // Send a request to the controller and deserialize the response.
    //
    // returns a `RequestError` if a non-2xx response is received.
    pub async fn send_json_request<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        body: Option<&U>,
    ) -> Result<T, RequestError> {
        let response = self.send_request(endpoint, method, body).await?;

        // Check if the response is an error.
        if !response.status().is_success() {
            let status = response.status().as_u16();

            // Read the error message from the response body.

            let error_response: ErrorResponse =
                response.json().await.map_err(RequestError::ReqwestError)?;
            return Err(RequestError::ErrStatusCode(ErrStatusCode {
                error: error_response.error,
                status,
            }));
        }

        response
            .json::<T>()
            .await
            .map_err(RequestError::ReqwestError)
    }
}
