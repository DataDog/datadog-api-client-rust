// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// CreateComponentOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::create_component`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateComponentOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub include: Option<String>,
}

impl CreateComponentOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// CreateDegradationOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::create_degradation`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateDegradationOptionalParams {
    /// Whether to notify page subscribers of the degradation.
    pub notify_subscribers: Option<bool>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
}

impl CreateDegradationOptionalParams {
    /// Whether to notify page subscribers of the degradation.
    pub fn notify_subscribers(mut self, value: bool) -> Self {
        self.notify_subscribers = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// CreateMaintenanceOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::create_maintenance`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateMaintenanceOptionalParams {
    /// Whether to notify page subscribers of the maintenance.
    pub notify_subscribers: Option<bool>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
}

impl CreateMaintenanceOptionalParams {
    /// Whether to notify page subscribers of the maintenance.
    pub fn notify_subscribers(mut self, value: bool) -> Self {
        self.notify_subscribers = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// CreateStatusPageOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::create_status_page`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateStatusPageOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub include: Option<String>,
}

impl CreateStatusPageOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetComponentOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::get_component`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetComponentOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub include: Option<String>,
}

impl GetComponentOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetDegradationOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::get_degradation`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetDegradationOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
}

impl GetDegradationOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetMaintenanceOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::get_maintenance`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetMaintenanceOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
}

impl GetMaintenanceOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetStatusPageOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::get_status_page`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetStatusPageOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub include: Option<String>,
}

impl GetStatusPageOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// ListComponentsOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::list_components`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListComponentsOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub include: Option<String>,
}

impl ListComponentsOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// ListDegradationsOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::list_degradations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListDegradationsOptionalParams {
    /// Optional page id filter.
    pub filter_page_id: Option<String>,
    /// Offset to use as the start of the page.
    pub page_offset: Option<i32>,
    /// The number of degradations to return per page.
    pub page_limit: Option<i32>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
    /// Optional degradation status filter. Supported values: investigating, identified, monitoring, resolved.
    pub filter_status: Option<String>,
    /// Sort order. Prefix with '-' for descending. Supported values: created_at, -created_at, modified_at, -modified_at.
    pub sort: Option<String>,
}

impl ListDegradationsOptionalParams {
    /// Optional page id filter.
    pub fn filter_page_id(mut self, value: String) -> Self {
        self.filter_page_id = Some(value);
        self
    }
    /// Offset to use as the start of the page.
    pub fn page_offset(mut self, value: i32) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// The number of degradations to return per page.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Optional degradation status filter. Supported values: investigating, identified, monitoring, resolved.
    pub fn filter_status(mut self, value: String) -> Self {
        self.filter_status = Some(value);
        self
    }
    /// Sort order. Prefix with '-' for descending. Supported values: created_at, -created_at, modified_at, -modified_at.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListMaintenancesOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::list_maintenances`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListMaintenancesOptionalParams {
    /// Optional page id filter.
    pub filter_page_id: Option<String>,
    /// Offset to use as the start of the page.
    pub page_offset: Option<i32>,
    /// The number of maintenances to return per page.
    pub page_limit: Option<i32>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
    /// Optional maintenance status filter. Supported values: scheduled, in_progress, completed.
    pub filter_status: Option<String>,
    /// Sort order. Prefix with '-' for descending. Supported values: created_at, -created_at, start_date, -start_date.
    pub sort: Option<String>,
}

impl ListMaintenancesOptionalParams {
    /// Optional page id filter.
    pub fn filter_page_id(mut self, value: String) -> Self {
        self.filter_page_id = Some(value);
        self
    }
    /// Offset to use as the start of the page.
    pub fn page_offset(mut self, value: i32) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// The number of maintenances to return per page.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Optional maintenance status filter. Supported values: scheduled, in_progress, completed.
    pub fn filter_status(mut self, value: String) -> Self {
        self.filter_status = Some(value);
        self
    }
    /// Sort order. Prefix with '-' for descending. Supported values: created_at, -created_at, start_date, -start_date.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListStatusPagesOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::list_status_pages`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListStatusPagesOptionalParams {
    /// Offset to use as the start of the page.
    pub page_offset: Option<i32>,
    /// The number of status pages to return per page.
    pub page_limit: Option<i32>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub include: Option<String>,
}

impl ListStatusPagesOptionalParams {
    /// Offset to use as the start of the page.
    pub fn page_offset(mut self, value: i32) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// The number of status pages to return per page.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// UpdateComponentOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::update_component`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateComponentOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub include: Option<String>,
}

