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

/// ListRUMRecoveryBatchRequestsOptionalParams is a struct for passing parameters to the method [`RUMRecoveryAPI::list_rum_recovery_batch_requests`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListRUMRecoveryBatchRequestsOptionalParams {
    /// Number of results returned per page, at most 100 (default 50). Omitted, zero, or negative values use 50. Invalid integers return 400.
    pub limit: Option<i32>,
    /// Opaque next-page cursor. Keep type, from, to, and query unchanged between pages.
    pub cursor: Option<String>,
    /// Start of the request creation-time range in Unix milliseconds, inclusive. This filters created_at, not the time range of the sessions being recovered. Must be earlier than to when both are provided.
    pub from: Option<i64>,
    /// End of the request creation-time range in Unix milliseconds, exclusive. This filters created_at, not the time range of the sessions being recovered. Must be later than from when both are provided.
    pub to: Option<i64>,
    /// Filter recovery requests whose stored query selects matching facet values. Supports facet:value and facet:(A OR B) terms combined with AND, including * and ? wildcards in filter values. Attribute facets keep their leading @. At most 4096 bytes. Exclusions, repeated facets, free text, ranges, and OR between terms return 400. Stored queries that cannot be matched unambiguously are excluded. Blank values apply no query filter.
    pub query: Option<String>,
}

impl ListRUMRecoveryBatchRequestsOptionalParams {
    /// Number of results returned per page, at most 100 (default 50). Omitted, zero, or negative values use 50. Invalid integers return 400.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
    /// Opaque next-page cursor. Keep type, from, to, and query unchanged between pages.
    pub fn cursor(mut self, value: String) -> Self {
        self.cursor = Some(value);
        self
    }
    /// Start of the request creation-time range in Unix milliseconds, inclusive. This filters created_at, not the time range of the sessions being recovered. Must be earlier than to when both are provided.
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }
    /// End of the request creation-time range in Unix milliseconds, exclusive. This filters created_at, not the time range of the sessions being recovered. Must be later than from when both are provided.
    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }
    /// Filter recovery requests whose stored query selects matching facet values. Supports facet:value and facet:(A OR B) terms combined with AND, including * and ? wildcards in filter values. Attribute facets keep their leading @. At most 4096 bytes. Exclusions, repeated facets, free text, ranges, and OR between terms return 400. Stored queries that cannot be matched unambiguously are excluded. Blank values apply no query filter.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}

