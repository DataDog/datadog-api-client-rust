// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListTeamsOwnershipMappingsOptionalParams is a struct for passing parameters to the method [`RumTeamsOwnershipAPI::list_teams_ownership_mappings`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListTeamsOwnershipMappingsOptionalParams {
    /// Filter mappings by RUM view name.
    pub filter_view_name: Option<Vec<String>>,
    /// Filter mappings by owning team handle.
    pub filter_team_handle: Option<Vec<String>>,
    /// Filter mappings by RUM application ID. Each value must be a valid UUID.
    pub filter_application_id: Option<Vec<uuid::Uuid>>,
    /// Filter mappings by RUM application service name.
    pub filter_service: Option<Vec<String>>,
}

impl ListTeamsOwnershipMappingsOptionalParams {
    /// Filter mappings by RUM view name.
    pub fn filter_view_name(mut self, value: Vec<String>) -> Self {
        self.filter_view_name = Some(value);
        self
    }
    /// Filter mappings by owning team handle.
    pub fn filter_team_handle(mut self, value: Vec<String>) -> Self {
        self.filter_team_handle = Some(value);
        self
    }
    /// Filter mappings by RUM application ID. Each value must be a valid UUID.
    pub fn filter_application_id(mut self, value: Vec<uuid::Uuid>) -> Self {
        self.filter_application_id = Some(value);
        self
    }
    /// Filter mappings by RUM application service name.
    pub fn filter_service(mut self, value: Vec<String>) -> Self {
        self.filter_service = Some(value);
        self
    }
}

/// ListTeamsOwnershipRulesOptionalParams is a struct for passing parameters to the method [`RumTeamsOwnershipAPI::list_teams_ownership_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListTeamsOwnershipRulesOptionalParams {
    /// Filter mappings by RUM view name.
    pub filter_view_name: Option<Vec<String>>,
    /// Filter mappings by owning team handle.
    pub filter_team_handle: Option<Vec<String>>,
    /// Filter mappings by RUM application ID. Each value must be a valid UUID.
    pub filter_application_id: Option<Vec<uuid::Uuid>>,
    /// Filter mappings by RUM application service name.
    pub filter_service: Option<Vec<String>>,
}

impl ListTeamsOwnershipRulesOptionalParams {
    /// Filter mappings by RUM view name.
    pub fn filter_view_name(mut self, value: Vec<String>) -> Self {
        self.filter_view_name = Some(value);
        self
    }
    /// Filter mappings by owning team handle.
    pub fn filter_team_handle(mut self, value: Vec<String>) -> Self {
        self.filter_team_handle = Some(value);
        self
    }
    /// Filter mappings by RUM application ID. Each value must be a valid UUID.
    pub fn filter_application_id(mut self, value: Vec<uuid::Uuid>) -> Self {
        self.filter_application_id = Some(value);
        self
    }
    /// Filter mappings by RUM application service name.
    pub fn filter_service(mut self, value: Vec<String>) -> Self {
        self.filter_service = Some(value);
        self
    }
}

/// CreateTeamsOwnershipMappingError is a struct for typed errors of method [`RumTeamsOwnershipAPI::create_teams_ownership_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamsOwnershipMappingError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateTeamsOwnershipMappingsBatchError is a struct for typed errors of method [`RumTeamsOwnershipAPI::create_teams_ownership_mappings_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamsOwnershipMappingsBatchError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTeamsOwnershipMappingError is a struct for typed errors of method [`RumTeamsOwnershipAPI::delete_teams_ownership_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamsOwnershipMappingError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTeamsOwnershipMappingError is a struct for typed errors of method [`RumTeamsOwnershipAPI::get_teams_ownership_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamsOwnershipMappingError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTeamsOwnershipMappingsError is a struct for typed errors of method [`RumTeamsOwnershipAPI::list_teams_ownership_mappings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTeamsOwnershipMappingsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTeamsOwnershipRulesError is a struct for typed errors of method [`RumTeamsOwnershipAPI::list_teams_ownership_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTeamsOwnershipRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage teams ownership mappings between RUM views and the teams that own them.
/// See <<https://docs.datadoghq.com/real_user_monitoring/ownership_of_views/>.>
#[derive(Debug, Clone)]
pub struct RumTeamsOwnershipAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RumTeamsOwnershipAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RumTeamsOwnershipAPI {
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

