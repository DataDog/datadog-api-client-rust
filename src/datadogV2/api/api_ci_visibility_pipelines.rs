// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListCIAppPipelineEventsOptionalParams is a struct for passing parameters to the method [`CIVisibilityPipelinesAPI::list_ci_app_pipeline_events`]
#[derive(Clone, Default, Debug)]
pub struct ListCIAppPipelineEventsOptionalParams {
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

impl ListCIAppPipelineEventsOptionalParams {
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

/// SearchCIAppPipelineEventsOptionalParams is a struct for passing parameters to the method [`CIVisibilityPipelinesAPI::search_ci_app_pipeline_events`]
#[derive(Clone, Default, Debug)]
pub struct SearchCIAppPipelineEventsOptionalParams {
    pub body: Option<crate::datadogV2::model::CIAppPipelineEventsRequest>,
}

impl SearchCIAppPipelineEventsOptionalParams {
    pub fn body(
        &mut self,
        value: crate::datadogV2::model::CIAppPipelineEventsRequest,
    ) -> &mut Self {
        self.body = Some(value);
        self
    }
}

/// AggregateCIAppPipelineEventsError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::aggregate_ci_app_pipeline_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateCIAppPipelineEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateCIAppPipelineEventError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::create_ci_app_pipeline_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCIAppPipelineEventError {
    Status400(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    Status401(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    Status403(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    Status408(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    Status413(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    Status429(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    Status500(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    Status503(Option<crate::datadogV2::model::HTTPCIAppErrors>),
    UnknownValue(serde_json::Value),
}

/// ListCIAppPipelineEventsError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::list_ci_app_pipeline_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCIAppPipelineEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchCIAppPipelineEventsError is a struct for typed errors of method [`CIVisibilityPipelinesAPI::search_ci_app_pipeline_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCIAppPipelineEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct CIVisibilityPipelinesAPI {
    config: configuration::Configuration,
}

impl Default for CIVisibilityPipelinesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl CIVisibilityPipelinesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Use this API endpoint to aggregate CI Visibility pipeline events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_pipeline_events(
        &self,
        body: crate::datadogV2::model::CIAppPipelinesAggregateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::CIAppPipelinesAnalyticsAggregateResponse>,
        Error<AggregateCIAppPipelineEventsError>,
    > {
        match self
            .aggregate_ci_app_pipeline_events_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Use this API endpoint to aggregate CI Visibility pipeline events into buckets of computed metrics and timeseries.
    pub async fn aggregate_ci_app_pipeline_events_with_http_info(
        &self,
        body: crate::datadogV2::model::CIAppPipelinesAggregateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppPipelinesAnalyticsAggregateResponse>,
        Error<AggregateCIAppPipelineEventsError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/pipelines/analytics/aggregate",
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
                crate::datadogV2::model::CIAppPipelinesAnalyticsAggregateResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<AggregateCIAppPipelineEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Send your pipeline event to your Datadog platform over HTTP. For details about how pipeline executions are modeled and what execution types we support, see [Pipeline Data Model And Execution Types](<https://docs.datadoghq.com/continuous_integration/guides/pipeline_data_model/>).
    ///
    /// Pipeline events can be submitted with a timestamp that is up to 18 hours in the past.
    pub async fn create_ci_app_pipeline_event(
        &self,
        body: crate::datadogV2::model::CIAppCreatePipelineEventRequest,
    ) -> Result<
        Option<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<CreateCIAppPipelineEventError>,
    > {
        match self.create_ci_app_pipeline_event_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Send your pipeline event to your Datadog platform over HTTP. For details about how pipeline executions are modeled and what execution types we support, see [Pipeline Data Model And Execution Types](<https://docs.datadoghq.com/continuous_integration/guides/pipeline_data_model/>).
    ///
    /// Pipeline events can be submitted with a timestamp that is up to 18 hours in the past.
    pub async fn create_ci_app_pipeline_event_with_http_info(
        &self,
        body: crate::datadogV2::model::CIAppCreatePipelineEventRequest,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<CreateCIAppPipelineEventError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/ci/pipeline", local_configuration.base_path);
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
            let local_entity: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateCIAppPipelineEventError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest pipeline events.
    pub async fn list_ci_app_pipeline_events(
        &self,
        params: ListCIAppPipelineEventsOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::CIAppPipelineEventsResponse>,
        Error<ListCIAppPipelineEventsError>,
    > {
        match self
            .list_ci_app_pipeline_events_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to see your latest pipeline events.
    pub async fn list_ci_app_pipeline_events_with_http_info(
        &self,
        params: ListCIAppPipelineEventsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppPipelineEventsResponse>,
        Error<ListCIAppPipelineEventsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/pipelines/events",
            local_configuration.base_path
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
            let local_entity: Option<crate::datadogV2::model::CIAppPipelineEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListCIAppPipelineEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_pipeline_events(
        &self,
        params: SearchCIAppPipelineEventsOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::CIAppPipelineEventsResponse>,
        Error<SearchCIAppPipelineEventsError>,
    > {
        match self
            .search_ci_app_pipeline_events_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns CI Visibility pipeline events that match a [search query](<https://docs.datadoghq.com/continuous_integration/explorer/search_syntax/>).
    /// [Results are paginated similarly to logs](<https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>).
    ///
    /// Use this endpoint to build complex events filtering and search.
    pub async fn search_ci_app_pipeline_events_with_http_info(
        &self,
        params: SearchCIAppPipelineEventsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CIAppPipelineEventsResponse>,
        Error<SearchCIAppPipelineEventsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/ci/pipelines/events/search",
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
            let local_entity: Option<crate::datadogV2::model::CIAppPipelineEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SearchCIAppPipelineEventsError> =
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
