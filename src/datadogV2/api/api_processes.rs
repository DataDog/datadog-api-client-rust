// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListProcessesOptionalParams is a struct for passing parameters to the method [`ProcessesAPI::list_processes`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListProcessesOptionalParams {
    /// String to search processes by.
    pub search: Option<String>,
    /// Comma-separated list of tags to filter processes by.
    pub tags: Option<String>,
    /// Unix timestamp (number of seconds since epoch) of the start of the query window.
    /// If not provided, the start of the query window will be 15 minutes before the `to` timestamp. If neither
    /// `from` nor `to` are provided, the query window will be `[now - 15m, now]`.
    pub from: Option<i64>,
    /// Unix timestamp (number of seconds since epoch) of the end of the query window.
    /// If not provided, the end of the query window will be 15 minutes after the `from` timestamp. If neither
    /// `from` nor `to` are provided, the query window will be `[now - 15m, now]`.
    pub to: Option<i64>,
    /// Maximum number of results returned.
    pub page_limit: Option<i32>,
    /// String to query the next page of results.
    /// This key is provided with each valid response from the API in `meta.page.after`.
    pub page_cursor: Option<String>,
}

impl ListProcessesOptionalParams {
    /// String to search processes by.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// Comma-separated list of tags to filter processes by.
    pub fn tags(mut self, value: String) -> Self {
        self.tags = Some(value);
        self
    }
    /// Unix timestamp (number of seconds since epoch) of the start of the query window.
    /// If not provided, the start of the query window will be 15 minutes before the `to` timestamp. If neither
    /// `from` nor `to` are provided, the query window will be `[now - 15m, now]`.
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }
    /// Unix timestamp (number of seconds since epoch) of the end of the query window.
    /// If not provided, the end of the query window will be 15 minutes after the `from` timestamp. If neither
    /// `from` nor `to` are provided, the query window will be `[now - 15m, now]`.
    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }
    /// Maximum number of results returned.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// String to query the next page of results.
    /// This key is provided with each valid response from the API in `meta.page.after`.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
}

/// ListProcessesError is a struct for typed errors of method [`ProcessesAPI::list_processes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListProcessesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// The processes API allows you to query processes data for your organization. See the [Live Processes page](<https://docs.datadoghq.com/infrastructure/process/>) for more information.
#[derive(Debug, Clone)]
pub struct ProcessesAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ProcessesAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ProcessesAPI {
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

    /// Get all processes for your organization.
    pub async fn list_processes(
        &self,
        params: ListProcessesOptionalParams,
    ) -> Result<crate::datadogV2::model::ProcessSummariesResponse, datadog::Error<ListProcessesError>>
    {
        match self.list_processes_with_http_info(params).await {
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

    pub fn list_processes_with_pagination(
        &self,
        mut params: ListProcessesOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::ProcessSummary, datadog::Error<ListProcessesError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 1000;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_processes(params.clone()).await?;
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

    /// Get all processes for your organization.
    pub async fn list_processes_with_http_info(
        &self,
        params: ListProcessesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ProcessSummariesResponse>,
        datadog::Error<ListProcessesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_processes";

        // unbox and build optional parameters
        let search = params.search;
        let tags = params.tags;
        let from = params.from;
        let to = params.to;
        let page_limit = params.page_limit;
        let page_cursor = params.page_cursor;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/processes",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tags {
            local_req_builder =
                local_req_builder.query(&[("tags", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = from {
            local_req_builder =
                local_req_builder.query(&[("from", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = to {
            local_req_builder = local_req_builder.query(&[("to", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ProcessSummariesResponse>(
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
            let local_entity: Option<ListProcessesError> =
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
