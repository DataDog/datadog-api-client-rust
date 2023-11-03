// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// SubmitLogParams is a struct for passing parameters to the method [`SubmitLog`]
#[derive(Clone, Debug, Default)]
pub struct SubmitLogParams {
    /// Log to send (JSON format).
    pub body: Vec<crate::datadogV2::model::HTTPLogItem>,
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV2::model::ContentEncoding>,
    /// Log tags can be passed as query parameters with `text/plain` content type.
    pub ddtags: Option<String>,
}

/// SubmitLogError is a struct for typed errors of method [`SubmitLog`]
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
        params: SubmitLogParams,
    ) -> Result<Option<std::collections::HashMap<String, serde_json::Value>>, Error<SubmitLogError>>
    {
        match self.submit_log_with_http_info(params).await {
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
        params: SubmitLogParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<SubmitLogError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;
        let content_encoding = params.content_encoding;
        let ddtags = params.ddtags;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/logs", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build parameters
        if let Some(ref local) = content_encoding {
            local_req_builder = local_req_builder.header("Content-Encoding", &local.to_string());
        };
        if let Some(ref local_str) = ddtags {
            local_req_builder = local_req_builder.query(&[("ddtags", &local_str.to_string())]);
        };

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
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
