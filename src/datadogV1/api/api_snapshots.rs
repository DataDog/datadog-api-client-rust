// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetGraphSnapshotOptionalParams is a struct for passing parameters to the method [`SnapshotsAPI::get_graph_snapshot`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetGraphSnapshotOptionalParams {
    /// The metric query.
    pub metric_query: Option<String>,
    /// A query that adds event bands to the graph.
    pub event_query: Option<String>,
    /// A JSON document defining the graph. `graph_def` can be used instead of `metric_query`.
    /// The JSON document uses the [grammar defined here](<https://docs.datadoghq.com/graphing/graphing_json/#grammar>)
    /// and should be formatted to a single line then URL encoded.
    pub graph_def: Option<String>,
    /// A title for the graph. If no title is specified, the graph does not have a title.
    pub title: Option<String>,
    /// The height of the graph. If no height is specified, the graph's original height is used.
    pub height: Option<i64>,
    /// The width of the graph. If no width is specified, the graph's original width is used.
    pub width: Option<i64>,
}

impl GetGraphSnapshotOptionalParams {
    /// The metric query.
    pub fn metric_query(mut self, value: String) -> Self {
        self.metric_query = Some(value);
        self
    }
    /// A query that adds event bands to the graph.
    pub fn event_query(mut self, value: String) -> Self {
        self.event_query = Some(value);
        self
    }
    /// A JSON document defining the graph. `graph_def` can be used instead of `metric_query`.
    /// The JSON document uses the [grammar defined here](<https://docs.datadoghq.com/graphing/graphing_json/#grammar>)
    /// and should be formatted to a single line then URL encoded.
    pub fn graph_def(mut self, value: String) -> Self {
        self.graph_def = Some(value);
        self
    }
    /// A title for the graph. If no title is specified, the graph does not have a title.
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
    /// The height of the graph. If no height is specified, the graph's original height is used.
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }
    /// The width of the graph. If no width is specified, the graph's original width is used.
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }
}

/// GetGraphSnapshotError is a struct for typed errors of method [`SnapshotsAPI::get_graph_snapshot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGraphSnapshotError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Take graph snapshots using the API.
#[derive(Debug, Clone)]
pub struct SnapshotsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for SnapshotsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl SnapshotsAPI {
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

    /// Take graph snapshots. Snapshots are PNG images generated by rendering a specified widget in a web page and capturing it once the data is available. The image is then uploaded to cloud storage.
    ///
    /// **Note**: When a snapshot is created, there is some delay before it is available.
    pub async fn get_graph_snapshot(
        &self,
        start: i64,
        end: i64,
        params: GetGraphSnapshotOptionalParams,
    ) -> Result<crate::datadogV1::model::GraphSnapshot, datadog::Error<GetGraphSnapshotError>> {
        match self
            .get_graph_snapshot_with_http_info(start, end, params)
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

    /// Take graph snapshots. Snapshots are PNG images generated by rendering a specified widget in a web page and capturing it once the data is available. The image is then uploaded to cloud storage.
    ///
    /// **Note**: When a snapshot is created, there is some delay before it is available.
    pub async fn get_graph_snapshot_with_http_info(
        &self,
        start: i64,
        end: i64,
        params: GetGraphSnapshotOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::GraphSnapshot>,
        datadog::Error<GetGraphSnapshotError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_graph_snapshot";

        // unbox and build optional parameters
        let metric_query = params.metric_query;
        let event_query = params.event_query;
        let graph_def = params.graph_def;
        let title = params.title;
        let height = params.height;
        let width = params.width;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/graph/snapshot",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = metric_query {
            local_req_builder =
                local_req_builder.query(&[("metric_query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = event_query {
            local_req_builder =
                local_req_builder.query(&[("event_query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = graph_def {
            local_req_builder =
                local_req_builder.query(&[("graph_def", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = title {
            local_req_builder =
                local_req_builder.query(&[("title", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = height {
            local_req_builder =
                local_req_builder.query(&[("height", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = width {
            local_req_builder =
                local_req_builder.query(&[("width", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::GraphSnapshot>(&local_content) {
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
            let local_entity: Option<GetGraphSnapshotError> =
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