    /// Create a teams ownership mapping for your organization.
    /// Returns the teams ownership mapping object from the request body when the request is successful.
    pub async fn create_teams_ownership_mapping(
        &self,
        body: crate::datadogV2::model::TeamsOwnershipMappingCreateRequest,
    ) -> Result<
        crate::datadogV2::model::TeamsOwnershipMappingResponse,
        datadog::Error<CreateTeamsOwnershipMappingError>,
    > {
        match self
            .create_teams_ownership_mapping_with_http_info(body)
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

    /// Create a teams ownership mapping for your organization.
    /// Returns the teams ownership mapping object from the request body when the request is successful.
    pub async fn create_teams_ownership_mapping_with_http_info(
        &self,
        body: crate::datadogV2::model::TeamsOwnershipMappingCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TeamsOwnershipMappingResponse>,
        datadog::Error<CreateTeamsOwnershipMappingError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_teams_ownership_mapping";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_teams_ownership_mapping' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/config/teams-ownership/mappings",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamsOwnershipMappingResponse>(
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
            let local_entity: Option<CreateTeamsOwnershipMappingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Add and remove teams ownership mappings for your organization in a single atomic request, following
    /// the JSON:API [atomic operations extension](<https://jsonapi.org/ext/atomic/>).
    /// Operations are applied together: if any operation is invalid, none of the operations are applied.
    /// Add operations are processed before remove operations, so results may not appear in the same
    /// order as the request.
    pub async fn create_teams_ownership_mappings_batch(
        &self,
        body: crate::datadogV2::model::TeamsOwnershipMappingBatchRequest,
    ) -> Result<
        crate::datadogV2::model::TeamsOwnershipMappingBatchResponse,
        datadog::Error<CreateTeamsOwnershipMappingsBatchError>,
    > {
        match self
            .create_teams_ownership_mappings_batch_with_http_info(body)
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

    /// Add and remove teams ownership mappings for your organization in a single atomic request, following
    /// the JSON:API [atomic operations extension](<https://jsonapi.org/ext/atomic/>).
    /// Operations are applied together: if any operation is invalid, none of the operations are applied.
    /// Add operations are processed before remove operations, so results may not appear in the same
    /// order as the request.
    pub async fn create_teams_ownership_mappings_batch_with_http_info(
        &self,
        body: crate::datadogV2::model::TeamsOwnershipMappingBatchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TeamsOwnershipMappingBatchResponse>,
        datadog::Error<CreateTeamsOwnershipMappingsBatchError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_teams_ownership_mappings_batch";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_teams_ownership_mappings_batch' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/config/teams-ownership/mappings/operations",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamsOwnershipMappingBatchResponse>(
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
            let local_entity: Option<CreateTeamsOwnershipMappingsBatchError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a specific teams ownership mapping from your organization.
    pub async fn delete_teams_ownership_mapping(
        &self,
        id: String,
    ) -> Result<(), datadog::Error<DeleteTeamsOwnershipMappingError>> {
        match self.delete_teams_ownership_mapping_with_http_info(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific teams ownership mapping from your organization.
    pub async fn delete_teams_ownership_mapping_with_http_info(
        &self,
        id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteTeamsOwnershipMappingError>>
    {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_teams_ownership_mapping";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_teams_ownership_mapping' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/config/teams-ownership/mappings/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id)
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
            let local_entity: Option<DeleteTeamsOwnershipMappingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a specific teams ownership mapping from your organization.
    pub async fn get_teams_ownership_mapping(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::TeamsOwnershipMappingResponse,
        datadog::Error<GetTeamsOwnershipMappingError>,
    > {
        match self.get_teams_ownership_mapping_with_http_info(id).await {
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

    /// Get a specific teams ownership mapping from your organization.
    pub async fn get_teams_ownership_mapping_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TeamsOwnershipMappingResponse>,
        datadog::Error<GetTeamsOwnershipMappingError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_teams_ownership_mapping";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_teams_ownership_mapping' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/config/teams-ownership/mappings/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamsOwnershipMappingResponse>(
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
            let local_entity: Option<GetTeamsOwnershipMappingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of teams ownership mappings for your organization, optionally filtered.
    pub async fn list_teams_ownership_mappings(
        &self,
        params: ListTeamsOwnershipMappingsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::TeamsOwnershipMappingsResponse,
        datadog::Error<ListTeamsOwnershipMappingsError>,
    > {
        match self
            .list_teams_ownership_mappings_with_http_info(params)
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

    /// Get the list of teams ownership mappings for your organization, optionally filtered.
    pub async fn list_teams_ownership_mappings_with_http_info(
        &self,
        params: ListTeamsOwnershipMappingsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TeamsOwnershipMappingsResponse>,
        datadog::Error<ListTeamsOwnershipMappingsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_teams_ownership_mappings";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_teams_ownership_mappings' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_view_name = params.filter_view_name;
        let filter_team_handle = params.filter_team_handle;
        let filter_application_id = params.filter_application_id;
        let filter_service = params.filter_service;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/config/teams-ownership/mappings",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = filter_view_name {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[view_name]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_team_handle {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[team_handle]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_application_id {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[application_id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_service {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[service]", &param.to_string())]);
            }
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
            match serde_json::from_str::<crate::datadogV2::model::TeamsOwnershipMappingsResponse>(
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
            let local_entity: Option<ListTeamsOwnershipMappingsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of teams ownership rules for your organization, optionally filtered.
    /// Rules group the underlying mappings by `view_name`, `application_id`, `service`, and `match_type`,
    /// collapsing every team that owns the same view into a single entry.
    pub async fn list_teams_ownership_rules(
        &self,
        params: ListTeamsOwnershipRulesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::TeamsOwnershipRulesResponse,
        datadog::Error<ListTeamsOwnershipRulesError>,
    > {
        match self.list_teams_ownership_rules_with_http_info(params).await {
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

    /// Get the list of teams ownership rules for your organization, optionally filtered.
    /// Rules group the underlying mappings by `view_name`, `application_id`, `service`, and `match_type`,
    /// collapsing every team that owns the same view into a single entry.
    pub async fn list_teams_ownership_rules_with_http_info(
        &self,
        params: ListTeamsOwnershipRulesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TeamsOwnershipRulesResponse>,
        datadog::Error<ListTeamsOwnershipRulesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_teams_ownership_rules";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_teams_ownership_rules' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_view_name = params.filter_view_name;
        let filter_team_handle = params.filter_team_handle;
        let filter_application_id = params.filter_application_id;
        let filter_service = params.filter_service;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/config/teams-ownership/rules",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = filter_view_name {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[view_name]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_team_handle {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[team_handle]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_application_id {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[application_id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_service {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[service]", &param.to_string())]);
            }
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
            match serde_json::from_str::<crate::datadogV2::model::TeamsOwnershipRulesResponse>(
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
            let local_entity: Option<ListTeamsOwnershipRulesError> =
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
