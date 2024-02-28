// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetDowntimeOptionalParams is a struct for passing parameters to the method [`DowntimesAPI::get_downtime`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetDowntimeOptionalParams {
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub include: Option<String>,
}

impl GetDowntimeOptionalParams {
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub fn include(&mut self, value: String) -> &mut Self {
        self.include = Some(value);
        self
    }
}

/// ListDowntimesOptionalParams is a struct for passing parameters to the method [`DowntimesAPI::list_downtimes`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListDowntimesOptionalParams {
    /// Only return downtimes that are active when the request is made.
    pub current_only: Option<bool>,
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub include: Option<String>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Maximum number of downtimes in the response.
    pub page_limit: Option<i64>,
}

impl ListDowntimesOptionalParams {
    /// Only return downtimes that are active when the request is made.
    pub fn current_only(&mut self, value: bool) -> &mut Self {
        self.current_only = Some(value);
        self
    }
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub fn include(&mut self, value: String) -> &mut Self {
        self.include = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(&mut self, value: i64) -> &mut Self {
        self.page_offset = Some(value);
        self
    }
    /// Maximum number of downtimes in the response.
    pub fn page_limit(&mut self, value: i64) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListMonitorDowntimesOptionalParams is a struct for passing parameters to the method [`DowntimesAPI::list_monitor_downtimes`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListMonitorDowntimesOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Maximum number of downtimes in the response.
    pub page_limit: Option<i64>,
}

impl ListMonitorDowntimesOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(&mut self, value: i64) -> &mut Self {
        self.page_offset = Some(value);
        self
    }
    /// Maximum number of downtimes in the response.
    pub fn page_limit(&mut self, value: i64) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
}

/// CancelDowntimeError is a struct for typed errors of method [`DowntimesAPI::cancel_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDowntimeError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateDowntimeError is a struct for typed errors of method [`DowntimesAPI::create_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDowntimeError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDowntimeError is a struct for typed errors of method [`DowntimesAPI::get_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDowntimeError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListDowntimesError is a struct for typed errors of method [`DowntimesAPI::list_downtimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDowntimesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListMonitorDowntimesError is a struct for typed errors of method [`DowntimesAPI::list_monitor_downtimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMonitorDowntimesError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDowntimeError is a struct for typed errors of method [`DowntimesAPI::update_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDowntimeError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct DowntimesAPI {
    config: configuration::Configuration,
}

impl Default for DowntimesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl DowntimesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Cancel a downtime.
    pub async fn cancel_downtime(
        &self,
        downtime_id: String,
    ) -> Result<Option<()>, Error<CancelDowntimeError>> {
        match self.cancel_downtime_with_http_info(downtime_id).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Cancel a downtime.
    pub async fn cancel_downtime_with_http_info(
        &self,
        downtime_id: String,
    ) -> Result<ResponseContent<()>, Error<CancelDowntimeError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.cancel_downtime";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime/{downtime_id}",
            local_configuration.get_operation_host(operation_id),
            downtime_id = urlencode(downtime_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<CancelDowntimeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Schedule a downtime.
    pub async fn create_downtime(
        &self,
        body: crate::datadogV2::model::DowntimeCreateRequest,
    ) -> Result<Option<crate::datadogV2::model::DowntimeResponse>, Error<CreateDowntimeError>> {
        match self.create_downtime_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Schedule a downtime.
    pub async fn create_downtime_with_http_info(
        &self,
        body: crate::datadogV2::model::DowntimeCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DowntimeResponse>,
        Error<CreateDowntimeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_downtime";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime",
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
            let local_entity: Option<crate::datadogV2::model::DowntimeResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateDowntimeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get downtime detail by `downtime_id`.
    pub async fn get_downtime(
        &self,
        downtime_id: String,
        params: GetDowntimeOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::DowntimeResponse>, Error<GetDowntimeError>> {
        match self.get_downtime_with_http_info(downtime_id, params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get downtime detail by `downtime_id`.
    pub async fn get_downtime_with_http_info(
        &self,
        downtime_id: String,
        params: GetDowntimeOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::DowntimeResponse>, Error<GetDowntimeError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_downtime";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime/{downtime_id}",
            local_configuration.get_operation_host(operation_id),
            downtime_id = urlencode(downtime_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::DowntimeResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetDowntimeError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all scheduled downtimes.
    pub async fn list_downtimes(
        &self,
        params: ListDowntimesOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::ListDowntimesResponse>, Error<ListDowntimesError>>
    {
        match self.list_downtimes_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all scheduled downtimes.
    pub async fn list_downtimes_with_http_info(
        &self,
        params: ListDowntimesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListDowntimesResponse>,
        Error<ListDowntimesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_downtimes";

        // unbox and build optional parameters
        let current_only = params.current_only;
        let include = params.include;
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = current_only {
            local_req_builder =
                local_req_builder.query(&[("current_only", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ListDowntimesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListDowntimesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all active downtimes for the specified monitor.
    pub async fn list_monitor_downtimes(
        &self,
        monitor_id: i64,
        params: ListMonitorDowntimesOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::MonitorDowntimeMatchResponse>,
        Error<ListMonitorDowntimesError>,
    > {
        match self
            .list_monitor_downtimes_with_http_info(monitor_id, params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all active downtimes for the specified monitor.
    pub async fn list_monitor_downtimes_with_http_info(
        &self,
        monitor_id: i64,
        params: ListMonitorDowntimesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MonitorDowntimeMatchResponse>,
        Error<ListMonitorDowntimesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_monitor_downtimes";

        // unbox and build optional parameters
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/monitor/{monitor_id}/downtime_matches",
            local_configuration.get_operation_host(operation_id),
            monitor_id = monitor_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::MonitorDowntimeMatchResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListMonitorDowntimesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a downtime by `downtime_id`.
    pub async fn update_downtime(
        &self,
        downtime_id: String,
        body: crate::datadogV2::model::DowntimeUpdateRequest,
    ) -> Result<Option<crate::datadogV2::model::DowntimeResponse>, Error<UpdateDowntimeError>> {
        match self.update_downtime_with_http_info(downtime_id, body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a downtime by `downtime_id`.
    pub async fn update_downtime_with_http_info(
        &self,
        downtime_id: String,
        body: crate::datadogV2::model::DowntimeUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DowntimeResponse>,
        Error<UpdateDowntimeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_downtime";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime/{downtime_id}",
            local_configuration.get_operation_host(operation_id),
            downtime_id = urlencode(downtime_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::DowntimeResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateDowntimeError> =
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
