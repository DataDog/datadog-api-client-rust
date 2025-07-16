// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetAggregatedConnectionsOptionalParams is a struct for passing parameters to the method [`CloudNetworkMonitoringAPI::get_aggregated_connections`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetAggregatedConnectionsOptionalParams {
    /// Unix timestamp (number of seconds since epoch) of the start of the query window. If not provided, the start of the query window is 15 minutes before the `to` timestamp. If neither `from` nor `to` are provided, the query window is `[now - 15m, now]`.
    pub from: Option<i64>,
    /// Unix timestamp (number of seconds since epoch) of the end of the query window. If not provided, the end of the query window is the current time. If neither `from` nor `to` are provided, the query window is `[now - 15m, now]`.
    pub to: Option<i64>,
    /// Comma-separated list of fields to group connections by.
    pub group_by: Option<String>,
    /// Comma-separated list of tags to filter connections by.
    pub tags: Option<String>,
    /// The number of connections to be returned. The maximum value is 7500.
    pub limit: Option<i32>,
}

impl GetAggregatedConnectionsOptionalParams {
    /// Unix timestamp (number of seconds since epoch) of the start of the query window. If not provided, the start of the query window is 15 minutes before the `to` timestamp. If neither `from` nor `to` are provided, the query window is `[now - 15m, now]`.
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }
    /// Unix timestamp (number of seconds since epoch) of the end of the query window. If not provided, the end of the query window is the current time. If neither `from` nor `to` are provided, the query window is `[now - 15m, now]`.
    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }
    /// Comma-separated list of fields to group connections by.
    pub fn group_by(mut self, value: String) -> Self {
        self.group_by = Some(value);
        self
    }
    /// Comma-separated list of tags to filter connections by.
    pub fn tags(mut self, value: String) -> Self {
        self.tags = Some(value);
        self
    }
    /// The number of connections to be returned. The maximum value is 7500.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// GetAggregatedConnectionsError is a struct for typed errors of method [`CloudNetworkMonitoringAPI::get_aggregated_connections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAggregatedConnectionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// The Cloud Network Monitoring API allows you to fetch aggregated connections and their attributes. See the [Cloud Network Monitoring page](<https://docs.datadoghq.com/network_monitoring/cloud_network_monitoring/>) for more information.
#[derive(Debug, Clone)]
pub struct CloudNetworkMonitoringAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CloudNetworkMonitoringAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CloudNetworkMonitoringAPI {
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

    /// Get all aggregated connections.
    pub async fn get_aggregated_connections(
        &self,
        params: GetAggregatedConnectionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SingleAggregatedConnectionResponseArray,
        datadog::Error<GetAggregatedConnectionsError>,
    > {
        match self.get_aggregated_connections_with_http_info(params).await {
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

    /// Get all aggregated connections.
    pub async fn get_aggregated_connections_with_http_info(
        &self,
        params: GetAggregatedConnectionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SingleAggregatedConnectionResponseArray>,
        datadog::Error<GetAggregatedConnectionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_aggregated_connections";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_aggregated_connections' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let from = params.from;
        let to = params.to;
        let group_by = params.group_by;
        let tags = params.tags;
        let limit = params.limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/network/connections/aggregate",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = from {
            local_req_builder =
                local_req_builder.query(&[("from", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = to {
            local_req_builder = local_req_builder.query(&[("to", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = group_by {
            local_req_builder =
                local_req_builder.query(&[("group_by", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tags {
            local_req_builder =
                local_req_builder.query(&[("tags", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SingleAggregatedConnectionResponseArray,
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
            let local_entity: Option<GetAggregatedConnectionsError> =
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
