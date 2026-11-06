// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListDashboardsOptionalParams is a struct for passing parameters to the method [`DashboardsAPI::list_dashboards`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListDashboardsOptionalParams {
    /// When `true`, this query only returns shared custom created
    /// or cloned dashboards.
    pub filter_shared: Option<bool>,
    /// When `true`, this query returns only deleted custom-created
    /// or cloned dashboards. This parameter is incompatible with `filter[shared]`.
    pub filter_deleted: Option<bool>,
    /// The maximum number of dashboards returned in the list.
    pub count: Option<i64>,
    /// The specific offset to use as the beginning of the returned response.
    pub start: Option<i64>,
}

impl ListDashboardsOptionalParams {
    /// When `true`, this query only returns shared custom created
    /// or cloned dashboards.
    pub fn filter_shared(mut self, value: bool) -> Self {
        self.filter_shared = Some(value);
        self
    }
    /// When `true`, this query returns only deleted custom-created
    /// or cloned dashboards. This parameter is incompatible with `filter[shared]`.
    pub fn filter_deleted(mut self, value: bool) -> Self {
        self.filter_deleted = Some(value);
        self
    }
    /// The maximum number of dashboards returned in the list.
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }
    /// The specific offset to use as the beginning of the returned response.
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }
}

/// ListDashboardsError is a struct for typed errors of method [`DashboardsAPI::list_dashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDashboardsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage all your dashboards, as well as access to your shared dashboards, through the API. See the [Dashboards page](<https://docs.datadoghq.com/dashboards/>) for more information.
#[derive(Debug, Clone)]
pub struct DashboardsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for DashboardsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl DashboardsAPI {
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

    /// Get all dashboards.
    ///
    /// **Note**: This query will only return custom created or cloned dashboards.
    /// This query will not return preset dashboards.
    pub async fn list_dashboards(
        &self,
        params: ListDashboardsOptionalParams,
    ) -> Result<
        crate::datadogV1_20270101::model::DashboardSummary,
        datadog::Error<ListDashboardsError>,
    > {
        match self.list_dashboards_with_http_info(params).await {
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

    pub fn list_dashboards_with_pagination(
        &self,
        mut params: ListDashboardsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV1_20270101::model::DashboardSummaryDefinition,
            datadog::Error<ListDashboardsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.count.is_none() {
                params.count = Some(page_size);
            } else {
                page_size = params.count.unwrap().clone();
            }
            loop {
                let resp = self.list_dashboards(params.clone()).await?;
                let Some(dashboards) = resp.dashboards else { break };

                let r = dashboards;
                let count = r.len();
                for team in r {
                    yield team;
                }
                if count < page_size as usize {
                    break;
                }
                if params.start.is_none() {
                    params.start = Some(page_size.clone());
                } else {
                    params.start = Some(params.start.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get all dashboards.
    ///
    /// **Note**: This query will only return custom created or cloned dashboards.
    /// This query will not return preset dashboards.
    pub async fn list_dashboards_with_http_info(
        &self,
        params: ListDashboardsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1_20270101::model::DashboardSummary>,
        datadog::Error<ListDashboardsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v1_20270101.list_dashboards";

        // unbox and build optional parameters
        let filter_shared = params.filter_shared;
        let filter_deleted = params.filter_deleted;
        let count = params.count;
        let start = params.start;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_shared {
            local_req_builder =
                local_req_builder.query(&[("filter[shared]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_deleted {
            local_req_builder =
                local_req_builder.query(&[("filter[deleted]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = count {
            local_req_builder =
                local_req_builder.query(&[("count", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = start {
            local_req_builder =
                local_req_builder.query(&[("start", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("DD-API-Version", HeaderValue::from_static("2027-01-01"));

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
            match serde_json::from_str::<crate::datadogV1_20270101::model::DashboardSummary>(
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
            let local_entity: Option<ListDashboardsError> =
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
