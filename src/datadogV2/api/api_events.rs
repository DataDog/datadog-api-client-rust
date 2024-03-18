// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListEventsOptionalParams is a struct for passing parameters to the method [`EventsAPI::list_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListEventsOptionalParams {
    /// Search query following events syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<String>,
    /// Order of events in results.
    pub sort: Option<crate::datadogV2::model::EventsSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page_limit: Option<i32>,
}

impl ListEventsOptionalParams {
    /// Search query following events syntax.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Minimum timestamp for requested events.
    pub fn filter_from(mut self, value: String) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// Maximum timestamp for requested events.
    pub fn filter_to(mut self, value: String) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// Order of events in results.
    pub fn sort(mut self, value: crate::datadogV2::model::EventsSort) -> Self {
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

/// SearchEventsOptionalParams is a struct for passing parameters to the method [`EventsAPI::search_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchEventsOptionalParams {
    pub body: Option<crate::datadogV2::model::EventsListRequest>,
}

impl SearchEventsOptionalParams {
    pub fn body(mut self, value: crate::datadogV2::model::EventsListRequest) -> Self {
        self.body = Some(value);
        self
    }
}

/// ListEventsError is a struct for typed errors of method [`EventsAPI::list_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchEventsError is a struct for typed errors of method [`EventsAPI::search_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct EventsAPI {
    config: configuration::Configuration,
}

impl Default for EventsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl EventsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// List endpoint returns events that match an events search query.
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest events.
    pub async fn list_events(
        &self,
        params: ListEventsOptionalParams,
    ) -> Result<crate::datadogV2::model::EventsListResponse, Error<ListEventsError>> {
        match self.list_events_with_http_info(params).await {
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

    pub fn list_events_with_pagination(
        &self,
        mut params: ListEventsOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::EventResponse, Error<ListEventsError>>> + '_
    {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_events(params.clone()).await?;
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

    /// List endpoint returns events that match an events search query.
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest events.
    pub async fn list_events_with_http_info(
        &self,
        params: ListEventsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::EventsListResponse>, Error<ListEventsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.list_events";

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/events",
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
            match serde_json::from_str::<crate::datadogV2::model::EventsListResponse>(
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
            let local_entity: Option<ListEventsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns events that match an events search query.
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_events(
        &self,
        params: SearchEventsOptionalParams,
    ) -> Result<crate::datadogV2::model::EventsListResponse, Error<SearchEventsError>> {
        match self.search_events_with_http_info(params).await {
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

    pub fn search_events_with_pagination(
        &self,
        mut params: SearchEventsOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::EventResponse, Error<SearchEventsError>>> + '_
    {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.body.is_none() {
                params.body = Some(crate::datadogV2::model::EventsListRequest::new());
            }
            if params.body.as_ref().unwrap().page.is_none() {
                params.body.as_mut().unwrap().page = Some(crate::datadogV2::model::EventsRequestPage::new());
            }
            if params.body.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                params.body.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = params.body.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.search_events(params.clone()).await?;
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

    /// List endpoint returns events that match an events search query.
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_events_with_http_info(
        &self,
        params: SearchEventsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::EventsListResponse>,
        Error<SearchEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_events";

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/events/search",
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
            match serde_json::from_str::<crate::datadogV2::model::EventsListResponse>(
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
            let local_entity: Option<SearchEventsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
