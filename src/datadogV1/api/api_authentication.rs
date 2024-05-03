// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ValidateError is a struct for typed errors of method [`AuthenticationAPI::validate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// All requests to Datadog’s API must be authenticated.
/// Requests that write data require reporting access and require an `API key`.
/// Requests that read data require full access and also require an `application key`.
///
/// **Note:** All Datadog API clients are configured by default to consume Datadog US site APIs.
/// If you are on the Datadog EU site, set the environment variable `DATADOG_HOST` to
/// `<https://api.datadoghq.eu`> or override this value directly when creating your client.
///
/// [Manage your account’s API and application keys](<https://app.datadoghq.com/organization-settings/>) in Datadog, and see the [API and Application Keys page](<https://docs.datadoghq.com/account_management/api-app-keys/>) in the documentation.
#[derive(Debug, Clone)]
pub struct AuthenticationAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for AuthenticationAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl AuthenticationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
            struct RetryableStatus;
            impl reqwest_retry::RetryableStrategy for RetryableStatus {
                fn handle(
                    &self,
                    res: &Result<reqwest::Response, reqwest_middleware::Error>,
                ) -> Option<reqwest_retry::Retryable> {
                    match res {
                        Ok(success) => reqwest_retry::default_on_request_success(success),
                        Err(_) => None,
                    }
                }
            }
            let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_max_retries(config.max_retries);

            let retry_middleware =
                reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                    backoff_policy,
                    RetryableStatus,
                );

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Check if the API key (not the APP key) is valid. If invalid, a 403 is returned.
    pub async fn validate(
        &self,
    ) -> Result<
        crate::datadogV1::model::AuthenticationValidationResponse,
        datadog::Error<ValidateError>,
    > {
        match self.validate_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Check if the API key (not the APP key) is valid. If invalid, a 403 is returned.
    pub async fn validate_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::AuthenticationValidationResponse>,
        datadog::Error<ValidateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.validate";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/validate",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::AuthenticationValidationResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ValidateError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
