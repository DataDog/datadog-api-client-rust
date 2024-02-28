// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListCIAppTestEventsOptionalParams is a struct for passing parameters to the method [`CIVisibilityTestsAPI::list_ci_app_test_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCIAppTestEventsOptionalParams {
    /// Search query following log syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<String>,
    /// Order of events in results.
    pub sort: Option<crate::datadogV2::model::CIAppSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page_limit: Option<i32>,
}

impl ListCIAppTestEventsOptionalParams {
    /// Search query following log syntax.
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
    pub fn sort(&mut self, value: crate::datadogV2::model::CIAppSort) -> &mut Self {
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

/// SearchCIAppTestEventsOptionalParams is a struct for passing parameters to the method [`CIVisibilityTestsAPI::search_ci_app_test_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchCIAppTestEventsOptionalParams {
    pub body: Option<crate::datadogV2::model::CIAppTestEventsRequest>,
}

impl SearchCIAppTestEventsOptionalParams {
    pub fn body(&mut self, value: crate::datadogV2::model::CIAppTestEventsRequest) -> &mut Self {
        self.body = Some(value);
        self
    }
}

/// AggregateCIAppTestEventsError is a struct for typed errors of method [`CIVisibilityTestsAPI::aggregate_ci_app_test_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateCIAppTestEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListCIAppTestEventsError is a struct for typed errors of method [`CIVisibilityTestsAPI::list_ci_app_test_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCIAppTestEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchCIAppTestEventsError is a struct for typed errors of method [`CIVisibilityTestsAPI::search_ci_app_test_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCIAppTestEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct CIVisibilityTestsAPI {
    config: configuration::Configuration,
}

impl Default for CIVisibilityTestsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl CIVisibilityTestsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// The API endpoint to aggregate CI Visibility test events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_test_events(
        &self,
        body: crate::datadogV2::model::CIAppTestsAggregateRequest,
    ) -> Result<
        crate::datadogV2::model::CIAppTestsAnalyticsAggregateResponse,
        Error<AggregateCIAppTestEventsError>,
    > {
        match self.aggregate_ci_app_test_events_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// The API endpoint to aggregate CI Visibility test events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_test_events_with_http_info(
        &self,
        body: crate::datadogV2::model::CIAppTestsAggregateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppTestsAnalyticsAggregateResponse>,
        Error<AggregateCIAppTestEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.aggregate_ci_app_test_events";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/tests/analytics/aggregate",
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
            match serde_json::from_str::<
                crate::datadogV2::model::CIAppTestsAnalyticsAggregateResponse,
            >(&local_content)
            {
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
            let local_entity: Option<AggregateCIAppTestEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns CI Visibility test events that match a [log search query](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest test events.
    pub async fn list_ci_app_test_events(
        &self,
        params: ListCIAppTestEventsOptionalParams,
    ) -> Result<crate::datadogV2::model::CIAppTestEventsResponse, Error<ListCIAppTestEventsError>>
    {
        match self.list_ci_app_test_events_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns CI Visibility test events that match a [log search query](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest test events.
    pub async fn list_ci_app_test_events_with_http_info(
        &self,
        params: ListCIAppTestEventsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppTestEventsResponse>,
        Error<ListCIAppTestEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_ci_app_test_events";

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/tests/events",
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
            match serde_json::from_str::<crate::datadogV2::model::CIAppTestEventsResponse>(
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
            let local_entity: Option<ListCIAppTestEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns CI Visibility test events that match a [log search query](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_test_events(
        &self,
        params: SearchCIAppTestEventsOptionalParams,
    ) -> Result<crate::datadogV2::model::CIAppTestEventsResponse, Error<SearchCIAppTestEventsError>>
    {
        match self.search_ci_app_test_events_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns CI Visibility test events that match a [log search query](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_test_events_with_http_info(
        &self,
        params: SearchCIAppTestEventsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppTestEventsResponse>,
        Error<SearchCIAppTestEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_ci_app_test_events";

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/tests/events/search",
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
            match serde_json::from_str::<crate::datadogV2::model::CIAppTestEventsResponse>(
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
            let local_entity: Option<SearchCIAppTestEventsError> =
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