impl UpdateComponentOptionalParams {
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page, group.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// UpdateDegradationOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::update_degradation`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateDegradationOptionalParams {
    /// Whether to notify page subscribers of the degradation.
    pub notify_subscribers: Option<bool>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
}

impl UpdateDegradationOptionalParams {
    /// Whether to notify page subscribers of the degradation.
    pub fn notify_subscribers(mut self, value: bool) -> Self {
        self.notify_subscribers = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// UpdateMaintenanceOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::update_maintenance`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateMaintenanceOptionalParams {
    /// Whether to notify page subscribers of the maintenance.
    pub notify_subscribers: Option<bool>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub include: Option<String>,
}

impl UpdateMaintenanceOptionalParams {
    /// Whether to notify page subscribers of the maintenance.
    pub fn notify_subscribers(mut self, value: bool) -> Self {
        self.notify_subscribers = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user, status_page.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// UpdateStatusPageOptionalParams is a struct for passing parameters to the method [`StatusPagesAPI::update_status_page`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateStatusPageOptionalParams {
    /// Whether to delete existing subscribers when updating a status page's type.
    pub delete_subscribers: Option<bool>,
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub include: Option<String>,
}

impl UpdateStatusPageOptionalParams {
    /// Whether to delete existing subscribers when updating a status page's type.
    pub fn delete_subscribers(mut self, value: bool) -> Self {
        self.delete_subscribers = Some(value);
        self
    }
    /// Comma-separated list of resources to include. Supported values: created_by_user, last_modified_by_user.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// CreateComponentError is a struct for typed errors of method [`StatusPagesAPI::create_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateComponentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateDegradationError is a struct for typed errors of method [`StatusPagesAPI::create_degradation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDegradationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateMaintenanceError is a struct for typed errors of method [`StatusPagesAPI::create_maintenance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMaintenanceError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateStatusPageError is a struct for typed errors of method [`StatusPagesAPI::create_status_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateStatusPageError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteComponentError is a struct for typed errors of method [`StatusPagesAPI::delete_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteComponentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteDegradationError is a struct for typed errors of method [`StatusPagesAPI::delete_degradation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDegradationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteStatusPageError is a struct for typed errors of method [`StatusPagesAPI::delete_status_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStatusPageError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetComponentError is a struct for typed errors of method [`StatusPagesAPI::get_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetComponentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetDegradationError is a struct for typed errors of method [`StatusPagesAPI::get_degradation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDegradationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMaintenanceError is a struct for typed errors of method [`StatusPagesAPI::get_maintenance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMaintenanceError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetStatusPageError is a struct for typed errors of method [`StatusPagesAPI::get_status_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStatusPageError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListComponentsError is a struct for typed errors of method [`StatusPagesAPI::list_components`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListComponentsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListDegradationsError is a struct for typed errors of method [`StatusPagesAPI::list_degradations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDegradationsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMaintenancesError is a struct for typed errors of method [`StatusPagesAPI::list_maintenances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMaintenancesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListStatusPagesError is a struct for typed errors of method [`StatusPagesAPI::list_status_pages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListStatusPagesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateComponentError is a struct for typed errors of method [`StatusPagesAPI::update_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateComponentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateDegradationError is a struct for typed errors of method [`StatusPagesAPI::update_degradation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDegradationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMaintenanceError is a struct for typed errors of method [`StatusPagesAPI::update_maintenance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMaintenanceError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateStatusPageError is a struct for typed errors of method [`StatusPagesAPI::update_status_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateStatusPageError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage your status pages and communicate service disruptions to stakeholders via Datadog's API. See the [Status Pages documentation](<https://docs.datadoghq.com/incident_response/status_pages/>) for more information.
#[derive(Debug, Clone)]
pub struct StatusPagesAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for StatusPagesAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl StatusPagesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
            struct RetryableStatus;
            impl reqwest_retry::RetryableStrategy for RetryableStatus {
                fn handle(
                    &self,
                    res: &Result<reqwest::Response, reqwest_middleware::Error>,
                ) -> Option<reqwest_retry::Retryable> {
                    match res {
                        Ok(success) => reqwest_retry::default_on_request_success(success),
                        Err(_) => None,
                    }
                }
            }
            let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_max_retries(config.max_retries);

