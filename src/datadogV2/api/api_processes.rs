// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListProcessesOptionalParams is a struct for passing parameters to the method [`ProcessesAPI::list_processes`]
#[derive(Clone, Debug)]
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

/// ListProcessesError is a struct for typed errors of method [`ProcessesAPI::list_processes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListProcessesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ProcessesAPI {
    config: configuration::Configuration,
}

impl Default for ProcessesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ProcessesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Get all processes for your organization.
    pub async fn list_processes(
        &self,
        params: ListProcessesOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::ProcessSummariesResponse>, Error<ListProcessesError>>
    {
        match self.list_processes_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all processes for your organization.
    pub async fn list_processes_with_http_info(
        &self,
        params: ListProcessesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ProcessSummariesResponse>,
        Error<ListProcessesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let search = params.search;
        let tags = params.tags;
        let from = params.from;
        let to = params.to;
        let page_limit = params.page_limit;
        let page_cursor = params.page_cursor;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/processes", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::ProcessSummariesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListProcessesError> =
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
