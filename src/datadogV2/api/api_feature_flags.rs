// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListFeatureFlagsOptionalParams is a struct for passing parameters to the method [`FeatureFlagsAPI::list_feature_flags`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFeatureFlagsOptionalParams {
    /// Filter feature flags by key (partial matching).
    pub key: Option<String>,
    /// Filter by archived status.
    pub is_archived: Option<bool>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
    /// Number of results to skip.
    pub offset: Option<i32>,
}

impl ListFeatureFlagsOptionalParams {
    /// Filter feature flags by key (partial matching).
    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }
    /// Filter by archived status.
    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }
    /// Maximum number of results to return.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
    /// Number of results to skip.
    pub fn offset(mut self, value: i32) -> Self {
        self.offset = Some(value);
        self
    }
}

/// ListFeatureFlagsEnvironmentsOptionalParams is a struct for passing parameters to the method [`FeatureFlagsAPI::list_feature_flags_environments`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFeatureFlagsEnvironmentsOptionalParams {
    /// Filter environments by name (partial matching).
    pub name: Option<String>,
    /// Filter environments by key (partial matching).
    pub key: Option<String>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
    /// Number of results to skip.
    pub offset: Option<i32>,
}

impl ListFeatureFlagsEnvironmentsOptionalParams {
    /// Filter environments by name (partial matching).
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Filter environments by key (partial matching).
    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }
    /// Maximum number of results to return.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
    /// Number of results to skip.
    pub fn offset(mut self, value: i32) -> Self {
        self.offset = Some(value);
        self
    }
}

