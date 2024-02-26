// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListLogsOptionalParams is a struct for passing parameters to the method [`LogsAPI::list_logs`]
#[derive(Clone, Default, Debug)]
pub struct ListLogsOptionalParams {
    pub body: Option<crate::datadogV2::model::LogsListRequest>,
}

impl ListLogsOptionalParams {
    pub fn body(&mut self, value: crate::datadogV2::model::LogsListRequest) -> &mut Self {
        self.body = Some(value);
        self
    }
}

/// ListLogsGetOptionalParams is a struct for passing parameters to the method [`LogsAPI::list_logs_get`]
#[derive(Clone, Default, Debug)]
pub struct ListLogsGetOptionalParams {
    /// Search query following logs syntax.
    pub filter_query: Option<String>,
    /// For customers with multiple indexes, the indexes to search.
    /// Defaults to '*' which means all indexes
    pub filter_indexes: Option<Vec<String>>,
    /// Minimum timestamp for requested logs.
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested logs.
    pub filter_to: Option<String>,
    /// Specifies the storage type to be used
    pub filter_storage_tier: Option<crate::datadogV2::model::LogsStorageTier>,
    /// Order of logs in results.
    pub sort: Option<crate::datadogV2::model::LogsSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of logs in the response.
    pub page_limit: Option<i32>,
}

