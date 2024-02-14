// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// SubmitLogOptionalParams is a struct for passing parameters to the method [`LogsAPI::submit_log`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SubmitLogOptionalParams {
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV1::model::ContentEncoding>,
    /// Log tags can be passed as query parameters with `text/plain` content type.
    pub ddtags: Option<String>,
}

impl SubmitLogOptionalParams {
    /// HTTP header used to compress the media-type.
    pub fn content_encoding(
        &mut self,
        value: crate::datadogV1::model::ContentEncoding,
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

/// ListLogsError is a struct for typed errors of method [`LogsAPI::list_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SubmitLogError is a struct for typed errors of method [`LogsAPI::submit_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitLogError {
    Status400(Option<crate::datadogV1::model::HTTPLogError>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
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

    /// List endpoint returns logs that match a log search query.
    /// [Results are paginated][1].
    ///
    /// **If you are considering archiving logs for your organization,
    /// consider use of the Datadog archive capabilities instead of the log list API.
    /// See [Datadog Logs Archive documentation][2].**
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination
    /// [2]: <https://docs.datadoghq.com/logs/archives>
    pub async fn list_logs(
        &self,
        body: crate::datadogV1::model::LogsListRequest,
    ) -> Result<Option<crate::datadogV1::model::LogsListResponse>, Error<ListLogsError>> {
        match self.list_logs_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns logs that match a log search query.
    /// [Results are paginated][1].
    ///
    /// **If you are considering archiving logs for your organization,
    /// consider use of the Datadog archive capabilities instead of the log list API.
    /// See [Datadog Logs Archive documentation][2].**
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination
    /// [2]: <https://docs.datadoghq.com/logs/archives>
    pub async fn list_logs_with_http_info(
        &self,
        body: crate::datadogV1::model::LogsListRequest,
    ) -> Result<ResponseContent<crate::datadogV1::model::LogsListResponse>, Error<ListLogsError>>
    {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/logs-queries/list", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::LogsListResponse> =
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
    ///
    /// The status codes answered by the HTTP API are:
    /// - 200: OK
    /// - 400: Bad request (likely an issue in the payload formatting)
    /// - 403: Permission issue (likely using an invalid API Key)
    /// - 413: Payload too large (batch is above 5MB uncompressed)
    /// - 5xx: Internal error, request should be retried after some time
    pub async fn submit_log(
        &self,
        body: Vec<crate::datadogV1::model::HTTPLogItem>,
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
    ///
    /// The status codes answered by the HTTP API are:
    /// - 200: OK
    /// - 400: Bad request (likely an issue in the payload formatting)
    /// - 403: Permission issue (likely using an invalid API Key)
    /// - 413: Payload too large (batch is above 5MB uncompressed)
    /// - 5xx: Internal error, request should be retried after some time
    pub async fn submit_log_with_http_info(
        &self,
        body: Vec<crate::datadogV1::model::HTTPLogItem>,
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

        let local_uri_str = format!("{}/v1/input", local_configuration.base_path);
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
