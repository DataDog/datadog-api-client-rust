// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListAuditLogsOptionalParams is a struct for passing parameters to the method [`AuditAPI::list_audit_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAuditLogsOptionalParams {
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

impl ListAuditLogsOptionalParams {
    /// Search query following Audit Logs syntax.
    pub fn filter_query(&mut self, value: String) -> &mut Self {
        self.filter_query = Some(value);
        self
    }
    /// Minimum timestamp for requested events.
    pub fn filter_from(&mut self, value: String) -> &mut Self {
        self.filter_from = Some(value);
        self
    }
    /// Maximum timestamp for requested events.
    pub fn filter_to(&mut self, value: String) -> &mut Self {
        self.filter_to = Some(value);
        self
    }
    /// Order of events in results.
    pub fn sort(&mut self, value: crate::datadogV2::model::AuditLogsSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// List following results with a cursor provided in the previous query.
    pub fn page_cursor(&mut self, value: String) -> &mut Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of events in the response.
    pub fn page_limit(&mut self, value: i32) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
}

/// SearchAuditLogsOptionalParams is a struct for passing parameters to the method [`AuditAPI::search_audit_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchAuditLogsOptionalParams {
    pub body: Option<crate::datadogV2::model::AuditLogsSearchEventsRequest>,
}

impl SearchAuditLogsOptionalParams {
    pub fn body(
        &mut self,
        value: crate::datadogV2::model::AuditLogsSearchEventsRequest,
    ) -> &mut Self {
        self.body = Some(value);
        self
    }
}

/// ListAuditLogsError is a struct for typed errors of method [`AuditAPI::list_audit_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAuditLogsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchAuditLogsError is a struct for typed errors of method [`AuditAPI::search_audit_logs`]
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
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn list_audit_logs(
        &self,
        params: ListAuditLogsOptionalParams,
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
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn list_audit_logs_with_http_info(
        &self,
        params: ListAuditLogsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuditLogsEventsResponse>,
        Error<ListAuditLogsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_audit_logs";

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/audit/events",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder =
                local_req_builder.query(&[("filter[to]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn search_audit_logs(
        &self,
        params: SearchAuditLogsOptionalParams,
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
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn search_audit_logs_with_http_info(
        &self,
        params: SearchAuditLogsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuditLogsEventsResponse>,
        Error<SearchAuditLogsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_audit_logs";

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/audit/events/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
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
