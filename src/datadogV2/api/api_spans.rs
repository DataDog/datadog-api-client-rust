// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListSpansGetOptionalParams is a struct for passing parameters to the method [`SpansAPI::list_spans_get`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSpansGetOptionalParams {
    /// Search query following spans syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested spans. Supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested spans. Supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    pub filter_to: Option<String>,
    /// Order of spans in results.
    pub sort: Option<crate::datadogV2::model::SpansSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of spans in the response.
    pub page_limit: Option<i32>,
}

impl ListSpansGetOptionalParams {
    /// Search query following spans syntax.
    pub fn filter_query(&mut self, value: String) -> &mut Self {
        self.filter_query = Some(value);
        self
    }
    /// Minimum timestamp for requested spans. Supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    pub fn filter_from(&mut self, value: String) -> &mut Self {
        self.filter_from = Some(value);
        self
    }
    /// Maximum timestamp for requested spans. Supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    pub fn filter_to(&mut self, value: String) -> &mut Self {
        self.filter_to = Some(value);
        self
    }
    /// Order of spans in results.
    pub fn sort(&mut self, value: crate::datadogV2::model::SpansSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// List following results with a cursor provided in the previous query.
    pub fn page_cursor(&mut self, value: String) -> &mut Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of spans in the response.
    pub fn page_limit(&mut self, value: i32) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
}

/// AggregateSpansError is a struct for typed errors of method [`SpansAPI::aggregate_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateSpansError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSpansError is a struct for typed errors of method [`SpansAPI::list_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSpansError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status422(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSpansGetError is a struct for typed errors of method [`SpansAPI::list_spans_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSpansGetError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status422(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SpansAPI {
    config: configuration::Configuration,
}

impl Default for SpansAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SpansAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// The API endpoint to aggregate spans into buckets and compute metrics and timeseries.
    /// This endpoint is rate limited to `300` requests per hour.
    pub async fn aggregate_spans(
        &self,
        body: crate::datadogV2::model::SpansAggregateRequest,
    ) -> Result<Option<crate::datadogV2::model::SpansAggregateResponse>, Error<AggregateSpansError>>
    {
        match self.aggregate_spans_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The API endpoint to aggregate spans into buckets and compute metrics and timeseries.
    /// This endpoint is rate limited to `300` requests per hour.
    pub async fn aggregate_spans_with_http_info(
        &self,
        body: crate::datadogV2::model::SpansAggregateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SpansAggregateResponse>,
        Error<AggregateSpansError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/spans/analytics/aggregate",
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
            let local_entity: Option<crate::datadogV2::model::SpansAggregateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<AggregateSpansError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns spans that match a span search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex spans filtering and search.
    /// This endpoint is rate limited to `300` requests per hour.
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination?tab=v2api
    pub async fn list_spans(
        &self,
        body: crate::datadogV2::model::SpansListRequest,
    ) -> Result<Option<crate::datadogV2::model::SpansListResponse>, Error<ListSpansError>> {
        match self.list_spans_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    pub fn list_spans_with_pagination(
        &self,
        mut body: crate::datadogV2::model::SpansListRequest,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::Span, Error<ListSpansError>>> + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if body.data.is_none() {
                body.data = Some(crate::datadogV2::model::SpansListRequestData::new());
            }
            if body.data.as_ref().unwrap().attributes.is_none() {
                body.data.as_mut().unwrap().attributes = Some(crate::datadogV2::model::SpansListRequestAttributes::new());
            }
            if body.data.as_ref().unwrap().attributes.as_ref().unwrap().page.is_none() {
                body.data.as_mut().unwrap().attributes.as_mut().unwrap().page = Some(crate::datadogV2::model::SpansListRequestPage::new());
            }
            if body.data.as_ref().unwrap().attributes.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                body.data.as_mut().unwrap().attributes.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = body.data.as_ref().unwrap().attributes.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.list_spans( body.clone(),).await?;

                let Some(resp) = resp else { break };
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

                body.data.as_mut().unwrap().attributes.as_mut().unwrap().page.as_mut().unwrap().cursor = Some(after);
            }
        }
    }

    /// List endpoint returns spans that match a span search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex spans filtering and search.
    /// This endpoint is rate limited to `300` requests per hour.
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination?tab=v2api
    pub async fn list_spans_with_http_info(
        &self,
        body: crate::datadogV2::model::SpansListRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::SpansListResponse>, Error<ListSpansError>>
    {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/spans/events/search",
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
            let local_entity: Option<crate::datadogV2::model::SpansListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListSpansError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns spans that match a span search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest spans.
    /// This endpoint is rate limited to `300` requests per hour.
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination?tab=v2api
    pub async fn list_spans_get(
        &self,
        params: ListSpansGetOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::SpansListResponse>, Error<ListSpansGetError>> {
        match self.list_spans_get_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    pub fn list_spans_get_with_pagination(
        &self,
        mut params: ListSpansGetOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::Span, Error<ListSpansGetError>>> + '_
    {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_spans_get(params.clone()).await?;

                let Some(resp) = resp else { break };
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

    /// List endpoint returns spans that match a span search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest spans.
    /// This endpoint is rate limited to `300` requests per hour.
    ///
    /// [1]: /logs/guide/collect-multiple-logs-with-pagination?tab=v2api
    pub async fn list_spans_get_with_http_info(
        &self,
        params: ListSpansGetOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::SpansListResponse>, Error<ListSpansGetError>>
    {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/spans/events", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::SpansListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListSpansGetError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
