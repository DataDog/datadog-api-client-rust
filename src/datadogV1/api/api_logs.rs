// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::io::Write;

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
    pub fn content_encoding(mut self, value: crate::datadogV1::model::ContentEncoding) -> Self {
        self.content_encoding = Some(value);
        self
    }
    /// Log tags can be passed as query parameters with `text/plain` content type.
    pub fn ddtags(mut self, value: String) -> Self {
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for LogsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl LogsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
        let client = middleware_client_builder.build();
        Self { config, client }
    }

    pub fn with_client_and_config(
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
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
    ) -> Result<crate::datadogV1::model::LogsListResponse, Error<ListLogsError>> {
        match self.list_logs_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        let operation_id = "v1.list_logs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/logs-queries/list",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
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
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => panic!(
                        "Unsupported content encoding: {}",
                        content_encoding
                            .to_str()
                            .expect("non-ascii content encoding header value")
                    ),
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::LogsListResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
    ) -> Result<std::collections::BTreeMap<String, serde_json::Value>, Error<SubmitLogError>> {
        match self.submit_log_with_http_info(body, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        let operation_id = "v1.submit_log";

        // unbox and build optional parameters
        let content_encoding = params.content_encoding;
        let ddtags = params.ddtags;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/v1/input",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = ddtags {
            local_req_builder =
                local_req_builder.query(&[("ddtags", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        if let Some(ref local) = content_encoding {
            headers.insert(
                "Content-Encoding",
                local
                    .to_string()
                    .parse()
                    .expect("failed to parse Content-Encoding header"),
            );
        }

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
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
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => panic!(
                        "Unsupported content encoding: {}",
                        content_encoding
                            .to_str()
                            .expect("non-ascii content encoding header value")
                    ),
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