            let retry_middleware =
                reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                    backoff_policy,
                    RetryableStatus,
                );

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Creates a new component.
    pub async fn create_component(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::CreateComponentRequest,
        params: CreateComponentOptionalParams,
    ) -> Result<crate::datadogV2::model::StatusPagesComponent, datadog::Error<CreateComponentError>>
    {
        match self
            .create_component_with_http_info(page_id, body, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Creates a new component.
    pub async fn create_component_with_http_info(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::CreateComponentRequest,
        params: CreateComponentOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPagesComponent>,
        datadog::Error<CreateComponentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_component";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/components",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPagesComponent>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateComponentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Creates a new degradation.
    pub async fn create_degradation(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::CreateDegradationRequest,
        params: CreateDegradationOptionalParams,
    ) -> Result<crate::datadogV2::model::Degradation, datadog::Error<CreateDegradationError>> {
        match self
            .create_degradation_with_http_info(page_id, body, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Creates a new degradation.
    pub async fn create_degradation_with_http_info(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::CreateDegradationRequest,
        params: CreateDegradationOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Degradation>,
        datadog::Error<CreateDegradationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_degradation";

        // unbox and build optional parameters
        let notify_subscribers = params.notify_subscribers;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/degradations",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = notify_subscribers {
            local_req_builder =
                local_req_builder.query(&[("notify_subscribers", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::Degradation>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateDegradationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Schedules a new maintenance.
    pub async fn create_maintenance(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::CreateMaintenanceRequest,
        params: CreateMaintenanceOptionalParams,
    ) -> Result<crate::datadogV2::model::Maintenance, datadog::Error<CreateMaintenanceError>> {
        match self
            .create_maintenance_with_http_info(page_id, body, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Schedules a new maintenance.
    pub async fn create_maintenance_with_http_info(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::CreateMaintenanceRequest,
        params: CreateMaintenanceOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Maintenance>,
        datadog::Error<CreateMaintenanceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_maintenance";

        // unbox and build optional parameters
        let notify_subscribers = params.notify_subscribers;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/maintenances",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = notify_subscribers {
            local_req_builder =
                local_req_builder.query(&[("notify_subscribers", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::Maintenance>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateMaintenanceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Creates a new status page.
    pub async fn create_status_page(
        &self,
        body: crate::datadogV2::model::CreateStatusPageRequest,
        params: CreateStatusPageOptionalParams,
    ) -> Result<crate::datadogV2::model::StatusPage, datadog::Error<CreateStatusPageError>> {
        match self.create_status_page_with_http_info(body, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Creates a new status page.
    pub async fn create_status_page_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateStatusPageRequest,
        params: CreateStatusPageOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPage>,
        datadog::Error<CreateStatusPageError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_status_page";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPage>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateStatusPageError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Deletes a component by its ID.
    pub async fn delete_component(
        &self,
        page_id: uuid::Uuid,
        component_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteComponentError>> {
        match self
            .delete_component_with_http_info(page_id, component_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Deletes a component by its ID.
    pub async fn delete_component_with_http_info(
        &self,
        page_id: uuid::Uuid,
        component_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteComponentError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_component";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/components/{component_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            component_id = datadog::urlencode(component_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteComponentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Deletes a degradation by its ID.
    pub async fn delete_degradation(
        &self,
        page_id: uuid::Uuid,
        degradation_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteDegradationError>> {
        match self
            .delete_degradation_with_http_info(page_id, degradation_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Deletes a degradation by its ID.
    pub async fn delete_degradation_with_http_info(
        &self,
        page_id: uuid::Uuid,
        degradation_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteDegradationError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_degradation";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/degradations/{degradation_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            degradation_id = datadog::urlencode(degradation_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteDegradationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Deletes a status page by its ID.
    pub async fn delete_status_page(
        &self,
        page_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteStatusPageError>> {
        match self.delete_status_page_with_http_info(page_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Deletes a status page by its ID.
    pub async fn delete_status_page_with_http_info(
        &self,
        page_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteStatusPageError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_status_page";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteStatusPageError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves a specific component by its ID.
    pub async fn get_component(
        &self,
        page_id: uuid::Uuid,
        component_id: uuid::Uuid,
        params: GetComponentOptionalParams,
    ) -> Result<crate::datadogV2::model::StatusPagesComponent, datadog::Error<GetComponentError>>
    {
        match self
            .get_component_with_http_info(page_id, component_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieves a specific component by its ID.
    pub async fn get_component_with_http_info(
        &self,
        page_id: uuid::Uuid,
        component_id: uuid::Uuid,
        params: GetComponentOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPagesComponent>,
        datadog::Error<GetComponentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_component";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/components/{component_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            component_id = datadog::urlencode(component_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPagesComponent>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetComponentError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves a specific degradation by its ID.
    pub async fn get_degradation(
        &self,
        page_id: uuid::Uuid,
        degradation_id: uuid::Uuid,
        params: GetDegradationOptionalParams,
    ) -> Result<crate::datadogV2::model::Degradation, datadog::Error<GetDegradationError>> {
        match self
            .get_degradation_with_http_info(page_id, degradation_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieves a specific degradation by its ID.
    pub async fn get_degradation_with_http_info(
        &self,
        page_id: uuid::Uuid,
        degradation_id: uuid::Uuid,
        params: GetDegradationOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Degradation>,
        datadog::Error<GetDegradationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_degradation";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/degradations/{degradation_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            degradation_id = datadog::urlencode(degradation_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::Degradation>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetDegradationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves a specific maintenance by its ID.
    pub async fn get_maintenance(
        &self,
        page_id: uuid::Uuid,
        maintenance_id: uuid::Uuid,
        params: GetMaintenanceOptionalParams,
    ) -> Result<crate::datadogV2::model::Maintenance, datadog::Error<GetMaintenanceError>> {
        match self
            .get_maintenance_with_http_info(page_id, maintenance_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieves a specific maintenance by its ID.
    pub async fn get_maintenance_with_http_info(
        &self,
        page_id: uuid::Uuid,
        maintenance_id: uuid::Uuid,
        params: GetMaintenanceOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Maintenance>,
        datadog::Error<GetMaintenanceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_maintenance";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/maintenances/{maintenance_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            maintenance_id = datadog::urlencode(maintenance_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::Maintenance>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetMaintenanceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves a specific status page by its ID.
    pub async fn get_status_page(
        &self,
        page_id: uuid::Uuid,
        params: GetStatusPageOptionalParams,
    ) -> Result<crate::datadogV2::model::StatusPage, datadog::Error<GetStatusPageError>> {
        match self.get_status_page_with_http_info(page_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieves a specific status page by its ID.
    pub async fn get_status_page_with_http_info(
        &self,
        page_id: uuid::Uuid,
        params: GetStatusPageOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPage>,
        datadog::Error<GetStatusPageError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_status_page";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPage>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetStatusPageError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Lists all components for a status page.
    pub async fn list_components(
        &self,
        page_id: uuid::Uuid,
        params: ListComponentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::StatusPagesComponentArray,
        datadog::Error<ListComponentsError>,
    > {
        match self.list_components_with_http_info(page_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Lists all components for a status page.
    pub async fn list_components_with_http_info(
        &self,
        page_id: uuid::Uuid,
        params: ListComponentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPagesComponentArray>,
        datadog::Error<ListComponentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_components";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/components",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPagesComponentArray>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListComponentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Lists all degradations for the organization. Optionally filter by status and page.
    pub async fn list_degradations(
        &self,
        params: ListDegradationsOptionalParams,
    ) -> Result<crate::datadogV2::model::DegradationArray, datadog::Error<ListDegradationsError>>
    {
        match self.list_degradations_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Lists all degradations for the organization. Optionally filter by status and page.
    pub async fn list_degradations_with_http_info(
        &self,
        params: ListDegradationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::DegradationArray>,
        datadog::Error<ListDegradationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_degradations";

        // unbox and build optional parameters
        let filter_page_id = params.filter_page_id;
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;
        let include = params.include;
        let filter_status = params.filter_status;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/degradations",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_page_id {
            local_req_builder =
                local_req_builder.query(&[("filter[page_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::DegradationArray>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListDegradationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Lists all maintenances for the organization. Optionally filter by status and page.
    pub async fn list_maintenances(
        &self,
        params: ListMaintenancesOptionalParams,
    ) -> Result<crate::datadogV2::model::MaintenanceArray, datadog::Error<ListMaintenancesError>>
    {
        match self.list_maintenances_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Lists all maintenances for the organization. Optionally filter by status and page.
    pub async fn list_maintenances_with_http_info(
        &self,
        params: ListMaintenancesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::MaintenanceArray>,
        datadog::Error<ListMaintenancesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_maintenances";

        // unbox and build optional parameters
        let filter_page_id = params.filter_page_id;
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;
        let include = params.include;
        let filter_status = params.filter_status;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/maintenances",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_page_id {
            local_req_builder =
                local_req_builder.query(&[("filter[page_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::MaintenanceArray>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListMaintenancesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Lists all status pages for the organization.
    pub async fn list_status_pages(
        &self,
        params: ListStatusPagesOptionalParams,
    ) -> Result<crate::datadogV2::model::StatusPageArray, datadog::Error<ListStatusPagesError>>
    {
        match self.list_status_pages_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Lists all status pages for the organization.
    pub async fn list_status_pages_with_http_info(
        &self,
        params: ListStatusPagesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPageArray>,
        datadog::Error<ListStatusPagesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_status_pages";

        // unbox and build optional parameters
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages",
            local_configuration.get_operation_host(operation_id)
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
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPageArray>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListStatusPagesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates an existing component's attributes.
    pub async fn update_component(
        &self,
        page_id: uuid::Uuid,
        component_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchComponentRequest,
        params: UpdateComponentOptionalParams,
    ) -> Result<crate::datadogV2::model::StatusPagesComponent, datadog::Error<UpdateComponentError>>
    {
        match self
            .update_component_with_http_info(page_id, component_id, body, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Updates an existing component's attributes.
    pub async fn update_component_with_http_info(
        &self,
        page_id: uuid::Uuid,
        component_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchComponentRequest,
        params: UpdateComponentOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPagesComponent>,
        datadog::Error<UpdateComponentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_component";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/components/{component_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            component_id = datadog::urlencode(component_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPagesComponent>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateComponentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates an existing degradation's attributes.
    pub async fn update_degradation(
        &self,
        page_id: uuid::Uuid,
        degradation_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchDegradationRequest,
        params: UpdateDegradationOptionalParams,
    ) -> Result<crate::datadogV2::model::Degradation, datadog::Error<UpdateDegradationError>> {
        match self
            .update_degradation_with_http_info(page_id, degradation_id, body, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Updates an existing degradation's attributes.
    pub async fn update_degradation_with_http_info(
        &self,
        page_id: uuid::Uuid,
        degradation_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchDegradationRequest,
        params: UpdateDegradationOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Degradation>,
        datadog::Error<UpdateDegradationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_degradation";

        // unbox and build optional parameters
        let notify_subscribers = params.notify_subscribers;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/degradations/{degradation_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            degradation_id = datadog::urlencode(degradation_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_query_param) = notify_subscribers {
            local_req_builder =
                local_req_builder.query(&[("notify_subscribers", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::Degradation>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateDegradationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates an existing maintenance's attributes.
    pub async fn update_maintenance(
        &self,
        page_id: uuid::Uuid,
        maintenance_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchMaintenanceRequest,
        params: UpdateMaintenanceOptionalParams,
    ) -> Result<crate::datadogV2::model::Maintenance, datadog::Error<UpdateMaintenanceError>> {
        match self
            .update_maintenance_with_http_info(page_id, maintenance_id, body, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Updates an existing maintenance's attributes.
    pub async fn update_maintenance_with_http_info(
        &self,
        page_id: uuid::Uuid,
        maintenance_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchMaintenanceRequest,
        params: UpdateMaintenanceOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Maintenance>,
        datadog::Error<UpdateMaintenanceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_maintenance";

        // unbox and build optional parameters
        let notify_subscribers = params.notify_subscribers;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}/maintenances/{maintenance_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string()),
            maintenance_id = datadog::urlencode(maintenance_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_query_param) = notify_subscribers {
            local_req_builder =
                local_req_builder.query(&[("notify_subscribers", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::Maintenance>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateMaintenanceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates an existing status page's attributes.
    pub async fn update_status_page(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchStatusPageRequest,
        params: UpdateStatusPageOptionalParams,
    ) -> Result<crate::datadogV2::model::StatusPage, datadog::Error<UpdateStatusPageError>> {
        match self
            .update_status_page_with_http_info(page_id, body, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Updates an existing status page's attributes.
    pub async fn update_status_page_with_http_info(
        &self,
        page_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchStatusPageRequest,
        params: UpdateStatusPageOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::StatusPage>,
        datadog::Error<UpdateStatusPageError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_status_page";

        // unbox and build optional parameters
        let delete_subscribers = params.delete_subscribers;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/statuspages/{page_id}",
            local_configuration.get_operation_host(operation_id),
            page_id = datadog::urlencode(page_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_query_param) = delete_subscribers {
            local_req_builder =
                local_req_builder.query(&[("delete_subscribers", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::StatusPage>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateStatusPageError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