/// CreateRUMRecoveryRequestError is a struct for typed errors of method [`RUMRecoveryAPI::create_rum_recovery_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRUMRecoveryRequestError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    RumManagedArchiveErrorResponse(crate::datadogV2::model::RumManagedArchiveErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRUMRecoveryRequestError is a struct for typed errors of method [`RUMRecoveryAPI::get_rum_recovery_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMRecoveryRequestError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    RumManagedArchiveErrorResponse(crate::datadogV2::model::RumManagedArchiveErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRUMRecoverySessionCountError is a struct for typed errors of method [`RUMRecoveryAPI::get_rum_recovery_session_count`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMRecoverySessionCountError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    RumManagedArchiveErrorResponse(crate::datadogV2::model::RumManagedArchiveErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRUMRecoveryBatchRequestsError is a struct for typed errors of method [`RUMRecoveryAPI::list_rum_recovery_batch_requests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRUMRecoveryBatchRequestsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    RumManagedArchiveErrorResponse(crate::datadogV2::model::RumManagedArchiveErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchRUMArchiveSessionsError is a struct for typed errors of method [`RUMRecoveryAPI::search_rum_archive_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchRUMArchiveSessionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    RumManagedArchiveErrorResponse(crate::datadogV2::model::RumManagedArchiveErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateRUMRecoveryBatchRequestError is a struct for typed errors of method [`RUMRecoveryAPI::update_rum_recovery_batch_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRUMRecoveryBatchRequestError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    RumManagedArchiveErrorResponse(crate::datadogV2::model::RumManagedArchiveErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Search archived RUM sessions and recover individual sessions or query-based batches. Your organization must have Managed Archives enabled. Creating, counting, listing, and canceling batches also require batch recovery enablement.
///
/// Requires `rum_rehydration_filter_read` for search and request details, or `rum_rehydration_index` to create, estimate recoverable sessions, list batches, or cancel recovery requests.
#[derive(Debug, Clone)]
pub struct RUMRecoveryAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RUMRecoveryAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RUMRecoveryAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = config.apply_headers(reqwest::Client::builder());
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

    /// Start targeted or query-based batch recovery. Use the returned `data.id` to poll the recovery request for progress. Batch creation checks a fresh recoverable-session estimate against the configured maximum (100,000 by default), accepting estimates equal to the maximum. Use the count operation to check the current maximum; creation can still reject a changed estimate. Narrow the query or time range if the maximum is exceeded.
    ///
    /// Creation is not idempotent. After an uncertain response, check session search or batch listing before retrying.
    pub async fn create_rum_recovery_request(
        &self,
        body: crate::datadogV2::model::RumRecoveryCreateRequest,
    ) -> Result<
        crate::datadogV2::model::RumRecoveryCreateResponse,
        datadog::Error<CreateRUMRecoveryRequestError>,
    > {
        match self.create_rum_recovery_request_with_http_info(body).await {
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

    /// Start targeted or query-based batch recovery. Use the returned `data.id` to poll the recovery request for progress. Batch creation checks a fresh recoverable-session estimate against the configured maximum (100,000 by default), accepting estimates equal to the maximum. Use the count operation to check the current maximum; creation can still reject a changed estimate. Narrow the query or time range if the maximum is exceeded.
    ///
    /// Creation is not idempotent. After an uncertain response, check session search or batch listing before retrying.
    pub async fn create_rum_recovery_request_with_http_info(
        &self,
        body: crate::datadogV2::model::RumRecoveryCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumRecoveryCreateResponse>,
        datadog::Error<CreateRUMRecoveryRequestError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_rum_recovery_request";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/recover/requests",
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
            match serde_json::from_str::<crate::datadogV2::model::RumRecoveryCreateResponse>(
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
            let local_entity: Option<CreateRUMRecoveryRequestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get progress for a targeted or batch recovery request. A `COMPLETED` request can include failed sessions; check individual session statuses for targeted recovery.
    pub async fn get_rum_recovery_request(
        &self,
        request_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::RumRecoveryRequestResponse,
        datadog::Error<GetRUMRecoveryRequestError>,
    > {
        match self
            .get_rum_recovery_request_with_http_info(request_id)
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

    /// Get progress for a targeted or batch recovery request. A `COMPLETED` request can include failed sessions; check individual session statuses for targeted recovery.
    pub async fn get_rum_recovery_request_with_http_info(
        &self,
        request_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumRecoveryRequestResponse>,
        datadog::Error<GetRUMRecoveryRequestError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_rum_recovery_request";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/recover/requests/{request_id}",
            local_configuration.get_operation_host(local_operation_id),
            request_id = datadog::urlencode(request_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::RumRecoveryRequestResponse>(
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
            let local_entity: Option<GetRUMRecoveryRequestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Return the full estimated number of recoverable sessions and the configured maximum per batch. The estimate is not capped at that maximum. This endpoint does not create a recovery request. Narrow the query or time range if the estimate exceeds max_recoverable_sessions. Creation checks a fresh estimate and returns 400 if it exceeds the configured maximum.
    pub async fn get_rum_recovery_session_count(
        &self,
        body: crate::datadogV2::model::RumRecoveryCountRequest,
    ) -> Result<
        crate::datadogV2::model::RumRecoveryCountResponse,
        datadog::Error<GetRUMRecoverySessionCountError>,
    > {
        match self
            .get_rum_recovery_session_count_with_http_info(body)
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

    /// Return the full estimated number of recoverable sessions and the configured maximum per batch. The estimate is not capped at that maximum. This endpoint does not create a recovery request. Narrow the query or time range if the estimate exceeds max_recoverable_sessions. Creation checks a fresh estimate and returns 400 if it exceeds the configured maximum.
    pub async fn get_rum_recovery_session_count_with_http_info(
        &self,
        body: crate::datadogV2::model::RumRecoveryCountRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumRecoveryCountResponse>,
        datadog::Error<GetRUMRecoverySessionCountError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_rum_recovery_session_count";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/recover/count",
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
            match serde_json::from_str::<crate::datadogV2::model::RumRecoveryCountResponse>(
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
            let local_entity: Option<GetRUMRecoverySessionCountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List batch recovery requests for your organization. Set `type=batch`. Optional `from` and `to` bounds filter request creation time; `query` filters the queries stored on recovery requests.
    /// A filtered page can be empty or contain fewer than `limit` results and still include `meta.next_cursor`. Continue until the cursor is absent, keeping the filters unchanged between pages.
    pub async fn list_rum_recovery_batch_requests(
        &self,
        type_: crate::datadogV2::model::RumRecoveryBatchRequestType,
        params: ListRUMRecoveryBatchRequestsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::RumRecoveryBatchListResponse,
        datadog::Error<ListRUMRecoveryBatchRequestsError>,
    > {
        match self
            .list_rum_recovery_batch_requests_with_http_info(type_, params)
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

    /// List batch recovery requests for your organization. Set `type=batch`. Optional `from` and `to` bounds filter request creation time; `query` filters the queries stored on recovery requests.
    /// A filtered page can be empty or contain fewer than `limit` results and still include `meta.next_cursor`. Continue until the cursor is absent, keeping the filters unchanged between pages.
    pub async fn list_rum_recovery_batch_requests_with_http_info(
        &self,
        type_: crate::datadogV2::model::RumRecoveryBatchRequestType,
        params: ListRUMRecoveryBatchRequestsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumRecoveryBatchListResponse>,
        datadog::Error<ListRUMRecoveryBatchRequestsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_rum_recovery_batch_requests";

        // unbox and build optional parameters
        let limit = params.limit;
        let cursor = params.cursor;
        let from = params.from;
        let to = params.to;
        let query = params.query;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/recover/requests",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("type", &type_.to_string())]);
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = cursor {
            local_req_builder =
                local_req_builder.query(&[("cursor", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = from {
            local_req_builder =
                local_req_builder.query(&[("from", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = to {
            local_req_builder = local_req_builder.query(&[("to", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::RumRecoveryBatchListResponse>(
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
            let local_entity: Option<ListRUMRecoveryBatchRequestsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Search archived RUM sessions. Use `meta.page.next_cursor` to retrieve the next page.
    pub async fn search_rum_archive_sessions(
        &self,
        body: crate::datadogV2::model::RumArchiveSessionsSearchRequest,
    ) -> Result<
        crate::datadogV2::model::RumArchiveSessionsResponse,
        datadog::Error<SearchRUMArchiveSessionsError>,
    > {
        match self.search_rum_archive_sessions_with_http_info(body).await {
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

    /// Search archived RUM sessions. Use `meta.page.next_cursor` to retrieve the next page.
    pub async fn search_rum_archive_sessions_with_http_info(
        &self,
        body: crate::datadogV2::model::RumArchiveSessionsSearchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumArchiveSessionsResponse>,
        datadog::Error<SearchRUMArchiveSessionsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.search_rum_archive_sessions";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/recover/sessions",
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
            match serde_json::from_str::<crate::datadogV2::model::RumArchiveSessionsResponse>(
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
            let local_entity: Option<SearchRUMArchiveSessionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Cancel pending work for a batch recovery request. Work already in progress can finish, and recovered data is not removed. Requests in a terminal state return `409`.
    pub async fn update_rum_recovery_batch_request(
        &self,
        request_id: uuid::Uuid,
        body: crate::datadogV2::model::RumRecoveryCancelRequest,
    ) -> Result<
        crate::datadogV2::model::RumRecoveryCancelResponse,
        datadog::Error<UpdateRUMRecoveryBatchRequestError>,
    > {
        match self
            .update_rum_recovery_batch_request_with_http_info(request_id, body)
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

    /// Cancel pending work for a batch recovery request. Work already in progress can finish, and recovered data is not removed. Requests in a terminal state return `409`.
    pub async fn update_rum_recovery_batch_request_with_http_info(
        &self,
        request_id: uuid::Uuid,
        body: crate::datadogV2::model::RumRecoveryCancelRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumRecoveryCancelResponse>,
        datadog::Error<UpdateRUMRecoveryBatchRequestError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_rum_recovery_batch_request";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/recover/requests/{request_id}",
            local_configuration.get_operation_host(local_operation_id),
            request_id = datadog::urlencode(request_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::RumRecoveryCancelResponse>(
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
            let local_entity: Option<UpdateRUMRecoveryBatchRequestError> =
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
