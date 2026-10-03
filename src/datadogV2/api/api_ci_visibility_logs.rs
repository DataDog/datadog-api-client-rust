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

/// SubmitCILogOptionalParams is a struct for passing parameters to the method [`CIVisibilityLogsAPI::submit_ci_log`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SubmitCILogOptionalParams {
    /// HTTP header used to compress the JSON request body.
    pub content_encoding: Option<crate::datadogV2::model::CILogContentEncoding>,
}

impl SubmitCILogOptionalParams {
    /// HTTP header used to compress the JSON request body.
    pub fn content_encoding(
        mut self,
        value: crate::datadogV2::model::CILogContentEncoding,
    ) -> Self {
        self.content_encoding = Some(value);
        self
    }
}

/// SubmitCILogError is a struct for typed errors of method [`CIVisibilityLogsAPI::submit_ci_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitCILogError {
    CILogIntakeErrors(crate::datadogV2::model::CILogIntakeErrors),
    CILogErrors(crate::datadogV2::model::CILogErrors),
    UnknownValue(serde_json::Value),
}

/// Send CI job logs over HTTP for CI Visibility.
#[derive(Debug, Clone)]
pub struct CIVisibilityLogsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CIVisibilityLogsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CIVisibilityLogsAPI {
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

    /// Send log lines for a CI job over HTTP. See the [CI Visibility Pipelines
    /// API](<https://docs.datadoghq.com/api/latest/ci-visibility-pipelines/send-pipeline-event/>) for submitting the
    /// associated pipeline and job events.
    ///
    /// A request can contain one log object or an array of up to 1,000 log objects. The maximum uncompressed request
    /// body size is 5.1 MiB.
    ///
    /// You can stream log lines while a CI job runs or send them after it finishes. After you submit the completed job
    /// event, 20 seconds without a new log line marks the job's logs as complete. Lines sent after that may not appear.
    ///
    /// A job can have up to 128 additional attributes and 256 tags. Additional attributes are top-level fields with
    /// string, number, boolean, or null values. Nested objects and arrays are rejected. Additional attributes and
    /// `ddtags` apply to all log lines in the job. If an additional attribute has different values on different lines,
    /// the first value received is used. Tags supplied on different lines are combined. A job can contain up to
    /// 2,000,000 log records or 1 GiB of message bytes in total.
    ///
    /// To reduce request size, send gzip-compressed JSON with the `Content-Encoding: gzip` header. Retry requests after
    /// a 408, 429, 500, or 503 response.
    pub async fn submit_ci_log(
        &self,
        body: Vec<crate::datadogV2::model::CILogItem>,
        params: SubmitCILogOptionalParams,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<SubmitCILogError>,
    > {
        match self.submit_ci_log_with_http_info(body, params).await {
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

    /// Send log lines for a CI job over HTTP. See the [CI Visibility Pipelines
    /// API](<https://docs.datadoghq.com/api/latest/ci-visibility-pipelines/send-pipeline-event/>) for submitting the
    /// associated pipeline and job events.
    ///
    /// A request can contain one log object or an array of up to 1,000 log objects. The maximum uncompressed request
    /// body size is 5.1 MiB.
    ///
    /// You can stream log lines while a CI job runs or send them after it finishes. After you submit the completed job
    /// event, 20 seconds without a new log line marks the job's logs as complete. Lines sent after that may not appear.
    ///
    /// A job can have up to 128 additional attributes and 256 tags. Additional attributes are top-level fields with
    /// string, number, boolean, or null values. Nested objects and arrays are rejected. Additional attributes and
    /// `ddtags` apply to all log lines in the job. If an additional attribute has different values on different lines,
    /// the first value received is used. Tags supplied on different lines are combined. A job can contain up to
    /// 2,000,000 log records or 1 GiB of message bytes in total.
    ///
    /// To reduce request size, send gzip-compressed JSON with the `Content-Encoding: gzip` header. Retry requests after
    /// a 408, 429, 500, or 503 response.
    pub async fn submit_ci_log_with_http_info(
        &self,
        body: Vec<crate::datadogV2::model::CILogItem>,
        params: SubmitCILogOptionalParams,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<SubmitCILogError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.submit_ci_log";

        // unbox and build optional parameters
        let content_encoding = params.content_encoding;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cilogs",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            let local_entity: Option<SubmitCILogError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
