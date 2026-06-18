// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetDelegatedTokenError is a struct for typed errors of method [`DelegatedTokenAPI::get_delegated_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDelegatedTokenError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Exchange a cloud-provider identity proof or Datadog credential for a short-lived delegated-user JWT
/// via Workload Identity Federation.
#[derive(Debug, Clone)]
pub struct DelegatedTokenAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for DelegatedTokenAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl DelegatedTokenAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
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

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Exchange a Workload Identity Federation (WIF) proof or Datadog credentials for a short-lived access token
    /// scoped to a Datadog user.
    ///
    /// To authenticate with a WIF identity, pass the cloud-provider token in the `Authorization` header using
    /// the `Bearer` or `Delegated` scheme. Datadog resolves the Datadog user from the persona mapping configured
    /// for that cloud identity.
    ///
    /// To obtain a token for the calling user directly, authenticate with standard Datadog API and application keys.
    ///
    /// Use the returned `access_token` as a bearer token in subsequent API calls.
    pub async fn get_delegated_token(
        &self,
    ) -> Result<
        crate::datadogV2::model::DelegatedTokenResponse,
        datadog::Error<GetDelegatedTokenError>,
    > {
        match self.get_delegated_token_with_http_info().await {
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

    /// Exchange a Workload Identity Federation (WIF) proof or Datadog credentials for a short-lived access token
    /// scoped to a Datadog user.
    ///
    /// To authenticate with a WIF identity, pass the cloud-provider token in the `Authorization` header using
    /// the `Bearer` or `Delegated` scheme. Datadog resolves the Datadog user from the persona mapping configured
    /// for that cloud identity.
    ///
    /// To obtain a token for the calling user directly, authenticate with standard Datadog API and application keys.
    ///
    /// Use the returned `access_token` as a bearer token in subsequent API calls.
    pub async fn get_delegated_token_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::DelegatedTokenResponse>,
        datadog::Error<GetDelegatedTokenError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_delegated_token";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/delegated-token",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::DelegatedTokenResponse>(
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
            let local_entity: Option<GetDelegatedTokenError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
