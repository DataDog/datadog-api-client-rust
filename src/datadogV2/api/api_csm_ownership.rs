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

/// GetOwnershipEvidenceOptionalParams is a struct for passing parameters to the method [`CSMOwnershipAPI::get_ownership_evidence`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetOwnershipEvidenceOptionalParams {
    /// A previously returned weak `ETag` value. When supplied and the evidence has not changed, the endpoint returns `304 Not Modified`.
    pub if_none_match: Option<String>,
}

impl GetOwnershipEvidenceOptionalParams {
    /// A previously returned weak `ETag` value. When supplied and the evidence has not changed, the endpoint returns `304 Not Modified`.
    pub fn if_none_match(mut self, value: String) -> Self {
        self.if_none_match = Some(value);
        self
    }
}

/// GetOwnershipInferenceOptionalParams is a struct for passing parameters to the method [`CSMOwnershipAPI::get_ownership_inference`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetOwnershipInferenceOptionalParams {
    /// A previously returned `ETag` value. When supplied and the resource has not changed, the endpoint returns `304 Not Modified`.
    pub if_none_match: Option<String>,
}

impl GetOwnershipInferenceOptionalParams {
    /// A previously returned `ETag` value. When supplied and the resource has not changed, the endpoint returns `304 Not Modified`.
    pub fn if_none_match(mut self, value: String) -> Self {
        self.if_none_match = Some(value);
        self
    }
}

/// ListOwnershipHistoryOptionalParams is a struct for passing parameters to the method [`CSMOwnershipAPI::list_ownership_history`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListOwnershipHistoryOptionalParams {
    /// An opaque, base64-encoded cursor token returned by a previous call in `pagination.next_cursor`. Omit to fetch the first page.
    pub cursor: Option<String>,
    /// The maximum number of history entries to return per page.
    pub limit: Option<i32>,
}

impl ListOwnershipHistoryOptionalParams {
    /// An opaque, base64-encoded cursor token returned by a previous call in `pagination.next_cursor`. Omit to fetch the first page.
    pub fn cursor(mut self, value: String) -> Self {
        self.cursor = Some(value);
        self
    }
    /// The maximum number of history entries to return per page.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// ListOwnershipHistoryByOwnerTypeOptionalParams is a struct for passing parameters to the method [`CSMOwnershipAPI::list_ownership_history_by_owner_type`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListOwnershipHistoryByOwnerTypeOptionalParams {
    /// An opaque, base64-encoded cursor token returned by a previous call in `pagination.next_cursor`. Omit to fetch the first page.
    pub cursor: Option<String>,
    /// The maximum number of history entries to return per page.
    pub limit: Option<i32>,
}

impl ListOwnershipHistoryByOwnerTypeOptionalParams {
    /// An opaque, base64-encoded cursor token returned by a previous call in `pagination.next_cursor`. Omit to fetch the first page.
    pub fn cursor(mut self, value: String) -> Self {
        self.cursor = Some(value);
        self
    }
    /// The maximum number of history entries to return per page.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// CreateOwnershipFeedbackError is a struct for typed errors of method [`CSMOwnershipAPI::create_ownership_feedback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOwnershipFeedbackError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    OwnershipInferenceResponse(crate::datadogV2::model::OwnershipInferenceResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOwnershipEvidenceError is a struct for typed errors of method [`CSMOwnershipAPI::get_ownership_evidence`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOwnershipEvidenceError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOwnershipInferenceError is a struct for typed errors of method [`CSMOwnershipAPI::get_ownership_inference`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOwnershipInferenceError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOwnershipSettingsError is a struct for typed errors of method [`CSMOwnershipAPI::get_ownership_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOwnershipSettingsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOwnershipUntaggedFindingsError is a struct for typed errors of method [`CSMOwnershipAPI::get_ownership_untagged_findings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOwnershipUntaggedFindingsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListOwnershipHistoryError is a struct for typed errors of method [`CSMOwnershipAPI::list_ownership_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOwnershipHistoryError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListOwnershipHistoryByOwnerTypeError is a struct for typed errors of method [`CSMOwnershipAPI::list_ownership_history_by_owner_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOwnershipHistoryByOwnerTypeError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListOwnershipInferencesError is a struct for typed errors of method [`CSMOwnershipAPI::list_ownership_inferences`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOwnershipInferencesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PostOwnershipSettingsError is a struct for typed errors of method [`CSMOwnershipAPI::post_ownership_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOwnershipSettingsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Datadog Cloud Security Management (CSM) Ownership infers the most likely owner
/// for a cloud resource by combining ownership signals from across the platform,
/// and lets you review the inference, inspect its evidence, and submit feedback to
/// persist, override, or correct the inferred owner.
/// For more information, see [Cloud Security Management](<https://docs.datadoghq.com/security/cloud_security_management>).
#[derive(Debug, Clone)]
pub struct CSMOwnershipAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CSMOwnershipAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CSMOwnershipAPI {
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