impl ListLogsGetOptionalParams {
    /// Search query following logs syntax.
    pub fn filter_query(&mut self, value: String) -> &mut Self {
        self.filter_query = Some(value);
        self
    }
    /// For customers with multiple indexes, the indexes to search.
    /// Defaults to '*' which means all indexes
    pub fn filter_indexes(&mut self, value: Vec<String>) -> &mut Self {
        self.filter_indexes = Some(value);
        self
    }
    /// Minimum timestamp for requested logs.
    pub fn filter_from(&mut self, value: String) -> &mut Self {
        self.filter_from = Some(value);
        self
    }
    /// Maximum timestamp for requested logs.
    pub fn filter_to(&mut self, value: String) -> &mut Self {
        self.filter_to = Some(value);
        self
    }
    /// Specifies the storage type to be used
    pub fn filter_storage_tier(
        &mut self,
        value: crate::datadogV2::model::LogsStorageTier,
    ) -> &mut Self {
        self.filter_storage_tier = Some(value);
        self
    }
    /// Order of logs in results.
    pub fn sort(&mut self, value: crate::datadogV2::model::LogsSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// List following results with a cursor provided in the previous query.
    pub fn page_cursor(&mut self, value: String) -> &mut Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of logs in the response.
    pub fn page_limit(&mut self, value: i32) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
}

/// SubmitLogOptionalParams is a struct for passing parameters to the method [`LogsAPI::submit_log`]
#[derive(Clone, Default, Debug)]
pub struct SubmitLogOptionalParams {
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV2::model::ContentEncoding>,
    /// Log tags can be passed as query parameters with `text/plain` content type.
    pub ddtags: Option<String>,
}

impl SubmitLogOptionalParams {
    /// HTTP header used to compress the media-type.
    pub fn content_encoding(
        &mut self,
        value: crate::datadogV2::model::ContentEncoding,
    ) -> &mut Self {
        self.content_encoding = Some(value);
        self
    }
    /// Log tags can be passed as query parameters with `text/plain` content type.
    pub fn ddtags(&mut self, value: String) -> &mut Self {
        self.ddtags = Some(value);
        self
    }
}

/// AggregateLogsError is a struct for typed errors of method [`LogsAPI::aggregate_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateLogsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLogsError is a struct for typed errors of method [`LogsAPI::list_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLogsGetError is a struct for typed errors of method [`LogsAPI::list_logs_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsGetError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SubmitLogError is a struct for typed errors of method [`LogsAPI::submit_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitLogError {
    Status400(Option<crate::datadogV2::model::HTTPLogErrors>),
    Status401(Option<crate::datadogV2::model::HTTPLogErrors>),
    Status403(Option<crate::datadogV2::model::HTTPLogErrors>),
    Status408(Option<crate::datadogV2::model::HTTPLogErrors>),
    Status413(Option<crate::datadogV2::model::HTTPLogErrors>),
    Status429(Option<crate::datadogV2::model::HTTPLogErrors>),
    Status500(Option<crate::datadogV2::model::HTTPLogErrors>),
    Status503(Option<crate::datadogV2::model::HTTPLogErrors>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct LogsAPI {
    config: configuration::Configuration,
}

impl Default for LogsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl LogsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// The API endpoint to aggregate events into buckets and compute metrics and timeseries.
    pub async fn aggregate_logs(
        &self,
        body: crate::datadogV2::model::LogsAggregateRequest,
    ) -> Result<Option<crate::datadogV2::model::LogsAggregateResponse>, Error<AggregateLogsError>>
    {
        match self.aggregate_logs_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The API endpoint to aggregate events into buckets and compute metrics and timeseries.
    pub async fn aggregate_logs_with_http_info(
        &self,
        body: crate::datadogV2::model::LogsAggregateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsAggregateResponse>,
        Error<AggregateLogsError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/analytics/aggregate",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::LogsAggregateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<AggregateLogsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns logs that match a log search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex logs filtering and search.
    ///
    /// **If you are considering archiving logs for your organization,
    /// consider use of the Datadog archive capabilities instead of the log list API.
    /// See [Datadog Logs Archive documentation][2].**
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination
    /// [2]: <https://docs.datadoghq.com/logs/archives>
    pub async fn list_logs(
        &self,
        params: ListLogsOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::LogsListResponse>, Error<ListLogsError>> {
        match self.list_logs_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    pub fn list_logs_with_pagination(
        &self,
        mut params: ListLogsOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::Log, Error<ListLogsError>>> + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.body.is_none() {
                params.body = Some(crate::datadogV2::model::LogsListRequest::new());
            }
            if params.body.as_ref().unwrap().page.is_none() {
                params.body.as_mut().unwrap().page = Some(crate::datadogV2::model::LogsListRequestPage::new());
            }
            if params.body.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                params.body.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = params.body.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.list_logs(params.clone()).await?;

                let Some(resp) = resp else { break };
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

    /// List endpoint returns logs that match a log search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex logs filtering and search.
    ///
    /// **If you are considering archiving logs for your organization,
    /// consider use of the Datadog archive capabilities instead of the log list API.
    /// See [Datadog Logs Archive documentation][2].**
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination
    /// [2]: <https://docs.datadoghq.com/logs/archives>
    pub async fn list_logs_with_http_info(
        &self,
        params: ListLogsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsListResponse>, Error<ListLogsError>>
    {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/events/search",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::LogsListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListLogsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns logs that match a log search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest logs.
    ///
    /// **If you are considering archiving logs for your organization,
    /// consider use of the Datadog archive capabilities instead of the log list API.
    /// See [Datadog Logs Archive documentation][2].**
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination
    /// [2]: <https://docs.datadoghq.com/logs/archives>
    pub async fn list_logs_get(
        &self,
        params: ListLogsGetOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::LogsListResponse>, Error<ListLogsGetError>> {
        match self.list_logs_get_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    pub fn list_logs_get_with_pagination(
        &self,
        mut params: ListLogsGetOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::Log, Error<ListLogsGetError>>> + '_
    {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_logs_get(params.clone()).await?;

                let Some(resp) = resp else { break };
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

    /// List endpoint returns logs that match a log search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest logs.
    ///
    /// **If you are considering archiving logs for your organization,
    /// consider use of the Datadog archive capabilities instead of the log list API.
    /// See [Datadog Logs Archive documentation][2].**
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination
    /// [2]: <https://docs.datadoghq.com/logs/archives>
    pub async fn list_logs_get_with_http_info(
        &self,
        params: ListLogsGetOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsListResponse>, Error<ListLogsGetError>>
    {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_indexes = params.filter_indexes;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let filter_storage_tier = params.filter_storage_tier;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/logs/events", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_indexes {
            local_req_builder = local_req_builder.query(&[(
                "filter[indexes]",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder =
                local_req_builder.query(&[("filter[to]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_storage_tier {
            local_req_builder = local_req_builder
                .query(&[("filter[storage_tier]", &local_query_param.to_string())]);
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

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::LogsListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListLogsGetError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Send your logs to your Datadog platform over HTTP. Limits per HTTP request are:
    ///
    /// - Maximum content size per payload (uncompressed): 5MB
    /// - Maximum size for a single log: 1MB
    /// - Maximum array size if sending multiple logs in an array: 1000 entries
    ///
    /// Any log exceeding 1MB is accepted and truncated by Datadog:
    /// - For a single log request, the API truncates the log at 1MB and returns a 2xx.
    /// - For a multi-logs request, the API processes all logs, truncates only logs larger than 1MB, and returns a 2xx.
    ///
    /// Datadog recommends sending your logs compressed.
    /// Add the `Content-Encoding: gzip` header to the request when sending compressed logs.
    /// Log events can be submitted with a timestamp that is up to 18 hours in the past.
    ///
    /// The status codes answered by the HTTP API are:
    /// - 202: Accepted: the request has been accepted for processing
    /// - 400: Bad request (likely an issue in the payload formatting)
    /// - 401: Unauthorized (likely a missing API Key)
    /// - 403: Permission issue (likely using an invalid API Key)
    /// - 408: Request Timeout, request should be retried after some time
    /// - 413: Payload too large (batch is above 5MB uncompressed)
    /// - 429: Too Many Requests, request should be retried after some time
    /// - 500: Internal Server Error, the server encountered an unexpected condition that prevented it from fulfilling the request, request should be retried after some time
    /// - 503: Service Unavailable, the server is not ready to handle the request probably because it is overloaded, request should be retried after some time
    pub async fn submit_log(
        &self,
        body: Vec<crate::datadogV2::model::HTTPLogItem>,
        params: SubmitLogOptionalParams,
    ) -> Result<Option<std::collections::BTreeMap<String, serde_json::Value>>, Error<SubmitLogError>>
    {
        match self.submit_log_with_http_info(body, params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Send your logs to your Datadog platform over HTTP. Limits per HTTP request are:
    ///
    /// - Maximum content size per payload (uncompressed): 5MB
    /// - Maximum size for a single log: 1MB
    /// - Maximum array size if sending multiple logs in an array: 1000 entries
    ///
    /// Any log exceeding 1MB is accepted and truncated by Datadog:
    /// - For a single log request, the API truncates the log at 1MB and returns a 2xx.
    /// - For a multi-logs request, the API processes all logs, truncates only logs larger than 1MB, and returns a 2xx.
    ///
    /// Datadog recommends sending your logs compressed.
    /// Add the `Content-Encoding: gzip` header to the request when sending compressed logs.
    /// Log events can be submitted with a timestamp that is up to 18 hours in the past.
    ///
    /// The status codes answered by the HTTP API are:
    /// - 202: Accepted: the request has been accepted for processing
    /// - 400: Bad request (likely an issue in the payload formatting)
    /// - 401: Unauthorized (likely a missing API Key)
    /// - 403: Permission issue (likely using an invalid API Key)
    /// - 408: Request Timeout, request should be retried after some time
    /// - 413: Payload too large (batch is above 5MB uncompressed)
    /// - 429: Too Many Requests, request should be retried after some time
    /// - 500: Internal Server Error, the server encountered an unexpected condition that prevented it from fulfilling the request, request should be retried after some time
    /// - 503: Service Unavailable, the server is not ready to handle the request probably because it is overloaded, request should be retried after some time
    pub async fn submit_log_with_http_info(
        &self,
        body: Vec<crate::datadogV2::model::HTTPLogItem>,
        params: SubmitLogOptionalParams,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<SubmitLogError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let content_encoding = params.content_encoding;
        let ddtags = params.ddtags;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/logs", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = ddtags {
            local_req_builder =
                local_req_builder.query(&[("ddtags", &local_query_param.to_string())]);
        };

        if let Some(ref local) = content_encoding {
            local_req_builder = local_req_builder.header("Content-Encoding", &local.to_string());
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SubmitLogError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
