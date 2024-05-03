// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListCIAppPipelineEventsOptionalParams is a struct for passing parameters to the method [`CIVisibilityPipelinesAPI::list_ci_app_pipeline_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCIAppPipelineEventsOptionalParams {
    /// Search query following log syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<chrono::DateTime<chrono::Utc>>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<chrono::DateTime<chrono::Utc>>,
    /// Order of events in results.
    pub sort: Option<crate::datadogV2::model::CIAppSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page_limit: Option<i32>,
}

impl ListCIAppPipelineEventsOptionalParams {
    /// Search query following log syntax.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Minimum timestamp for requested events.
    pub fn filter_from(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// Maximum timestamp for requested events.
    pub fn filter_to(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// Order of events in results.
    pub fn sort(mut self, value: crate::datadogV2::model::CIAppSort) -> Self {
        self.sort = Some(value);
        self
    }
    /// List following results with a cursor provided in the previous query.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of events in the response.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// SearchCIAppPipelineEventsOptionalParams is a struct for passing parameters to the method [`CIVisibilityPipelinesAPI::search_ci_app_pipeline_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchCIAppPipelineEventsOptionalParams {
    pub body: Option<crate::datadogV2::model::CIAppPipelineEventsRequest>,
}

impl SearchCIAppPipelineEventsOptionalParams {
    pub fn body(mut self, value: crate::datadogV2::model::CIAppPipelineEventsRequest) -> Self {
        self.body = Some(value);
        self
    }
}

/// AggregateCIAppPipelineEventsError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::aggregate_ci_app_pipeline_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateCIAppPipelineEventsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateCIAppPipelineEventError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::create_ci_app_pipeline_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCIAppPipelineEventError {
    HTTPCIAppErrors(crate::datadogV2::model::HTTPCIAppErrors),
    UnknownValue(serde_json::Value),
}

/// ListCIAppPipelineEventsError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::list_ci_app_pipeline_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCIAppPipelineEventsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchCIAppPipelineEventsError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::search_ci_app_pipeline_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCIAppPipelineEventsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Search or aggregate your CI Visibility pipeline events and send them to your Datadog site over HTTP. See the [CI Pipeline Visibility in Datadog page](<https://docs.datadoghq.com/continuous_integration/pipelines/>) for more information.
#[derive(Debug, Clone)]
pub struct CIVisibilityPipelinesAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CIVisibilityPipelinesAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CIVisibilityPipelinesAPI {
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

    /// Use this API endpoint to aggregate CI Visibility pipeline events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_pipeline_events(
        &self,
        body: crate::datadogV2::model::CIAppPipelinesAggregateRequest,
    ) -> Result<
        crate::datadogV2::model::CIAppPipelinesAnalyticsAggregateResponse,
        datadog::Error<AggregateCIAppPipelineEventsError>,
    > {
        match self
            .aggregate_ci_app_pipeline_events_with_http_info(body)
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

    /// Use this API endpoint to aggregate CI Visibility pipeline events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_pipeline_events_with_http_info(
        &self,
        body: crate::datadogV2::model::CIAppPipelinesAggregateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CIAppPipelinesAnalyticsAggregateResponse>,
        datadog::Error<AggregateCIAppPipelineEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.aggregate_ci_app_pipeline_events";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/pipelines/analytics/aggregate",
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
            match serde_json::from_str::<
                crate::datadogV2::model::CIAppPipelinesAnalyticsAggregateResponse,
            >(&local_content)
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
            let local_entity: Option<AggregateCIAppPipelineEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Send your pipeline event to your Datadog platform over HTTP. For details about how pipeline executions are modeled and what execution types we support, see [Pipeline Data Model And Execution Types](<https://docs.datadoghq.com/continuous_integration/guides/pipeline_data_model/>).
    ///
    /// Pipeline events can be submitted with a timestamp that is up to 18 hours in the past.
    pub async fn create_ci_app_pipeline_event(
        &self,
        body: crate::datadogV2::model::CIAppCreatePipelineEventRequest,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<CreateCIAppPipelineEventError>,
    > {
        match self.create_ci_app_pipeline_event_with_http_info(body).await {
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

    /// Send your pipeline event to your Datadog platform over HTTP. For details about how pipeline executions are modeled and what execution types we support, see [Pipeline Data Model And Execution Types](<https://docs.datadoghq.com/continuous_integration/guides/pipeline_data_model/>).
    ///
    /// Pipeline events can be submitted with a timestamp that is up to 18 hours in the past.
    pub async fn create_ci_app_pipeline_event_with_http_info(
        &self,
        body: crate::datadogV2::model::CIAppCreatePipelineEventRequest,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<CreateCIAppPipelineEventError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_ci_app_pipeline_event";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/pipeline",
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
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
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
            let local_entity: Option<CreateCIAppPipelineEventError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest pipeline events.
    pub async fn list_ci_app_pipeline_events(
        &self,
        params: ListCIAppPipelineEventsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CIAppPipelineEventsResponse,
        datadog::Error<ListCIAppPipelineEventsError>,
    > {
        match self
            .list_ci_app_pipeline_events_with_http_info(params)
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

    pub fn list_ci_app_pipeline_events_with_pagination(
        &self,
        mut params: ListCIAppPipelineEventsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::CIAppPipelineEvent,
            datadog::Error<ListCIAppPipelineEventsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_ci_app_pipeline_events(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                params.page_cursor = Some(after);
            }
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest pipeline events.
    pub async fn list_ci_app_pipeline_events_with_http_info(
        &self,
        params: ListCIAppPipelineEventsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CIAppPipelineEventsResponse>,
        datadog::Error<ListCIAppPipelineEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_ci_app_pipeline_events";

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/pipelines/events",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_from {
            local_req_builder = local_req_builder.query(&[(
                "filter[from]",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder = local_req_builder.query(&[(
                "filter[to]",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CIAppPipelineEventsResponse>(
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
            let local_entity: Option<ListCIAppPipelineEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_pipeline_events(
        &self,
        params: SearchCIAppPipelineEventsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CIAppPipelineEventsResponse,
        datadog::Error<SearchCIAppPipelineEventsError>,
    > {
        match self
            .search_ci_app_pipeline_events_with_http_info(params)
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

    pub fn search_ci_app_pipeline_events_with_pagination(
        &self,
        mut params: SearchCIAppPipelineEventsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::CIAppPipelineEvent,
            datadog::Error<SearchCIAppPipelineEventsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.body.is_none() {
                params.body = Some(crate::datadogV2::model::CIAppPipelineEventsRequest::new());
            }
            if params.body.as_ref().unwrap().page.is_none() {
                params.body.as_mut().unwrap().page = Some(crate::datadogV2::model::CIAppQueryPageOptions::new());
            }
            if params.body.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                params.body.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = params.body.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.search_ci_app_pipeline_events(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                params.body.as_mut().unwrap().page.as_mut().unwrap().cursor = Some(after);
            }
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_pipeline_events_with_http_info(
        &self,
        params: SearchCIAppPipelineEventsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CIAppPipelineEventsResponse>,
        datadog::Error<SearchCIAppPipelineEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_ci_app_pipeline_events";

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/pipelines/events/search",
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
            match serde_json::from_str::<crate::datadogV2::model::CIAppPipelineEventsResponse>(
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
            let local_entity: Option<SearchCIAppPipelineEventsError> =
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
