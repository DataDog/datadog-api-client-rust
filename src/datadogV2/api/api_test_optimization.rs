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
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// SearchFlakyTestsOptionalParams is a struct for passing parameters to the method [`TestOptimizationAPI::search_flaky_tests`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchFlakyTestsOptionalParams {
    pub body: Option<crate::datadogV2::model::FlakyTestsSearchRequest>,
}

impl SearchFlakyTestsOptionalParams {
    pub fn body(mut self, value: crate::datadogV2::model::FlakyTestsSearchRequest) -> Self {
        self.body = Some(value);
        self
    }
}

/// SearchFlakyTestsError is a struct for typed errors of method [`TestOptimizationAPI::search_flaky_tests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchFlakyTestsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Search and manage flaky tests through Test Optimization. See the [Test Optimization page](<https://docs.datadoghq.com/tests/>) for more information.
#[derive(Debug, Clone)]
pub struct TestOptimizationAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for TestOptimizationAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl TestOptimizationAPI {
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

    /// List endpoint returning flaky tests from Flaky Test Management. Results are paginated.
    pub async fn search_flaky_tests(
        &self,
        params: SearchFlakyTestsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::FlakyTestsSearchResponse,
        datadog::Error<SearchFlakyTestsError>,
    > {
        match self.search_flaky_tests_with_http_info(params).await {
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

    pub fn search_flaky_tests_with_pagination(
        &self,
        mut params: SearchFlakyTestsOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::FlakyTest, datadog::Error<SearchFlakyTestsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.body.is_none() {
                params.body = Some(crate::datadogV2::model::FlakyTestsSearchRequest::new());
            }
            if params.body.as_ref().unwrap().data.is_none() {
                params.body.as_mut().unwrap().data = Some(crate::datadogV2::model::FlakyTestsSearchRequestData::new());
            }
            if params.body.as_ref().unwrap().data.as_ref().unwrap().attributes.is_none() {
                params.body.as_mut().unwrap().data.as_mut().unwrap().attributes = Some(crate::datadogV2::model::FlakyTestsSearchRequestAttributes::new());
            }
            if params.body.as_ref().unwrap().data.as_ref().unwrap().attributes.as_ref().unwrap().page.is_none() {
                params.body.as_mut().unwrap().data.as_mut().unwrap().attributes.as_mut().unwrap().page = Some(crate::datadogV2::model::FlakyTestsSearchPageOptions::new());
            }
            if params.body.as_ref().unwrap().data.as_ref().unwrap().attributes.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                params.body.as_mut().unwrap().data.as_mut().unwrap().attributes.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = params.body.as_ref().unwrap().data.as_ref().unwrap().attributes.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.search_flaky_tests(params.clone()).await?;
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
                let Some(pagination) = meta.pagination else { break };
                let Some(next_page) = pagination.next_page.unwrap() else { break };

                params.body.as_mut().unwrap().data.as_mut().unwrap().attributes.as_mut().unwrap().page.as_mut().unwrap().cursor = Some(next_page);
            }
        }
    }

    /// List endpoint returning flaky tests from Flaky Test Management. Results are paginated.
    pub async fn search_flaky_tests_with_http_info(
        &self,
        params: SearchFlakyTestsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FlakyTestsSearchResponse>,
        datadog::Error<SearchFlakyTestsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_flaky_tests";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.search_flaky_tests' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/test/flaky-test-management/tests",
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
            match serde_json::from_str::<crate::datadogV2::model::FlakyTestsSearchResponse>(
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
            let local_entity: Option<SearchFlakyTestsError> =
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
