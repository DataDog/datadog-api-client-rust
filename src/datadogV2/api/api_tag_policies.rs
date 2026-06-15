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

/// DeleteTagPolicyOptionalParams is a struct for passing parameters to the method [`TagPoliciesAPI::delete_tag_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteTagPolicyOptionalParams {
    /// Whether to permanently delete the policy instead of performing a soft delete. Defaults to `false`.
    pub hard_delete: Option<bool>,
}

impl DeleteTagPolicyOptionalParams {
    /// Whether to permanently delete the policy instead of performing a soft delete. Defaults to `false`.
    pub fn hard_delete(mut self, value: bool) -> Self {
        self.hard_delete = Some(value);
        self
    }
}

/// GetTagPolicyOptionalParams is a struct for passing parameters to the method [`TagPoliciesAPI::get_tag_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetTagPolicyOptionalParams {
    /// Comma-separated list of related resources to include alongside the policy. Currently the only supported value is `score`.
    pub include: Option<crate::datadogV2::model::TagPolicyInclude>,
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds.
    pub ts_start: Option<i64>,
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub ts_end: Option<i64>,
}

impl GetTagPolicyOptionalParams {
    /// Comma-separated list of related resources to include alongside the policy. Currently the only supported value is `score`.
    pub fn include(mut self, value: crate::datadogV2::model::TagPolicyInclude) -> Self {
        self.include = Some(value);
        self
    }
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds.
    pub fn ts_start(mut self, value: i64) -> Self {
        self.ts_start = Some(value);
        self
    }
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub fn ts_end(mut self, value: i64) -> Self {
        self.ts_end = Some(value);
        self
    }
}

/// GetTagPolicyScoreOptionalParams is a struct for passing parameters to the method [`TagPoliciesAPI::get_tag_policy_score`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetTagPolicyScoreOptionalParams {
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds.
    pub ts_start: Option<i64>,
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub ts_end: Option<i64>,
}

impl GetTagPolicyScoreOptionalParams {
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds.
    pub fn ts_start(mut self, value: i64) -> Self {
        self.ts_start = Some(value);
        self
    }
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub fn ts_end(mut self, value: i64) -> Self {
        self.ts_end = Some(value);
        self
    }
}

/// ListTagPoliciesOptionalParams is a struct for passing parameters to the method [`TagPoliciesAPI::list_tag_policies`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListTagPoliciesOptionalParams {
    /// Whether to include policies that are currently disabled. Defaults to `false`.
    pub include_disabled: Option<bool>,
    /// Whether to include policies that have been soft-deleted. Defaults to `false`.
    pub include_deleted: Option<bool>,
    /// Comma-separated list of related resources to include alongside each policy in the response. Currently the only supported value is `score`.
    pub include: Option<crate::datadogV2::model::TagPolicyInclude>,
    /// Restrict the result set to policies whose source matches the given value.
    pub filter_source: Option<crate::datadogV2::model::TagPolicySource>,
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Defaults to a recent window appropriate for the source.
    pub ts_start: Option<i64>,
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub ts_end: Option<i64>,
}

impl ListTagPoliciesOptionalParams {
    /// Whether to include policies that are currently disabled. Defaults to `false`.
    pub fn include_disabled(mut self, value: bool) -> Self {
        self.include_disabled = Some(value);
        self
    }
    /// Whether to include policies that have been soft-deleted. Defaults to `false`.
    pub fn include_deleted(mut self, value: bool) -> Self {
        self.include_deleted = Some(value);
        self
    }
    /// Comma-separated list of related resources to include alongside each policy in the response. Currently the only supported value is `score`.
    pub fn include(mut self, value: crate::datadogV2::model::TagPolicyInclude) -> Self {
        self.include = Some(value);
        self
    }
    /// Restrict the result set to policies whose source matches the given value.
    pub fn filter_source(mut self, value: crate::datadogV2::model::TagPolicySource) -> Self {
        self.filter_source = Some(value);
        self
    }
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Defaults to a recent window appropriate for the source.
    pub fn ts_start(mut self, value: i64) -> Self {
        self.ts_start = Some(value);
        self
    }
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub fn ts_end(mut self, value: i64) -> Self {
        self.ts_end = Some(value);
        self
    }
}

