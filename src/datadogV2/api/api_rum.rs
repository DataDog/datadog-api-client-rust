// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// AggregateRUMEventsParams is a struct for passing parameters to the method [`AggregateRUMEvents`]
#[derive(Clone, Debug)]
pub struct AggregateRUMEventsParams {
    pub body: crate::datadogV2::model::RUMAggregateRequest,
}

/// CreateRUMApplicationParams is a struct for passing parameters to the method [`CreateRUMApplication`]
#[derive(Clone, Debug)]
pub struct CreateRUMApplicationParams {
    pub body: crate::datadogV2::model::RUMApplicationCreateRequest,
}

/// DeleteRUMApplicationParams is a struct for passing parameters to the method [`DeleteRUMApplication`]
#[derive(Clone, Debug)]
pub struct DeleteRUMApplicationParams {
    /// RUM application ID.
    pub id: String,
}

/// GetRUMApplicationParams is a struct for passing parameters to the method [`GetRUMApplication`]
#[derive(Clone, Debug)]
pub struct GetRUMApplicationParams {
    /// RUM application ID.
    pub id: String,
}

/// ListRUMEventsParams is a struct for passing parameters to the method [`ListRUMEvents`]
#[derive(Clone, Debug)]
pub struct ListRUMEventsParams {
    /// Search query following RUM syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<String>,
    /// Order of events in results.
    pub sort: Option<crate::datadogV2::model::RUMSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page_limit: Option<i32>,
}

/// SearchRUMEventsParams is a struct for passing parameters to the method [`SearchRUMEvents`]
#[derive(Clone, Debug)]
pub struct SearchRUMEventsParams {
    pub body: crate::datadogV2::model::RUMSearchEventsRequest,
}

/// UpdateRUMApplicationParams is a struct for passing parameters to the method [`UpdateRUMApplication`]
#[derive(Clone, Debug)]
pub struct UpdateRUMApplicationParams {
    /// RUM application ID.
    pub id: String,
    pub body: crate::datadogV2::model::RUMApplicationUpdateRequest,
}

/// AggregateRUMEventsError is a struct for typed errors of method [`AggregateRUMEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateRUMEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateRUMApplicationError is a struct for typed errors of method [`CreateRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRUMApplicationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteRUMApplicationError is a struct for typed errors of method [`DeleteRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRUMApplicationError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetRUMApplicationError is a struct for typed errors of method [`GetRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMApplicationError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetRUMApplicationsError is a struct for typed errors of method [`GetRUMApplications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMApplicationsError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListRUMEventsError is a struct for typed errors of method [`ListRUMEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRUMEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchRUMEventsError is a struct for typed errors of method [`SearchRUMEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchRUMEventsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateRUMApplicationError is a struct for typed errors of method [`UpdateRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRUMApplicationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status422(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct RumAPI {
    config: configuration::Configuration,
}

impl Default for RumAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl RumAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// The API endpoint to aggregate RUM events into buckets of computed metrics and timeseries.
    pub async fn aggregate_rum_events(
        &self,
        params: AggregateRUMEventsParams,
    ) -> Result<
        Option<crate::datadogV2::model::RUMAnalyticsAggregateResponse>,
        Error<AggregateRUMEventsError>,
    > {
        match self.aggregate_rum_events_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The API endpoint to aggregate RUM events into buckets of computed metrics and timeseries.
    pub async fn aggregate_rum_events_with_http_info(
        &self,
        params: AggregateRUMEventsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RUMAnalyticsAggregateResponse>,
        Error<AggregateRUMEventsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/analytics/aggregate",
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
            let local_entity: Option<crate::datadogV2::model::RUMAnalyticsAggregateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<AggregateRUMEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a new RUM application in your organization.
    pub async fn create_rum_application(
        &self,
        params: CreateRUMApplicationParams,
    ) -> Result<
        Option<crate::datadogV2::model::RUMApplicationResponse>,
        Error<CreateRUMApplicationError>,
    > {
        match self.create_rum_application_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a new RUM application in your organization.
    pub async fn create_rum_application_with_http_info(
        &self,
        params: CreateRUMApplicationParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RUMApplicationResponse>,
        Error<CreateRUMApplicationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/rum/applications", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::RUMApplicationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateRUMApplicationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an existing RUM application in your organization.
    pub async fn delete_rum_application(
        &self,
        params: DeleteRUMApplicationParams,
    ) -> Result<Option<()>, Error<DeleteRUMApplicationError>> {
        match self.delete_rum_application_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an existing RUM application in your organization.
    pub async fn delete_rum_application_with_http_info(
        &self,
        params: DeleteRUMApplicationParams,
    ) -> Result<ResponseContent<()>, Error<DeleteRUMApplicationError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let id = params.id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{id}",
            local_configuration.base_path,
            id = urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteRUMApplicationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the RUM application with given ID in your organization.
    pub async fn get_rum_application(
        &self,
        params: GetRUMApplicationParams,
    ) -> Result<
        Option<crate::datadogV2::model::RUMApplicationResponse>,
        Error<GetRUMApplicationError>,
    > {
        match self.get_rum_application_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the RUM application with given ID in your organization.
    pub async fn get_rum_application_with_http_info(
        &self,
        params: GetRUMApplicationParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RUMApplicationResponse>,
        Error<GetRUMApplicationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let id = params.id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{id}",
            local_configuration.base_path,
            id = urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::RUMApplicationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetRUMApplicationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all the RUM applications in your organization.
    pub async fn get_rum_applications(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::RUMApplicationsResponse>,
        Error<GetRUMApplicationsError>,
    > {
        match self.get_rum_applications_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all the RUM applications in your organization.
    pub async fn get_rum_applications_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RUMApplicationsResponse>,
        Error<GetRUMApplicationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/rum/applications", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::RUMApplicationsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetRUMApplicationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest RUM events.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn list_rum_events(
        &self,
        params: ListRUMEventsParams,
    ) -> Result<Option<crate::datadogV2::model::RUMEventsResponse>, Error<ListRUMEventsError>> {
        match self.list_rum_events_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest RUM events.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn list_rum_events_with_http_info(
        &self,
        params: ListRUMEventsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RUMEventsResponse>,
        Error<ListRUMEventsError>,
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

        let local_uri_str = format!("{}/api/v2/rum/events", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::RUMEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListRUMEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns RUM events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex RUM events filtering and search.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn search_rum_events(
        &self,
        params: SearchRUMEventsParams,
    ) -> Result<Option<crate::datadogV2::model::RUMEventsResponse>, Error<SearchRUMEventsError>>
    {
        match self.search_rum_events_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List endpoint returns RUM events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex RUM events filtering and search.
    ///
    /// [1]: https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination
    pub async fn search_rum_events_with_http_info(
        &self,
        params: SearchRUMEventsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RUMEventsResponse>,
        Error<SearchRUMEventsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/rum/events/search", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::RUMEventsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SearchRUMEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the RUM application with given ID in your organization.
    pub async fn update_rum_application(
        &self,
        params: UpdateRUMApplicationParams,
    ) -> Result<
        Option<crate::datadogV2::model::RUMApplicationResponse>,
        Error<UpdateRUMApplicationError>,
    > {
        match self.update_rum_application_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the RUM application with given ID in your organization.
    pub async fn update_rum_application_with_http_info(
        &self,
        params: UpdateRUMApplicationParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RUMApplicationResponse>,
        Error<UpdateRUMApplicationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let id = params.id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{id}",
            local_configuration.base_path,
            id = urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::RUMApplicationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateRUMApplicationError> =
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