/// ArchiveFeatureFlagError is a struct for typed errors of method [`FeatureFlagsAPI::archive_feature_flag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveFeatureFlagError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFeatureFlagError is a struct for typed errors of method [`FeatureFlagsAPI::create_feature_flag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFeatureFlagError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFeatureFlagsEnvironmentError is a struct for typed errors of method [`FeatureFlagsAPI::create_feature_flags_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFeatureFlagsEnvironmentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteFeatureFlagsEnvironmentError is a struct for typed errors of method [`FeatureFlagsAPI::delete_feature_flags_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFeatureFlagsEnvironmentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DisableFeatureFlagEnvironmentError is a struct for typed errors of method [`FeatureFlagsAPI::disable_feature_flag_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableFeatureFlagEnvironmentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EnableFeatureFlagEnvironmentError is a struct for typed errors of method [`FeatureFlagsAPI::enable_feature_flag_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnableFeatureFlagEnvironmentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFeatureFlagError is a struct for typed errors of method [`FeatureFlagsAPI::get_feature_flag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFeatureFlagError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFeatureFlagsEnvironmentError is a struct for typed errors of method [`FeatureFlagsAPI::get_feature_flags_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFeatureFlagsEnvironmentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFeatureFlagsError is a struct for typed errors of method [`FeatureFlagsAPI::list_feature_flags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFeatureFlagsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFeatureFlagsEnvironmentsError is a struct for typed errors of method [`FeatureFlagsAPI::list_feature_flags_environments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFeatureFlagsEnvironmentsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnarchiveFeatureFlagError is a struct for typed errors of method [`FeatureFlagsAPI::unarchive_feature_flag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnarchiveFeatureFlagError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFeatureFlagError is a struct for typed errors of method [`FeatureFlagsAPI::update_feature_flag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFeatureFlagError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFeatureFlagsEnvironmentError is a struct for typed errors of method [`FeatureFlagsAPI::update_feature_flags_environment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFeatureFlagsEnvironmentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage feature flags and environments.
#[derive(Debug, Clone)]
pub struct FeatureFlagsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for FeatureFlagsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl FeatureFlagsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        #[allow(unused_mut)]

        let mut reqwest_client_builder = reqwest::Client::builder();

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        #[allow(unused_mut)]


        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        #[cfg(feature = "retry")]
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

    /// Archives a feature flag. Archived flags are
    /// hidden from the main list but remain accessible and can be unarchived.
    pub async fn archive_feature_flag(
        &self,
        feature_flag_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::FeatureFlagResponse, datadog::Error<ArchiveFeatureFlagError>>
    {
        match self
            .archive_feature_flag_with_http_info(feature_flag_id)
            .await
        {
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

    /// Archives a feature flag. Archived flags are
    /// hidden from the main list but remain accessible and can be unarchived.
    pub async fn archive_feature_flag_with_http_info(
        &self,
        feature_flag_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FeatureFlagResponse>,
        datadog::Error<ArchiveFeatureFlagError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.archive_feature_flag";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/{feature_flag_id}/archive",
            local_configuration.get_operation_host(operation_id),
            feature_flag_id = datadog::urlencode(feature_flag_id.to_string())
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
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            match serde_json::from_str::<crate::datadogV2::model::FeatureFlagResponse>(
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
            let local_entity: Option<ArchiveFeatureFlagError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Creates a new feature flag with variants.
    pub async fn create_feature_flag(
        &self,
        body: crate::datadogV2::model::CreateFeatureFlagRequest,
    ) -> Result<crate::datadogV2::model::FeatureFlagResponse, datadog::Error<CreateFeatureFlagError>>
    {
        match self.create_feature_flag_with_http_info(body).await {
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

    /// Creates a new feature flag with variants.
    pub async fn create_feature_flag_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateFeatureFlagRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FeatureFlagResponse>,
        datadog::Error<CreateFeatureFlagError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_feature_flag";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::FeatureFlagResponse>(
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
            let local_entity: Option<CreateFeatureFlagError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Creates a new environment for organizing feature flags.
    pub async fn create_feature_flags_environment(
        &self,
        body: crate::datadogV2::model::CreateEnvironmentRequest,
    ) -> Result<
        crate::datadogV2::model::EnvironmentResponse,
        datadog::Error<CreateFeatureFlagsEnvironmentError>,
    > {
        match self
            .create_feature_flags_environment_with_http_info(body)
            .await
        {
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

    /// Creates a new environment for organizing feature flags.
    pub async fn create_feature_flags_environment_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateEnvironmentRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::EnvironmentResponse>,
        datadog::Error<CreateFeatureFlagsEnvironmentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_feature_flags_environment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/environments",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::EnvironmentResponse>(
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
            let local_entity: Option<CreateFeatureFlagsEnvironmentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Deletes an environment. This operation cannot be undone.
    pub async fn delete_feature_flags_environment(
        &self,
        environment_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteFeatureFlagsEnvironmentError>> {
        match self
            .delete_feature_flags_environment_with_http_info(environment_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Deletes an environment. This operation cannot be undone.
    pub async fn delete_feature_flags_environment_with_http_info(
        &self,
        environment_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteFeatureFlagsEnvironmentError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_feature_flags_environment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/environments/{environment_id}",
            local_configuration.get_operation_host(operation_id),
            environment_id = datadog::urlencode(environment_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteFeatureFlagsEnvironmentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Disable a feature flag in a specific environment.
    pub async fn disable_feature_flag_environment(
        &self,
        feature_flag_id: uuid::Uuid,
        environment_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DisableFeatureFlagEnvironmentError>> {
        match self
            .disable_feature_flag_environment_with_http_info(feature_flag_id, environment_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Disable a feature flag in a specific environment.
    pub async fn disable_feature_flag_environment_with_http_info(
        &self,
        feature_flag_id: uuid::Uuid,
        environment_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DisableFeatureFlagEnvironmentError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.disable_feature_flag_environment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/{feature_flag_id}/environments/{environment_id}/disable",
            local_configuration.get_operation_host(operation_id),
            feature_flag_id = datadog::urlencode(feature_flag_id.to_string()),
            environment_id = datadog::urlencode(environment_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DisableFeatureFlagEnvironmentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Enable a feature flag in a specific environment.
    pub async fn enable_feature_flag_environment(
        &self,
        feature_flag_id: uuid::Uuid,
        environment_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<EnableFeatureFlagEnvironmentError>> {
        match self
            .enable_feature_flag_environment_with_http_info(feature_flag_id, environment_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Enable a feature flag in a specific environment.
    pub async fn enable_feature_flag_environment_with_http_info(
        &self,
        feature_flag_id: uuid::Uuid,
        environment_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<EnableFeatureFlagEnvironmentError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.enable_feature_flag_environment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/{feature_flag_id}/environments/{environment_id}/enable",
            local_configuration.get_operation_host(operation_id),
            feature_flag_id = datadog::urlencode(feature_flag_id.to_string()),
            environment_id = datadog::urlencode(environment_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<EnableFeatureFlagEnvironmentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the details of a specific feature flag
    /// including variants and environment status.
    pub async fn get_feature_flag(
        &self,
        feature_flag_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::FeatureFlagResponse, datadog::Error<GetFeatureFlagError>>
    {
        match self.get_feature_flag_with_http_info(feature_flag_id).await {
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

    /// Returns the details of a specific feature flag
    /// including variants and environment status.
    pub async fn get_feature_flag_with_http_info(
        &self,
        feature_flag_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FeatureFlagResponse>,
        datadog::Error<GetFeatureFlagError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_feature_flag";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/{feature_flag_id}",
            local_configuration.get_operation_host(operation_id),
            feature_flag_id = datadog::urlencode(feature_flag_id.to_string())
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            match serde_json::from_str::<crate::datadogV2::model::FeatureFlagResponse>(
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
            let local_entity: Option<GetFeatureFlagError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the details of a specific environment.
    pub async fn get_feature_flags_environment(
        &self,
        environment_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::EnvironmentResponse,
        datadog::Error<GetFeatureFlagsEnvironmentError>,
    > {
        match self
            .get_feature_flags_environment_with_http_info(environment_id)
            .await
        {
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

    /// Returns the details of a specific environment.
    pub async fn get_feature_flags_environment_with_http_info(
        &self,
        environment_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::EnvironmentResponse>,
        datadog::Error<GetFeatureFlagsEnvironmentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_feature_flags_environment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/environments/{environment_id}",
            local_configuration.get_operation_host(operation_id),
            environment_id = datadog::urlencode(environment_id.to_string())
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            match serde_json::from_str::<crate::datadogV2::model::EnvironmentResponse>(
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
            let local_entity: Option<GetFeatureFlagsEnvironmentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns a list of feature flags for the organization.
    /// Supports filtering by key and archived status.
    pub async fn list_feature_flags(
        &self,
        params: ListFeatureFlagsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListFeatureFlagsResponse,
        datadog::Error<ListFeatureFlagsError>,
    > {
        match self.list_feature_flags_with_http_info(params).await {
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

    /// Returns a list of feature flags for the organization.
    /// Supports filtering by key and archived status.
    pub async fn list_feature_flags_with_http_info(
        &self,
        params: ListFeatureFlagsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListFeatureFlagsResponse>,
        datadog::Error<ListFeatureFlagsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_feature_flags";

        // unbox and build optional parameters
        let key = params.key;
        let is_archived = params.is_archived;
        let limit = params.limit;
        let offset = params.offset;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = key {
            local_req_builder = local_req_builder.query(&[("key", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = is_archived {
            local_req_builder =
                local_req_builder.query(&[("is_archived", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
        };

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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            match serde_json::from_str::<crate::datadogV2::model::ListFeatureFlagsResponse>(
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
            let local_entity: Option<ListFeatureFlagsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns a list of environments for the organization.
    /// Supports filtering by name and key.
    pub async fn list_feature_flags_environments(
        &self,
        params: ListFeatureFlagsEnvironmentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListEnvironmentsResponse,
        datadog::Error<ListFeatureFlagsEnvironmentsError>,
    > {
        match self
            .list_feature_flags_environments_with_http_info(params)
            .await
        {
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

    /// Returns a list of environments for the organization.
    /// Supports filtering by name and key.
    pub async fn list_feature_flags_environments_with_http_info(
        &self,
        params: ListFeatureFlagsEnvironmentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListEnvironmentsResponse>,
        datadog::Error<ListFeatureFlagsEnvironmentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_feature_flags_environments";

        // unbox and build optional parameters
        let name = params.name;
        let key = params.key;
        let limit = params.limit;
        let offset = params.offset;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/environments",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = name {
            local_req_builder =
                local_req_builder.query(&[("name", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = key {
            local_req_builder = local_req_builder.query(&[("key", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
        };

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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            match serde_json::from_str::<crate::datadogV2::model::ListEnvironmentsResponse>(
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
            let local_entity: Option<ListFeatureFlagsEnvironmentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Unarchives a previously archived feature flag,
    /// making it visible in the main list again.
    pub async fn unarchive_feature_flag(
        &self,
        feature_flag_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::FeatureFlagResponse,
        datadog::Error<UnarchiveFeatureFlagError>,
    > {
        match self
            .unarchive_feature_flag_with_http_info(feature_flag_id)
            .await
        {
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

    /// Unarchives a previously archived feature flag,
    /// making it visible in the main list again.
    pub async fn unarchive_feature_flag_with_http_info(
        &self,
        feature_flag_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FeatureFlagResponse>,
        datadog::Error<UnarchiveFeatureFlagError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.unarchive_feature_flag";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/{feature_flag_id}/unarchive",
            local_configuration.get_operation_host(operation_id),
            feature_flag_id = datadog::urlencode(feature_flag_id.to_string())
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
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
            match serde_json::from_str::<crate::datadogV2::model::FeatureFlagResponse>(
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
            let local_entity: Option<UnarchiveFeatureFlagError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates an existing feature flag's metadata such as
    ///  name and description. Does not modify targeting rules or allocations.
    pub async fn update_feature_flag(
        &self,
        feature_flag_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateFeatureFlagRequest,
    ) -> Result<crate::datadogV2::model::FeatureFlagResponse, datadog::Error<UpdateFeatureFlagError>>
    {
        match self
            .update_feature_flag_with_http_info(feature_flag_id, body)
            .await
        {
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

    /// Updates an existing feature flag's metadata such as
    ///  name and description. Does not modify targeting rules or allocations.
    pub async fn update_feature_flag_with_http_info(
        &self,
        feature_flag_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateFeatureFlagRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FeatureFlagResponse>,
        datadog::Error<UpdateFeatureFlagError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_feature_flag";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/{feature_flag_id}",
            local_configuration.get_operation_host(operation_id),
            feature_flag_id = datadog::urlencode(feature_flag_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::FeatureFlagResponse>(
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
            let local_entity: Option<UpdateFeatureFlagError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates an existing environment's metadata such as
    ///  name and description.
    pub async fn update_feature_flags_environment(
        &self,
        environment_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateEnvironmentRequest,
    ) -> Result<
        crate::datadogV2::model::EnvironmentResponse,
        datadog::Error<UpdateFeatureFlagsEnvironmentError>,
    > {
        match self
            .update_feature_flags_environment_with_http_info(environment_id, body)
            .await
        {
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

    /// Updates an existing environment's metadata such as
    ///  name and description.
    pub async fn update_feature_flags_environment_with_http_info(
        &self,
        environment_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateEnvironmentRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::EnvironmentResponse>,
        datadog::Error<UpdateFeatureFlagsEnvironmentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_feature_flags_environment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/feature-flags/environments/{environment_id}",
            local_configuration.get_operation_host(operation_id),
            environment_id = datadog::urlencode(environment_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::EnvironmentResponse>(
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
            let local_entity: Option<UpdateFeatureFlagsEnvironmentError> =
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