/// CreateTagPolicyError is a struct for typed errors of method [`TagPoliciesAPI::create_tag_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTagPolicyError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTagPolicyError is a struct for typed errors of method [`TagPoliciesAPI::delete_tag_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagPolicyError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTagPolicyError is a struct for typed errors of method [`TagPoliciesAPI::get_tag_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagPolicyError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTagPolicyScoreError is a struct for typed errors of method [`TagPoliciesAPI::get_tag_policy_score`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagPolicyScoreError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagPoliciesError is a struct for typed errors of method [`TagPoliciesAPI::list_tag_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagPoliciesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTagPolicyError is a struct for typed errors of method [`TagPoliciesAPI::update_tag_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTagPolicyError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Tag Policies define rules that govern which tag values are accepted for a given tag key,
/// scoped to a particular telemetry source (such as logs, spans, or metrics). Policies can be
/// `blocking` (data not matching the policy is rejected) or `surfacing` (matching data is
/// highlighted but not blocked). Each policy reports a compliance `score` derived from how
/// much recent telemetry adheres to the policy.
#[derive(Debug, Clone)]
pub struct TagPoliciesAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for TagPoliciesAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl TagPoliciesAPI {
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

    /// Create a new tag policy for the organization. The caller's organization is derived from
    /// the authenticated user; cross-organization creation is not supported. Fields such as
    /// `policy_id`, `version`, and the timestamp/audit fields are assigned by the server.
    pub async fn create_tag_policy(
        &self,
        body: crate::datadogV2::model::TagPolicyCreateRequest,
    ) -> Result<crate::datadogV2::model::TagPolicyResponse, datadog::Error<CreateTagPolicyError>>
    {
        match self.create_tag_policy_with_http_info(body).await {
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

    /// Create a new tag policy for the organization. The caller's organization is derived from
    /// the authenticated user; cross-organization creation is not supported. Fields such as
    /// `policy_id`, `version`, and the timestamp/audit fields are assigned by the server.
    pub async fn create_tag_policy_with_http_info(
        &self,
        body: crate::datadogV2::model::TagPolicyCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagPolicyResponse>,
        datadog::Error<CreateTagPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_tag_policy";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_tag_policy' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag-policies",
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
            match serde_json::from_str::<crate::datadogV2::model::TagPolicyResponse>(&local_content)
            {
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
            let local_entity: Option<CreateTagPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a tag policy. By default the policy is soft-deleted so it can be recovered later
    /// and so that historical score data remains queryable. Pass `hard_delete=true` to remove
    /// the policy permanently.
    pub async fn delete_tag_policy(
        &self,
        policy_id: String,
        params: DeleteTagPolicyOptionalParams,
    ) -> Result<(), datadog::Error<DeleteTagPolicyError>> {
        match self
            .delete_tag_policy_with_http_info(policy_id, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a tag policy. By default the policy is soft-deleted so it can be recovered later
    /// and so that historical score data remains queryable. Pass `hard_delete=true` to remove
    /// the policy permanently.
    pub async fn delete_tag_policy_with_http_info(
        &self,
        policy_id: String,
        params: DeleteTagPolicyOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteTagPolicyError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_tag_policy";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_tag_policy' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let hard_delete = params.hard_delete;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag-policies/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = hard_delete {
            local_req_builder =
                local_req_builder.query(&[("hard_delete", &local_query_param.to_string())]);
        };

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
            let local_entity: Option<DeleteTagPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a single tag policy by ID. Optionally include the policy's current compliance
    /// score via the `include=score` query parameter. Policies belonging to other organizations
    /// cannot be retrieved.
    pub async fn get_tag_policy(
        &self,
        policy_id: String,
        params: GetTagPolicyOptionalParams,
    ) -> Result<crate::datadogV2::model::TagPolicyResponse, datadog::Error<GetTagPolicyError>> {
        match self.get_tag_policy_with_http_info(policy_id, params).await {
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

    /// Retrieve a single tag policy by ID. Optionally include the policy's current compliance
    /// score via the `include=score` query parameter. Policies belonging to other organizations
    /// cannot be retrieved.
    pub async fn get_tag_policy_with_http_info(
        &self,
        policy_id: String,
        params: GetTagPolicyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagPolicyResponse>,
        datadog::Error<GetTagPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_tag_policy";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_tag_policy' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;
        let ts_start = params.ts_start;
        let ts_end = params.ts_end;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag-policies/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = ts_start {
            local_req_builder =
                local_req_builder.query(&[("ts_start", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = ts_end {
            local_req_builder =
                local_req_builder.query(&[("ts_end", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::TagPolicyResponse>(&local_content)
            {
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
            let local_entity: Option<GetTagPolicyError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve the compliance score for a single tag policy. The score is computed over the
    /// requested time window (or a source-appropriate default) and represents the percentage of
    /// telemetry within that window that conforms to the policy. A `null` score indicates that
    /// no relevant telemetry was found.
    pub async fn get_tag_policy_score(
        &self,
        policy_id: String,
        params: GetTagPolicyScoreOptionalParams,
    ) -> Result<
        crate::datadogV2::model::TagPolicyScoreResponse,
        datadog::Error<GetTagPolicyScoreError>,
    > {
        match self
            .get_tag_policy_score_with_http_info(policy_id, params)
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

    /// Retrieve the compliance score for a single tag policy. The score is computed over the
    /// requested time window (or a source-appropriate default) and represents the percentage of
    /// telemetry within that window that conforms to the policy. A `null` score indicates that
    /// no relevant telemetry was found.
    pub async fn get_tag_policy_score_with_http_info(
        &self,
        policy_id: String,
        params: GetTagPolicyScoreOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagPolicyScoreResponse>,
        datadog::Error<GetTagPolicyScoreError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_tag_policy_score";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_tag_policy_score' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let ts_start = params.ts_start;
        let ts_end = params.ts_end;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag-policies/{policy_id}/score",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = ts_start {
            local_req_builder =
                local_req_builder.query(&[("ts_start", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = ts_end {
            local_req_builder =
                local_req_builder.query(&[("ts_end", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::TagPolicyScoreResponse>(
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
            let local_entity: Option<GetTagPolicyScoreError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve all tag policies for the organization. Optionally include disabled or deleted
    /// policies, filter by telemetry source, and include each policy's current compliance score
    /// via the `include=score` query parameter.
    pub async fn list_tag_policies(
        &self,
        params: ListTagPoliciesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::TagPoliciesListResponse,
        datadog::Error<ListTagPoliciesError>,
    > {
        match self.list_tag_policies_with_http_info(params).await {
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

    /// Retrieve all tag policies for the organization. Optionally include disabled or deleted
    /// policies, filter by telemetry source, and include each policy's current compliance score
    /// via the `include=score` query parameter.
    pub async fn list_tag_policies_with_http_info(
        &self,
        params: ListTagPoliciesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagPoliciesListResponse>,
        datadog::Error<ListTagPoliciesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_tag_policies";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_tag_policies' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include_disabled = params.include_disabled;
        let include_deleted = params.include_deleted;
        let include = params.include;
        let filter_source = params.filter_source;
        let ts_start = params.ts_start;
        let ts_end = params.ts_end;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag-policies",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include_disabled {
            local_req_builder =
                local_req_builder.query(&[("include_disabled", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_deleted {
            local_req_builder =
                local_req_builder.query(&[("include_deleted", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_source {
            local_req_builder =
                local_req_builder.query(&[("filter[source]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = ts_start {
            local_req_builder =
                local_req_builder.query(&[("ts_start", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = ts_end {
            local_req_builder =
                local_req_builder.query(&[("ts_end", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::TagPoliciesListResponse>(
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
            let local_entity: Option<ListTagPoliciesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update one or more attributes of an existing tag policy. Only the fields supplied in the
    /// request body are modified; omitted fields retain their current values. The policy's
    /// `source` cannot be changed after creation.
    pub async fn update_tag_policy(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::TagPolicyUpdateRequest,
    ) -> Result<crate::datadogV2::model::TagPolicyResponse, datadog::Error<UpdateTagPolicyError>>
    {
        match self.update_tag_policy_with_http_info(policy_id, body).await {
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

    /// Update one or more attributes of an existing tag policy. Only the fields supplied in the
    /// request body are modified; omitted fields retain their current values. The policy's
    /// `source` cannot be changed after creation.
    pub async fn update_tag_policy_with_http_info(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::TagPolicyUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagPolicyResponse>,
        datadog::Error<UpdateTagPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_tag_policy";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_tag_policy' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag-policies/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::TagPolicyResponse>(&local_content)
            {
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
            let local_entity: Option<UpdateTagPolicyError> =
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
