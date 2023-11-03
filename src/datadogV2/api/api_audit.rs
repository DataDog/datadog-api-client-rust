// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListAuditLogsParams is a struct for passing parameters to the method [`ListAuditLogs`]
#[derive(Clone, Debug, Default)]
pub struct ListAuditLogsParams {
    /// Search query following Audit Logs syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<String>,
    /// Order of events in results.
    pub sort: Option<crate::datadogV2::model::AuditLogsSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page_limit: Option<i32>,
}

/// SearchAuditLogsParams is a struct for passing parameters to the method [`SearchAuditLogs`]
#[derive(Clone, Debug, Default)]
pub struct SearchAuditLogsParams {
    pub body: Option<Option<crate::datadogV2::model::AuditLogsSearchEventsRequest>>,
}

/// ListAuditLogsError is a struct for typed errors of method [`ListAuditLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAuditLogsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchAuditLogsError is a struct for typed errors of method [`SearchAuditLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchAuditLogsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct AuditAPI {
    config: configuration::Configuration,
}

impl Default for AuditAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl AuditAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// List endpoint returns events that match a Audit Logs search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest Audit Logs events.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn list_audit_logs(
        &self,
        params: ListAuditLogsParams,
    ) -> Result<Option<crate::datadogV2::model::AuditLogsEventsResponse>, Error<ListAuditLogsError>>
    {
        match self.list_audit_logs_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns events that match a Audit Logs search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest Audit Logs events.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn list_audit_logs_with_http_info(
        &self,
        params: ListAuditLogsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuditLogsEventsResponse>,
        Error<ListAuditLogsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/audit/events", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build parameters
        if let Some(ref local_str) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_to {
            local_req_builder = local_req_builder.query(&[("filter[to]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_limit {
            local_req_builder = local_req_builder.query(&[("page[limit]", &local_str.to_string())]);
        };

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV2::model::AuditLogsEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListAuditLogsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns Audit Logs events that match an Audit search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex Audit Logs events filtering and search.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn search_audit_logs(
        &self,
        params: SearchAuditLogsParams,
    ) -> Result<Option<crate::datadogV2::model::AuditLogsEventsResponse>, Error<SearchAuditLogsError>>
    {
        match self.search_audit_logs_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns Audit Logs events that match an Audit search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex Audit Logs events filtering and search.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn search_audit_logs_with_http_info(
        &self,
        params: SearchAuditLogsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuditLogsEventsResponse>,
        Error<SearchAuditLogsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/audit/events/search",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build parameters

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        if body.is_some() {
            local_req_builder = local_req_builder.json(&body.unwrap());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::AuditLogsEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SearchAuditLogsError> =
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
