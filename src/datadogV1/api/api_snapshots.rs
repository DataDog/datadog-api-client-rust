// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetGraphSnapshotParams is a struct for passing parameters to the method [`SnapshotsAPI::get_graph_snapshot`]
#[derive(Clone, Debug)]
pub struct GetGraphSnapshotParams {
    /// The POSIX timestamp of the start of the query in seconds.
    pub start: i64,
    /// The POSIX timestamp of the end of the query in seconds.
    pub end: i64,
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

/// GetGraphSnapshotError is a struct for typed errors of method [`SnapshotsAPI::get_graph_snapshot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGraphSnapshotError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SnapshotsAPI {
    config: configuration::Configuration,
}

impl Default for SnapshotsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SnapshotsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Take graph snapshots.
    /// **Note**: When a snapshot is created, there is some delay before it is available.
    pub async fn get_graph_snapshot(
        &self,
        params: GetGraphSnapshotParams,
    ) -> Result<Option<crate::datadogV1::model::GraphSnapshot>, Error<GetGraphSnapshotError>> {
        match self.get_graph_snapshot_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Take graph snapshots.
    /// **Note**: When a snapshot is created, there is some delay before it is available.
    pub async fn get_graph_snapshot_with_http_info(
        &self,
        params: GetGraphSnapshotParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::GraphSnapshot>, Error<GetGraphSnapshotError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start = params.start;
        let end = params.end;
        let metric_query = params.metric_query;
        let event_query = params.event_query;
        let graph_def = params.graph_def;
        let title = params.title;
        let height = params.height;
        let width = params.width;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/graph/snapshot", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_str) = metric_query {
            local_req_builder =
                local_req_builder.query(&[("metric_query", &local_str.to_string())]);
        };
        if let Some(ref local_str) = event_query {
            local_req_builder = local_req_builder.query(&[("event_query", &local_str.to_string())]);
        };
        if let Some(ref local_str) = graph_def {
            local_req_builder = local_req_builder.query(&[("graph_def", &local_str.to_string())]);
        };
        if let Some(ref local_str) = title {
            local_req_builder = local_req_builder.query(&[("title", &local_str.to_string())]);
        };
        if let Some(ref local_str) = height {
            local_req_builder = local_req_builder.query(&[("height", &local_str.to_string())]);
        };
        if let Some(ref local_str) = width {
            local_req_builder = local_req_builder.query(&[("width", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::GraphSnapshot> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetGraphSnapshotError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
