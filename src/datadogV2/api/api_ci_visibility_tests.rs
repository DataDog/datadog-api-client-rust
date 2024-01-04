// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// AggregateCIAppTestEventsParams is a struct for passing parameters to the method [`AggregateCIAppTestEvents`]
#[derive(Clone, Debug)]
pub struct AggregateCIAppTestEventsParams {
    pub body: crate::datadogV2::model::CIAppTestsAggregateRequest,
}

/// ListCIAppTestEventsParams is a struct for passing parameters to the method [`ListCIAppTestEvents`]
#[derive(Clone, Debug)]
pub struct ListCIAppTestEventsParams {
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

/// SearchCIAppTestEventsParams is a struct for passing parameters to the method [`SearchCIAppTestEvents`]
#[derive(Clone, Debug)]
pub struct SearchCIAppTestEventsParams {
    pub body: Option<Option<crate::datadogV2::model::CIAppTestEventsRequest>>,
}

/// AggregateCIAppTestEventsError is a struct for typed errors of method [`AggregateCIAppTestEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateCIAppTestEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListCIAppTestEventsError is a struct for typed errors of method [`ListCIAppTestEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCIAppTestEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchCIAppTestEventsError is a struct for typed errors of method [`SearchCIAppTestEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCIAppTestEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct CiVisibilityTestsAPI {
    config: configuration::Configuration,
}

impl Default for CiVisibilityTestsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl CiVisibilityTestsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// The API endpoint to aggregate CI Visibility test events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_test_events(
        &self,
        params: AggregateCIAppTestEventsParams,
    ) -> Result<
        Option<crate::datadogV2::model::CIAppTestsAnalyticsAggregateResponse>,
        Error<AggregateCIAppTestEventsError>,
    > {
        match self
            .aggregate_ci_app_test_events_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The API endpoint to aggregate CI Visibility test events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_test_events_with_http_info(
        &self,
        params: AggregateCIAppTestEventsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppTestsAnalyticsAggregateResponse>,
        Error<AggregateCIAppTestEventsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/tests/analytics/aggregate",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<
                crate::datadogV2::model::CIAppTestsAnalyticsAggregateResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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

    /// List endpoint returns CI Visibility test events that match a [log search query](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    /// [Results are paginated similarly to logs](https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination).
    ///
    /// Use this endpoint to see your latest test events.
    pub async fn list_ci_app_test_events(
        &self,
        params: ListCIAppTestEventsParams,
    ) -> Result<
        Option<crate::datadogV2::model::CIAppTestEventsResponse>,
        Error<ListCIAppTestEventsError>,
    > {
        match self.list_ci_app_test_events_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns CI Visibility test events that match a [log search query](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    /// [Results are paginated similarly to logs](https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination).
    ///
    /// Use this endpoint to see your latest test events.
    pub async fn list_ci_app_test_events_with_http_info(
        &self,
        params: ListCIAppTestEventsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppTestEventsResponse>,
        Error<ListCIAppTestEventsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/ci/tests/events", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::CIAppTestEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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

    /// List endpoint returns CI Visibility test events that match a [log search query](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    /// [Results are paginated similarly to logs](https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_test_events(
        &self,
        params: SearchCIAppTestEventsParams,
    ) -> Result<
        Option<crate::datadogV2::model::CIAppTestEventsResponse>,
        Error<SearchCIAppTestEventsError>,
    > {
        match self.search_ci_app_test_events_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns CI Visibility test events that match a [log search query](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    /// [Results are paginated similarly to logs](https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_test_events_with_http_info(
        &self,
        params: SearchCIAppTestEventsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppTestEventsResponse>,
        Error<SearchCIAppTestEventsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/tests/events/search",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::CIAppTestEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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