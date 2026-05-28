// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetPrunedTraceByIDOptionalParams is a struct for passing parameters to the method [`APMTraceAPI::get_pruned_trace_by_id`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetPrunedTraceByIDOptionalParams {
    /// Span ID to expand and preserve in the pruned tree even when its branch would
    /// normally be summarized.
    pub expand_span_id: Option<i64>,
    /// Optional Unix time hint, in seconds, used to optimize the lookup of the trace
    /// in long-term storage.
    pub time_hint: Option<i32>,
    /// Force the trace to be loaded from a specific source. When unset, the API picks
    /// the source automatically.
    pub force_source: Option<String>,
    /// Restrict the pruned tree to spans matching the given `key:value` pairs.
    /// Values may be passed as repeated query parameters.
    pub include_path: Option<Vec<String>>,
    /// Regex patterns of tag keys whose values must be included in the pruned spans.
    /// Values may be passed as repeated query parameters.
    pub tag_include: Option<Vec<String>>,
    /// Regex patterns of tag keys whose values must be excluded from the pruned spans.
    /// Values may be passed as repeated query parameters.
    pub tag_exclude: Option<Vec<String>>,
    /// When set to `true`, only service entry spans are included in the pruned tree.
    pub only_service_entry_spans: Option<bool>,
}

impl GetPrunedTraceByIDOptionalParams {
    /// Span ID to expand and preserve in the pruned tree even when its branch would
    /// normally be summarized.
    pub fn expand_span_id(mut self, value: i64) -> Self {
        self.expand_span_id = Some(value);
        self
    }
    /// Optional Unix time hint, in seconds, used to optimize the lookup of the trace
    /// in long-term storage.
    pub fn time_hint(mut self, value: i32) -> Self {
        self.time_hint = Some(value);
        self
    }
    /// Force the trace to be loaded from a specific source. When unset, the API picks
    /// the source automatically.
    pub fn force_source(mut self, value: String) -> Self {
        self.force_source = Some(value);
        self
    }
    /// Restrict the pruned tree to spans matching the given `key:value` pairs.
    /// Values may be passed as repeated query parameters.
    pub fn include_path(mut self, value: Vec<String>) -> Self {
        self.include_path = Some(value);
        self
    }
    /// Regex patterns of tag keys whose values must be included in the pruned spans.
    /// Values may be passed as repeated query parameters.
    pub fn tag_include(mut self, value: Vec<String>) -> Self {
        self.tag_include = Some(value);
        self
    }
    /// Regex patterns of tag keys whose values must be excluded from the pruned spans.
    /// Values may be passed as repeated query parameters.
    pub fn tag_exclude(mut self, value: Vec<String>) -> Self {
        self.tag_exclude = Some(value);
        self
    }
    /// When set to `true`, only service entry spans are included in the pruned tree.
    pub fn only_service_entry_spans(mut self, value: bool) -> Self {
        self.only_service_entry_spans = Some(value);
        self
    }
}

/// GetTraceByIDOptionalParams is a struct for passing parameters to the method [`APMTraceAPI::get_trace_by_id`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetTraceByIDOptionalParams {
    /// List of span fields to include in the response. When omitted, every available field is returned.
    /// Values may be passed as repeated query parameters or as a single comma-separated value.
    pub include_fields: Option<Vec<String>>,
}

impl GetTraceByIDOptionalParams {
    /// List of span fields to include in the response. When omitted, every available field is returned.
    /// Values may be passed as repeated query parameters or as a single comma-separated value.
    pub fn include_fields(mut self, value: Vec<String>) -> Self {
        self.include_fields = Some(value);
        self
    }
}

/// GetPrunedTraceByIDError is a struct for typed errors of method [`APMTraceAPI::get_pruned_trace_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPrunedTraceByIDError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTraceByIDError is a struct for typed errors of method [`APMTraceAPI::get_trace_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTraceByIDError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Retrieve full or pruned APM traces by trace ID.
#[derive(Debug, Clone)]
pub struct APMTraceAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for APMTraceAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl APMTraceAPI {
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

