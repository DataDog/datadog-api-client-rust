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

/// DeleteTagRuleOptionalParams is a struct for passing parameters to the method [`TagRulesAPI::delete_tag_rule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteTagRuleOptionalParams {
    /// Whether to permanently delete the rule instead of performing a soft delete. Defaults to `false`.
    pub hard_delete: Option<bool>,
}

impl DeleteTagRuleOptionalParams {
    /// Whether to permanently delete the rule instead of performing a soft delete. Defaults to `false`.
    pub fn hard_delete(mut self, value: bool) -> Self {
        self.hard_delete = Some(value);
        self
    }
}

/// GetTagRuleOptionalParams is a struct for passing parameters to the method [`TagRulesAPI::get_tag_rule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetTagRuleOptionalParams {
    /// Comma-separated list of related resources to include alongside the rule. Currently the only supported value is `score`.
    pub include: Option<crate::datadogV2::model::TagRuleInclude>,
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds.
    pub ts_start: Option<i64>,
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub ts_end: Option<i64>,
}

impl GetTagRuleOptionalParams {
    /// Comma-separated list of related resources to include alongside the rule. Currently the only supported value is `score`.
    pub fn include(mut self, value: crate::datadogV2::model::TagRuleInclude) -> Self {
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

/// GetTagRuleScoreOptionalParams is a struct for passing parameters to the method [`TagRulesAPI::get_tag_rule_score`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetTagRuleScoreOptionalParams {
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds.
    pub ts_start: Option<i64>,
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub ts_end: Option<i64>,
}

impl GetTagRuleScoreOptionalParams {
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

/// ListTagRulesOptionalParams is a struct for passing parameters to the method [`TagRulesAPI::list_tag_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListTagRulesOptionalParams {
    /// Whether to include rules that are currently disabled. Defaults to `false`.
    pub include_disabled: Option<bool>,
    /// Whether to include rules that have been soft-deleted. Defaults to `false`.
    pub include_deleted: Option<bool>,
    /// Comma-separated list of related resources to include alongside each rule in the response. Currently the only supported value is `score`.
    pub include: Option<crate::datadogV2::model::TagRuleInclude>,
    /// Restrict the result set to rules whose source matches the given value.
    pub filter_source: Option<crate::datadogV2::model::TagRuleSource>,
    /// Start of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Defaults to a recent window appropriate for the source.
    pub ts_start: Option<i64>,
    /// End of the time window used for compliance score computation, as a Unix timestamp in milliseconds. Must be in the past and greater than `ts_start`.
    pub ts_end: Option<i64>,
}

impl ListTagRulesOptionalParams {
    /// Whether to include rules that are currently disabled. Defaults to `false`.
    pub fn include_disabled(mut self, value: bool) -> Self {
        self.include_disabled = Some(value);
        self
    }
    /// Whether to include rules that have been soft-deleted. Defaults to `false`.
    pub fn include_deleted(mut self, value: bool) -> Self {
        self.include_deleted = Some(value);
        self
    }
    /// Comma-separated list of related resources to include alongside each rule in the response. Currently the only supported value is `score`.
    pub fn include(mut self, value: crate::datadogV2::model::TagRuleInclude) -> Self {
        self.include = Some(value);
        self
    }
    /// Restrict the result set to rules whose source matches the given value.
    pub fn filter_source(mut self, value: crate::datadogV2::model::TagRuleSource) -> Self {
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

/// CreateTagRuleError is a struct for typed errors of method [`TagRulesAPI::create_tag_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTagRuleError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTagRuleError is a struct for typed errors of method [`TagRulesAPI::delete_tag_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagRuleError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTagRuleError is a struct for typed errors of method [`TagRulesAPI::get_tag_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagRuleError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTagRuleScoreError is a struct for typed errors of method [`TagRulesAPI::get_tag_rule_score`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagRuleScoreError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagRulesError is a struct for typed errors of method [`TagRulesAPI::list_tag_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagRulesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTagRuleError is a struct for typed errors of method [`TagRulesAPI::update_tag_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTagRuleError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Tag Rules define rules that govern which tag values are accepted for a given tag key,
/// scoped to a particular telemetry source (such as logs, spans, or metrics). Rules can be
/// `blocking` (data not matching the rule is rejected) or `surfacing` (matching data is
/// highlighted but not blocked). Each rule reports a compliance `score` derived from how
/// much recent telemetry adheres to the rule.
#[derive(Debug, Clone)]
pub struct TagRulesAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for TagRulesAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl TagRulesAPI {
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

    /// Create a new tag rule for the organization. The caller's organization is derived from
    /// the authenticated user; cross-organization creation is not supported. Fields such as
    /// `policy_id`, `version`, and the timestamp/audit fields are assigned by the server.
    pub async fn create_tag_rule(
        &self,
        body: crate::datadogV2::model::TagRuleCreateRequest,
    ) -> Result<crate::datadogV2::model::TagRuleResponse, datadog::Error<CreateTagRuleError>> {
        match self.create_tag_rule_with_http_info(body).await {
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

    /// Create a new tag rule for the organization. The caller's organization is derived from
    /// the authenticated user; cross-organization creation is not supported. Fields such as
    /// `policy_id`, `version`, and the timestamp/audit fields are assigned by the server.
    pub async fn create_tag_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::TagRuleCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagRuleResponse>,
        datadog::Error<CreateTagRuleError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_tag_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_tag_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag_policies",
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
            match serde_json::from_str::<crate::datadogV2::model::TagRuleResponse>(&local_content) {
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
            let local_entity: Option<CreateTagRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a tag rule. By default the rule is soft-deleted so it can be recovered later
    /// and so that historical score data remains queryable. Pass `hard_delete=true` to remove
    /// the rule permanently.
    pub async fn delete_tag_rule(
        &self,
        policy_id: String,
        params: DeleteTagRuleOptionalParams,
    ) -> Result<(), datadog::Error<DeleteTagRuleError>> {
        match self.delete_tag_rule_with_http_info(policy_id, params).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a tag rule. By default the rule is soft-deleted so it can be recovered later
    /// and so that historical score data remains queryable. Pass `hard_delete=true` to remove
    /// the rule permanently.
    pub async fn delete_tag_rule_with_http_info(
        &self,
        policy_id: String,
        params: DeleteTagRuleOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteTagRuleError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_tag_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_tag_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let hard_delete = params.hard_delete;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag_policies/{policy_id}",
            local_configuration.get_operation_host(local_operation_id),
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
            let local_entity: Option<DeleteTagRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a single tag rule by ID. Optionally include the rule's current compliance
    /// score via the `include=score` query parameter. Rules belonging to other organizations
    /// cannot be retrieved.
    pub async fn get_tag_rule(
        &self,
        policy_id: String,
        params: GetTagRuleOptionalParams,
    ) -> Result<crate::datadogV2::model::TagRuleResponse, datadog::Error<GetTagRuleError>> {
        match self.get_tag_rule_with_http_info(policy_id, params).await {
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

    /// Retrieve a single tag rule by ID. Optionally include the rule's current compliance
    /// score via the `include=score` query parameter. Rules belonging to other organizations
    /// cannot be retrieved.
    pub async fn get_tag_rule_with_http_info(
        &self,
        policy_id: String,
        params: GetTagRuleOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagRuleResponse>,
        datadog::Error<GetTagRuleError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_tag_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_tag_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;
        let ts_start = params.ts_start;
        let ts_end = params.ts_end;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag_policies/{policy_id}",
            local_configuration.get_operation_host(local_operation_id),
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
            match serde_json::from_str::<crate::datadogV2::model::TagRuleResponse>(&local_content) {
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
            let local_entity: Option<GetTagRuleError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve the compliance score for a single tag rule. The score is computed over the
    /// requested time window (or a source-appropriate default) and represents the percentage of
    /// telemetry within that window that conforms to the rule. A `null` score indicates that
    /// no relevant telemetry was found.
    pub async fn get_tag_rule_score(
        &self,
        policy_id: String,
        params: GetTagRuleScoreOptionalParams,
    ) -> Result<crate::datadogV2::model::TagRuleScoreResponse, datadog::Error<GetTagRuleScoreError>>
    {
        match self
            .get_tag_rule_score_with_http_info(policy_id, params)
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

    /// Retrieve the compliance score for a single tag rule. The score is computed over the
    /// requested time window (or a source-appropriate default) and represents the percentage of
    /// telemetry within that window that conforms to the rule. A `null` score indicates that
    /// no relevant telemetry was found.
    pub async fn get_tag_rule_score_with_http_info(
        &self,
        policy_id: String,
        params: GetTagRuleScoreOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagRuleScoreResponse>,
        datadog::Error<GetTagRuleScoreError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_tag_rule_score";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_tag_rule_score' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let ts_start = params.ts_start;
        let ts_end = params.ts_end;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag_policies/{policy_id}/score",
            local_configuration.get_operation_host(local_operation_id),
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
            match serde_json::from_str::<crate::datadogV2::model::TagRuleScoreResponse>(
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
            let local_entity: Option<GetTagRuleScoreError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve all tag rules for the organization. Optionally include disabled or deleted
    /// rules, filter by telemetry source, and include each rule's current compliance score
    /// via the `include=score` query parameter.
    pub async fn list_tag_rules(
        &self,
        params: ListTagRulesOptionalParams,
    ) -> Result<crate::datadogV2::model::TagRulesListResponse, datadog::Error<ListTagRulesError>>
    {
        match self.list_tag_rules_with_http_info(params).await {
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

    /// Retrieve all tag rules for the organization. Optionally include disabled or deleted
    /// rules, filter by telemetry source, and include each rule's current compliance score
    /// via the `include=score` query parameter.
    pub async fn list_tag_rules_with_http_info(
        &self,
        params: ListTagRulesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagRulesListResponse>,
        datadog::Error<ListTagRulesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_tag_rules";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_tag_rules' is not enabled".to_string(),
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
            "{}/api/v2/tag_policies",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TagRulesListResponse>(
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
            let local_entity: Option<ListTagRulesError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update one or more attributes of an existing tag rule. Only the fields supplied in the
    /// request body are modified; omitted fields retain their current values. The rule's
    /// `source` cannot be changed after creation.
    pub async fn update_tag_rule(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::TagRuleUpdateRequest,
    ) -> Result<crate::datadogV2::model::TagRuleResponse, datadog::Error<UpdateTagRuleError>> {
        match self.update_tag_rule_with_http_info(policy_id, body).await {
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

    /// Update one or more attributes of an existing tag rule. Only the fields supplied in the
    /// request body are modified; omitted fields retain their current values. The rule's
    /// `source` cannot be changed after creation.
    pub async fn update_tag_rule_with_http_info(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::TagRuleUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TagRuleResponse>,
        datadog::Error<UpdateTagRuleError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_tag_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_tag_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tag_policies/{policy_id}",
            local_configuration.get_operation_host(local_operation_id),
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
            match serde_json::from_str::<crate::datadogV2::model::TagRuleResponse>(&local_content) {
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
            let local_entity: Option<UpdateTagRuleError> =
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