    /// Submit feedback on the current ownership inference for a resource and owner type. Valid actions are `confirm`, `reject`, `correct`, and `persist`.
    ///
    /// The request must include the current inference `checksum` in `inference_checksum`. If the checksum does not match the current inference state, the endpoint returns `409 Conflict`.
    ///
    /// When `action` is `correct`, `corrected_owner_handle` and `corrected_owner_type` are required.
    pub async fn create_ownership_feedback(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        body: crate::datadogV2::model::OwnershipFeedbackRequest,
    ) -> Result<
        crate::datadogV2::model::OwnershipFeedbackResponse,
        datadog::Error<CreateOwnershipFeedbackError>,
    > {
        match self
            .create_ownership_feedback_with_http_info(resource_id, owner_type, body)
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

    /// Submit feedback on the current ownership inference for a resource and owner type. Valid actions are `confirm`, `reject`, `correct`, and `persist`.
    ///
    /// The request must include the current inference `checksum` in `inference_checksum`. If the checksum does not match the current inference state, the endpoint returns `409 Conflict`.
    ///
    /// When `action` is `correct`, `corrected_owner_handle` and `corrected_owner_type` are required.
    pub async fn create_ownership_feedback_with_http_info(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        body: crate::datadogV2::model::OwnershipFeedbackRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipFeedbackResponse>,
        datadog::Error<CreateOwnershipFeedbackError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_ownership_feedback";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ownership_feedback' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/{resource_id}/{owner_type}/feedback",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id),
            owner_type = datadog::urlencode(owner_type.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipFeedbackResponse>(
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
            let local_entity: Option<CreateOwnershipFeedbackError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the evidence versions backing the current ownership inference for a resource and owner type.
    ///
    /// This endpoint supports weak ETag caching. Pass the previously returned `ETag` value in the `If-None-Match` request header to receive a `304 Not Modified` response when the evidence has not changed.
    pub async fn get_ownership_evidence(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        params: GetOwnershipEvidenceOptionalParams,
    ) -> Result<
        crate::datadogV2::model::OwnershipEvidenceResponse,
        datadog::Error<GetOwnershipEvidenceError>,
    > {
        match self
            .get_ownership_evidence_with_http_info(resource_id, owner_type, params)
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

    /// Get the evidence versions backing the current ownership inference for a resource and owner type.
    ///
    /// This endpoint supports weak ETag caching. Pass the previously returned `ETag` value in the `If-None-Match` request header to receive a `304 Not Modified` response when the evidence has not changed.
    pub async fn get_ownership_evidence_with_http_info(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        params: GetOwnershipEvidenceOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipEvidenceResponse>,
        datadog::Error<GetOwnershipEvidenceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_ownership_evidence";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ownership_evidence' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let if_none_match = params.if_none_match;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/{resource_id}/{owner_type}/evidence",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id),
            owner_type = datadog::urlencode(owner_type.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        if let Some(ref local) = if_none_match {
            headers.insert(
                "If-None-Match",
                local
                    .to_string()
                    .parse()
                    .expect("failed to parse If-None-Match header"),
            );
        }

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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipEvidenceResponse>(
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
            let local_entity: Option<GetOwnershipEvidenceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the current ownership inference for a resource for a specific owner type.
    ///
    /// This endpoint supports ETag-based caching. Pass the previously returned `ETag` value in the `If-None-Match` request header to receive a `304 Not Modified` response when the inference has not changed.
    pub async fn get_ownership_inference(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        params: GetOwnershipInferenceOptionalParams,
    ) -> Result<
        crate::datadogV2::model::OwnershipInferenceResponse,
        datadog::Error<GetOwnershipInferenceError>,
    > {
        match self
            .get_ownership_inference_with_http_info(resource_id, owner_type, params)
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

    /// Get the current ownership inference for a resource for a specific owner type.
    ///
    /// This endpoint supports ETag-based caching. Pass the previously returned `ETag` value in the `If-None-Match` request header to receive a `304 Not Modified` response when the inference has not changed.
    pub async fn get_ownership_inference_with_http_info(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        params: GetOwnershipInferenceOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipInferenceResponse>,
        datadog::Error<GetOwnershipInferenceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_ownership_inference";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ownership_inference' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let if_none_match = params.if_none_match;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/{resource_id}/{owner_type}",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id),
            owner_type = datadog::urlencode(owner_type.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        if let Some(ref local) = if_none_match {
            headers.insert(
                "If-None-Match",
                local
                    .to_string()
                    .parse()
                    .expect("failed to parse If-None-Match header"),
            );
        }

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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipInferenceResponse>(
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
            let local_entity: Option<GetOwnershipInferenceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get ownership settings for the org. When settings are unset, the API returns the default opt-out configuration with `auto_tag` set to `true` and `confidence_level` set to `high`.
    pub async fn get_ownership_settings(
        &self,
    ) -> Result<
        crate::datadogV2::model::OwnershipSettingsResponse,
        datadog::Error<GetOwnershipSettingsError>,
    > {
        match self.get_ownership_settings_with_http_info().await {
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

    /// Get ownership settings for the org. When settings are unset, the API returns the default opt-out configuration with `auto_tag` set to `true` and `confidence_level` set to `high`.
    pub async fn get_ownership_settings_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipSettingsResponse>,
        datadog::Error<GetOwnershipSettingsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_ownership_settings";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ownership_settings' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/settings",
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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipSettingsResponse>(
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
            let local_entity: Option<GetOwnershipSettingsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Count findings with no team tag, grouped by ownership confidence level.
    pub async fn get_ownership_untagged_findings(
        &self,
    ) -> Result<
        crate::datadogV2::model::OwnershipUntaggedFindingsResponse,
        datadog::Error<GetOwnershipUntaggedFindingsError>,
    > {
        match self.get_ownership_untagged_findings_with_http_info().await {
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

    /// Count findings with no team tag, grouped by ownership confidence level.
    pub async fn get_ownership_untagged_findings_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipUntaggedFindingsResponse>,
        datadog::Error<GetOwnershipUntaggedFindingsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_ownership_untagged_findings";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ownership_untagged_findings' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/settings/untagged",
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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipUntaggedFindingsResponse>(
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
            let local_entity: Option<GetOwnershipUntaggedFindingsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List inference history entries for a resource across all owner types, ordered from most recent to oldest. Uses cursor-based pagination.
    pub async fn list_ownership_history(
        &self,
        resource_id: String,
        params: ListOwnershipHistoryOptionalParams,
    ) -> Result<
        crate::datadogV2::model::OwnershipHistoryResponse,
        datadog::Error<ListOwnershipHistoryError>,
    > {
        match self
            .list_ownership_history_with_http_info(resource_id, params)
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

    /// List inference history entries for a resource across all owner types, ordered from most recent to oldest. Uses cursor-based pagination.
    pub async fn list_ownership_history_with_http_info(
        &self,
        resource_id: String,
        params: ListOwnershipHistoryOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipHistoryResponse>,
        datadog::Error<ListOwnershipHistoryError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_ownership_history";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ownership_history' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let cursor = params.cursor;
        let limit = params.limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/{resource_id}/history",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = cursor {
            local_req_builder =
                local_req_builder.query(&[("cursor", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipHistoryResponse>(
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
            let local_entity: Option<ListOwnershipHistoryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List inference history entries for a resource filtered by owner type, ordered from most recent to oldest. Uses cursor-based pagination.
    pub async fn list_ownership_history_by_owner_type(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        params: ListOwnershipHistoryByOwnerTypeOptionalParams,
    ) -> Result<
        crate::datadogV2::model::OwnershipHistoryResponse,
        datadog::Error<ListOwnershipHistoryByOwnerTypeError>,
    > {
        match self
            .list_ownership_history_by_owner_type_with_http_info(resource_id, owner_type, params)
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

    /// List inference history entries for a resource filtered by owner type, ordered from most recent to oldest. Uses cursor-based pagination.
    pub async fn list_ownership_history_by_owner_type_with_http_info(
        &self,
        resource_id: String,
        owner_type: crate::datadogV2::model::OwnershipOwnerType,
        params: ListOwnershipHistoryByOwnerTypeOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipHistoryResponse>,
        datadog::Error<ListOwnershipHistoryByOwnerTypeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_ownership_history_by_owner_type";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ownership_history_by_owner_type' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let cursor = params.cursor;
        let limit = params.limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/{resource_id}/{owner_type}/history",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id),
            owner_type = datadog::urlencode(owner_type.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = cursor {
            local_req_builder =
                local_req_builder.query(&[("cursor", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipHistoryResponse>(
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
            let local_entity: Option<ListOwnershipHistoryByOwnerTypeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get all current ownership inferences for a resource, one per owner type (`user`, `team`, `service`, `unknown`).
    pub async fn list_ownership_inferences(
        &self,
        resource_id: String,
    ) -> Result<
        crate::datadogV2::model::OwnershipInferenceListResponse,
        datadog::Error<ListOwnershipInferencesError>,
    > {
        match self
            .list_ownership_inferences_with_http_info(resource_id)
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

    /// Get all current ownership inferences for a resource, one per owner type (`user`, `team`, `service`, `unknown`).
    pub async fn list_ownership_inferences_with_http_info(
        &self,
        resource_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipInferenceListResponse>,
        datadog::Error<ListOwnershipInferencesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_ownership_inferences";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ownership_inferences' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id)
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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipInferenceListResponse>(
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
            let local_entity: Option<ListOwnershipInferencesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update ownership settings for the org.
    pub async fn post_ownership_settings(
        &self,
        body: crate::datadogV2::model::OwnershipSettingsRequest,
    ) -> Result<
        crate::datadogV2::model::OwnershipSettingsResponse,
        datadog::Error<PostOwnershipSettingsError>,
    > {
        match self.post_ownership_settings_with_http_info(body).await {
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

    /// Update ownership settings for the org.
    pub async fn post_ownership_settings_with_http_info(
        &self,
        body: crate::datadogV2::model::OwnershipSettingsRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OwnershipSettingsResponse>,
        datadog::Error<PostOwnershipSettingsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.post_ownership_settings";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.post_ownership_settings' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/ownership/settings",
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
            match serde_json::from_str::<crate::datadogV2::model::OwnershipSettingsResponse>(
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
            let local_entity: Option<PostOwnershipSettingsError> =
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