    /// Retrieve a pruned, hierarchical view of an APM trace by its trace ID.
    /// The trace is summarized as a tree of spans rooted at the trace root and reduced in size
    /// to keep rendering large traces in the UI practical.
    /// This endpoint is rate limited to `60` requests per minute per organization.
    pub async fn get_pruned_trace_by_id(
        &self,
        trace_id: String,
        params: GetPrunedTraceByIDOptionalParams,
    ) -> Result<crate::datadogV2::model::PrunedTraceResponse, datadog::Error<GetPrunedTraceByIDError>>
    {
        match self
            .get_pruned_trace_by_id_with_http_info(trace_id, params)
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

    /// Retrieve a pruned, hierarchical view of an APM trace by its trace ID.
    /// The trace is summarized as a tree of spans rooted at the trace root and reduced in size
    /// to keep rendering large traces in the UI practical.
    /// This endpoint is rate limited to `60` requests per minute per organization.
    pub async fn get_pruned_trace_by_id_with_http_info(
        &self,
        trace_id: String,
        params: GetPrunedTraceByIDOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::PrunedTraceResponse>,
        datadog::Error<GetPrunedTraceByIDError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_pruned_trace_by_id";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_pruned_trace_by_id' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let expand_span_id = params.expand_span_id;
        let time_hint = params.time_hint;
        let force_source = params.force_source;
        let include_path = params.include_path;
        let tag_include = params.tag_include;
        let tag_exclude = params.tag_exclude;
        let only_service_entry_spans = params.only_service_entry_spans;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/pruned_trace/{trace_id}",
            local_configuration.get_operation_host(operation_id),
            trace_id = datadog::urlencode(trace_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = expand_span_id {
            local_req_builder =
                local_req_builder.query(&[("expand_span_id", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = time_hint {
            local_req_builder =
                local_req_builder.query(&[("time_hint", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = force_source {
            local_req_builder =
                local_req_builder.query(&[("force_source", &local_query_param.to_string())]);
        };
        if let Some(ref local) = include_path {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("include_path", &param.to_string())]);
            }
        };
        if let Some(ref local) = tag_include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("tag_include", &param.to_string())]);
            }
        };
        if let Some(ref local) = tag_exclude {
            for param in local {
                local_req_builder = local_req_builder.query(&[("tag_exclude", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = only_service_entry_spans {
            local_req_builder = local_req_builder
                .query(&[("only_service_entry_spans", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::PrunedTraceResponse>(
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
            let local_entity: Option<GetPrunedTraceByIDError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a full APM trace by its trace ID, including every span in the trace.
    /// Traces are returned from live storage when available and fall back to longer-term storage.
    /// This endpoint is rate limited to `60` requests per minute per organization.
    pub async fn get_trace_by_id(
        &self,
        trace_id: String,
        params: GetTraceByIDOptionalParams,
    ) -> Result<crate::datadogV2::model::TraceResponse, datadog::Error<GetTraceByIDError>> {
        match self.get_trace_by_id_with_http_info(trace_id, params).await {
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

    /// Retrieve a full APM trace by its trace ID, including every span in the trace.
    /// Traces are returned from live storage when available and fall back to longer-term storage.
    /// This endpoint is rate limited to `60` requests per minute per organization.
    pub async fn get_trace_by_id_with_http_info(
        &self,
        trace_id: String,
        params: GetTraceByIDOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TraceResponse>,
        datadog::Error<GetTraceByIDError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_trace_by_id";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_trace_by_id' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include_fields = params.include_fields;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/trace/{trace_id}",
            local_configuration.get_operation_host(operation_id),
            trace_id = datadog::urlencode(trace_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include_fields {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("include_fields", &param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::TraceResponse>(&local_content) {
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
            let local_entity: Option<GetTraceByIDError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
