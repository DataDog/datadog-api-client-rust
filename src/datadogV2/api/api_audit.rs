// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListAuditLogsOptionalParams is a struct for passing parameters to the method [`AuditAPI::list_audit_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAuditLogsOptionalParams {
    /// Search query following Audit Logs syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<chrono::DateTime<chrono::Utc>>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<chrono::DateTime<chrono::Utc>>,
    /// Order of events in results.
    pub sort: Option<crate::datadogV2::model::AuditLogsSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page_limit: Option<i32>,
}

impl ListAuditLogsOptionalParams {
    /// Search query following Audit Logs syntax.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Minimum timestamp for requested events.
    pub fn filter_from(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// Maximum timestamp for requested events.
    pub fn filter_to(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// Order of events in results.
    pub fn sort(mut self, value: crate::datadogV2::model::AuditLogsSort) -> Self {
        self.sort = Some(value);
        self
    }
    /// List following results with a cursor provided in the previous query.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of events in the response.
    pub fn page_limit(mut self, value: i32) -> Self {
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
    pub fn body(mut self, value: crate::datadogV2::model::AuditLogsSearchEventsRequest) -> Self {
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
    ) -> Result<crate::datadogV2::model::AuditLogsEventsResponse, Error<ListAuditLogsError>> {
        match self.list_audit_logs_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_audit_logs_with_pagination(
        &self,
        mut params: ListAuditLogsOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::AuditLogsEvent, Error<ListAuditLogsError>>>
           + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_audit_logs(params.clone()).await?;
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
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::AuditLogsEventsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
    ) -> Result<crate::datadogV2::model::AuditLogsEventsResponse, Error<SearchAuditLogsError>> {
        match self.search_audit_logs_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn search_audit_logs_with_pagination(
        &self,
        mut params: SearchAuditLogsOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::AuditLogsEvent, Error<SearchAuditLogsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.body.is_none() {
                params.body = Some(crate::datadogV2::model::AuditLogsSearchEventsRequest::new());
            }
            if params.body.as_ref().unwrap().page.is_none() {
                params.body.as_mut().unwrap().page = Some(crate::datadogV2::model::AuditLogsQueryPageOptions::new());
            }
            if params.body.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                params.body.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = params.body.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.search_audit_logs(params.clone()).await?;
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

                params.body.as_mut().unwrap().page.as_mut().unwrap().cursor = Some(after);
            }
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
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<crate::datadogV2::model::AuditLogsEventsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
