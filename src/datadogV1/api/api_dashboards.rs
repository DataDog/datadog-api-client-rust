// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetPublicDashboardInvitationsOptionalParams is a struct for passing parameters to the method [`DashboardsAPI::get_public_dashboard_invitations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetPublicDashboardInvitationsOptionalParams {
    /// The number of records to return in a single request.
    pub page_size: Option<i64>,
    /// The page to access (base 0).
    pub page_number: Option<i64>,
}

impl GetPublicDashboardInvitationsOptionalParams {
    /// The number of records to return in a single request.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The page to access (base 0).
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
}

/// ListDashboardsOptionalParams is a struct for passing parameters to the method [`DashboardsAPI::list_dashboards`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListDashboardsOptionalParams {
    /// When `true`, this query only returns shared custom created
    /// or cloned dashboards.
    pub filter_shared: Option<bool>,
    /// When `true`, this query returns only deleted custom-created
    /// or cloned dashboards. This parameter is incompatible with `filter[shared]`.
    pub filter_deleted: Option<bool>,
    /// The maximum number of dashboards returned in the list.
    pub count: Option<i64>,
    /// The specific offset to use as the beginning of the returned response.
    pub start: Option<i64>,
}

impl ListDashboardsOptionalParams {
    /// When `true`, this query only returns shared custom created
    /// or cloned dashboards.
    pub fn filter_shared(mut self, value: bool) -> Self {
        self.filter_shared = Some(value);
        self
    }
    /// When `true`, this query returns only deleted custom-created
    /// or cloned dashboards. This parameter is incompatible with `filter[shared]`.
    pub fn filter_deleted(mut self, value: bool) -> Self {
        self.filter_deleted = Some(value);
        self
    }
    /// The maximum number of dashboards returned in the list.
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }
    /// The specific offset to use as the beginning of the returned response.
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }
}

/// CreateDashboardError is a struct for typed errors of method [`DashboardsAPI::create_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreatePublicDashboardError is a struct for typed errors of method [`DashboardsAPI::create_public_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePublicDashboardError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardError is a struct for typed errors of method [`DashboardsAPI::delete_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardsError is a struct for typed errors of method [`DashboardsAPI::delete_dashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePublicDashboardError is a struct for typed errors of method [`DashboardsAPI::delete_public_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePublicDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePublicDashboardInvitationError is a struct for typed errors of method [`DashboardsAPI::delete_public_dashboard_invitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePublicDashboardInvitationError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDashboardError is a struct for typed errors of method [`DashboardsAPI::get_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPublicDashboardError is a struct for typed errors of method [`DashboardsAPI::get_public_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPublicDashboardInvitationsError is a struct for typed errors of method [`DashboardsAPI::get_public_dashboard_invitations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicDashboardInvitationsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListDashboardsError is a struct for typed errors of method [`DashboardsAPI::list_dashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDashboardsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RestoreDashboardsError is a struct for typed errors of method [`DashboardsAPI::restore_dashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestoreDashboardsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SendPublicDashboardInvitationError is a struct for typed errors of method [`DashboardsAPI::send_public_dashboard_invitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPublicDashboardInvitationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardError is a struct for typed errors of method [`DashboardsAPI::update_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdatePublicDashboardError is a struct for typed errors of method [`DashboardsAPI::update_public_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePublicDashboardError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct DashboardsAPI {
    config: configuration::Configuration,
}

impl Default for DashboardsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl DashboardsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a dashboard using the specified options. When defining queries in your widgets, take note of which queries should have the `as_count()` or `as_rate()` modifiers appended.
    /// Refer to the following [documentation](<https://docs.datadoghq.com/developers/metrics/type_modifiers/?tab=count#in-application-modifiers>) for more information on these modifiers.
    pub async fn create_dashboard(
        &self,
        body: crate::datadogV1::model::Dashboard,
    ) -> Result<crate::datadogV1::model::Dashboard, Error<CreateDashboardError>> {
        match self.create_dashboard_with_http_info(body).await {
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

    /// Create a dashboard using the specified options. When defining queries in your widgets, take note of which queries should have the `as_count()` or `as_rate()` modifiers appended.
    /// Refer to the following [documentation](<https://docs.datadoghq.com/developers/metrics/type_modifiers/?tab=count#in-application-modifiers>) for more information on these modifiers.
    pub async fn create_dashboard_with_http_info(
        &self,
        body: crate::datadogV1::model::Dashboard,
    ) -> Result<ResponseContent<crate::datadogV1::model::Dashboard>, Error<CreateDashboardError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.create_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard",
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
            match serde_json::from_str::<crate::datadogV1::model::Dashboard>(&local_content) {
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
            let local_entity: Option<CreateDashboardError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Share a specified private dashboard, generating a URL at which it can be publicly viewed.
    pub async fn create_public_dashboard(
        &self,
        body: crate::datadogV1::model::SharedDashboard,
    ) -> Result<crate::datadogV1::model::SharedDashboard, Error<CreatePublicDashboardError>> {
        match self.create_public_dashboard_with_http_info(body).await {
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

    /// Share a specified private dashboard, generating a URL at which it can be publicly viewed.
    pub async fn create_public_dashboard_with_http_info(
        &self,
        body: crate::datadogV1::model::SharedDashboard,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboard>,
        Error<CreatePublicDashboardError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_public_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public",
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
            match serde_json::from_str::<crate::datadogV1::model::SharedDashboard>(&local_content) {
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
            let local_entity: Option<CreatePublicDashboardError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a dashboard using the specified ID.
    pub async fn delete_dashboard(
        &self,
        dashboard_id: String,
    ) -> Result<crate::datadogV1::model::DashboardDeleteResponse, Error<DeleteDashboardError>> {
        match self.delete_dashboard_with_http_info(dashboard_id).await {
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

    /// Delete a dashboard using the specified ID.
    pub async fn delete_dashboard_with_http_info(
        &self,
        dashboard_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::DashboardDeleteResponse>,
        Error<DeleteDashboardError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.get_operation_host(operation_id),
            dashboard_id = urlencode(dashboard_id)
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
            match serde_json::from_str::<crate::datadogV1::model::DashboardDeleteResponse>(
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
            let local_entity: Option<DeleteDashboardError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete dashboards using the specified IDs. If there are any failures, no dashboards will be deleted (partial success is not allowed).
    pub async fn delete_dashboards(
        &self,
        body: crate::datadogV1::model::DashboardBulkDeleteRequest,
    ) -> Result<(), Error<DeleteDashboardsError>> {
        match self.delete_dashboards_with_http_info(body).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete dashboards using the specified IDs. If there are any failures, no dashboards will be deleted (partial success is not allowed).
    pub async fn delete_dashboards_with_http_info(
        &self,
        body: crate::datadogV1::model::DashboardBulkDeleteRequest,
    ) -> Result<ResponseContent<()>, Error<DeleteDashboardsError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_dashboards";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard",
            local_configuration.get_operation_host(operation_id)
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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteDashboardsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Revoke the public URL for a dashboard (rendering it private) associated with the specified token.
    pub async fn delete_public_dashboard(
        &self,
        token: String,
    ) -> Result<
        crate::datadogV1::model::DeleteSharedDashboardResponse,
        Error<DeletePublicDashboardError>,
    > {
        match self.delete_public_dashboard_with_http_info(token).await {
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

    /// Revoke the public URL for a dashboard (rendering it private) associated with the specified token.
    pub async fn delete_public_dashboard_with_http_info(
        &self,
        token: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::DeleteSharedDashboardResponse>,
        Error<DeletePublicDashboardError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_public_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}",
            local_configuration.get_operation_host(operation_id),
            token = urlencode(token)
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
            match serde_json::from_str::<crate::datadogV1::model::DeleteSharedDashboardResponse>(
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
            let local_entity: Option<DeletePublicDashboardError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Revoke previously sent invitation emails and active sessions used to access a given shared dashboard for specific email addresses.
    pub async fn delete_public_dashboard_invitation(
        &self,
        token: String,
        body: crate::datadogV1::model::SharedDashboardInvites,
    ) -> Result<(), Error<DeletePublicDashboardInvitationError>> {
        match self
            .delete_public_dashboard_invitation_with_http_info(token, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Revoke previously sent invitation emails and active sessions used to access a given shared dashboard for specific email addresses.
    pub async fn delete_public_dashboard_invitation_with_http_info(
        &self,
        token: String,
        body: crate::datadogV1::model::SharedDashboardInvites,
    ) -> Result<ResponseContent<()>, Error<DeletePublicDashboardInvitationError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_public_dashboard_invitation";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}/invitation",
            local_configuration.get_operation_host(operation_id),
            token = urlencode(token)
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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeletePublicDashboardInvitationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a dashboard using the specified ID.
    pub async fn get_dashboard(
        &self,
        dashboard_id: String,
    ) -> Result<crate::datadogV1::model::Dashboard, Error<GetDashboardError>> {
        match self.get_dashboard_with_http_info(dashboard_id).await {
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

    /// Get a dashboard using the specified ID.
    pub async fn get_dashboard_with_http_info(
        &self,
        dashboard_id: String,
    ) -> Result<ResponseContent<crate::datadogV1::model::Dashboard>, Error<GetDashboardError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.get_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.get_operation_host(operation_id),
            dashboard_id = urlencode(dashboard_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV1::model::Dashboard>(&local_content) {
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
            let local_entity: Option<GetDashboardError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Fetch an existing shared dashboard's sharing metadata associated with the specified token.
    pub async fn get_public_dashboard(
        &self,
        token: String,
    ) -> Result<crate::datadogV1::model::SharedDashboard, Error<GetPublicDashboardError>> {
        match self.get_public_dashboard_with_http_info(token).await {
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

    /// Fetch an existing shared dashboard's sharing metadata associated with the specified token.
    pub async fn get_public_dashboard_with_http_info(
        &self,
        token: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboard>,
        Error<GetPublicDashboardError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_public_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}",
            local_configuration.get_operation_host(operation_id),
            token = urlencode(token)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV1::model::SharedDashboard>(&local_content) {
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
            let local_entity: Option<GetPublicDashboardError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Describe the invitations that exist for the given shared dashboard (paginated).
    pub async fn get_public_dashboard_invitations(
        &self,
        token: String,
        params: GetPublicDashboardInvitationsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::SharedDashboardInvites,
        Error<GetPublicDashboardInvitationsError>,
    > {
        match self
            .get_public_dashboard_invitations_with_http_info(token, params)
            .await
        {
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

    /// Describe the invitations that exist for the given shared dashboard (paginated).
    pub async fn get_public_dashboard_invitations_with_http_info(
        &self,
        token: String,
        params: GetPublicDashboardInvitationsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboardInvites>,
        Error<GetPublicDashboardInvitationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_public_dashboard_invitations";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}/invitation",
            local_configuration.get_operation_host(operation_id),
            token = urlencode(token)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page_number", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::SharedDashboardInvites>(
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
            let local_entity: Option<GetPublicDashboardInvitationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all dashboards.
    ///
    /// **Note**: This query will only return custom created or cloned dashboards.
    /// This query will not return preset dashboards.
    pub async fn list_dashboards(
        &self,
        params: ListDashboardsOptionalParams,
    ) -> Result<crate::datadogV1::model::DashboardSummary, Error<ListDashboardsError>> {
        match self.list_dashboards_with_http_info(params).await {
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

    pub fn list_dashboards_with_pagination(
        &self,
        mut params: ListDashboardsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV1::model::DashboardSummaryDefinition,
            Error<ListDashboardsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.count.is_none() {
                params.count = Some(page_size);
            } else {
                page_size = params.count.unwrap().clone();
            }
            loop {
                let resp = self.list_dashboards(params.clone()).await?;
                let Some(dashboards) = resp.dashboards else { break };

                let r = dashboards;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.start.is_none() {
                    params.start = Some(page_size.clone());
                } else {
                    params.start = Some(params.start.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get all dashboards.
    ///
    /// **Note**: This query will only return custom created or cloned dashboards.
    /// This query will not return preset dashboards.
    pub async fn list_dashboards_with_http_info(
        &self,
        params: ListDashboardsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::DashboardSummary>,
        Error<ListDashboardsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_dashboards";

        // unbox and build optional parameters
        let filter_shared = params.filter_shared;
        let filter_deleted = params.filter_deleted;
        let count = params.count;
        let start = params.start;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_shared {
            local_req_builder =
                local_req_builder.query(&[("filter[shared]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_deleted {
            local_req_builder =
                local_req_builder.query(&[("filter[deleted]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = count {
            local_req_builder =
                local_req_builder.query(&[("count", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = start {
            local_req_builder =
                local_req_builder.query(&[("start", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::DashboardSummary>(&local_content)
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
            let local_entity: Option<ListDashboardsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Restore dashboards using the specified IDs. If there are any failures, no dashboards will be restored (partial success is not allowed).
    pub async fn restore_dashboards(
        &self,
        body: crate::datadogV1::model::DashboardRestoreRequest,
    ) -> Result<(), Error<RestoreDashboardsError>> {
        match self.restore_dashboards_with_http_info(body).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Restore dashboards using the specified IDs. If there are any failures, no dashboards will be restored (partial success is not allowed).
    pub async fn restore_dashboards_with_http_info(
        &self,
        body: crate::datadogV1::model::DashboardRestoreRequest,
    ) -> Result<ResponseContent<()>, Error<RestoreDashboardsError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.restore_dashboards";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard",
            local_configuration.get_operation_host(operation_id)
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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<RestoreDashboardsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Send emails to specified email addresses containing links to access a given authenticated shared dashboard. Email addresses must already belong to the authenticated shared dashboard's share_list.
    pub async fn send_public_dashboard_invitation(
        &self,
        token: String,
        body: crate::datadogV1::model::SharedDashboardInvites,
    ) -> Result<
        crate::datadogV1::model::SharedDashboardInvites,
        Error<SendPublicDashboardInvitationError>,
    > {
        match self
            .send_public_dashboard_invitation_with_http_info(token, body)
            .await
        {
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

    /// Send emails to specified email addresses containing links to access a given authenticated shared dashboard. Email addresses must already belong to the authenticated shared dashboard's share_list.
    pub async fn send_public_dashboard_invitation_with_http_info(
        &self,
        token: String,
        body: crate::datadogV1::model::SharedDashboardInvites,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboardInvites>,
        Error<SendPublicDashboardInvitationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.send_public_dashboard_invitation";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}/invitation",
            local_configuration.get_operation_host(operation_id),
            token = urlencode(token)
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
            match serde_json::from_str::<crate::datadogV1::model::SharedDashboardInvites>(
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
            let local_entity: Option<SendPublicDashboardInvitationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a dashboard using the specified ID.
    pub async fn update_dashboard(
        &self,
        dashboard_id: String,
        body: crate::datadogV1::model::Dashboard,
    ) -> Result<crate::datadogV1::model::Dashboard, Error<UpdateDashboardError>> {
        match self
            .update_dashboard_with_http_info(dashboard_id, body)
            .await
        {
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

    /// Update a dashboard using the specified ID.
    pub async fn update_dashboard_with_http_info(
        &self,
        dashboard_id: String,
        body: crate::datadogV1::model::Dashboard,
    ) -> Result<ResponseContent<crate::datadogV1::model::Dashboard>, Error<UpdateDashboardError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.update_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.get_operation_host(operation_id),
            dashboard_id = urlencode(dashboard_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV1::model::Dashboard>(&local_content) {
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
            let local_entity: Option<UpdateDashboardError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a shared dashboard associated with the specified token.
    pub async fn update_public_dashboard(
        &self,
        token: String,
        body: crate::datadogV1::model::SharedDashboardUpdateRequest,
    ) -> Result<crate::datadogV1::model::SharedDashboard, Error<UpdatePublicDashboardError>> {
        match self
            .update_public_dashboard_with_http_info(token, body)
            .await
        {
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

    /// Update a shared dashboard associated with the specified token.
    pub async fn update_public_dashboard_with_http_info(
        &self,
        token: String,
        body: crate::datadogV1::model::SharedDashboardUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboard>,
        Error<UpdatePublicDashboardError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_public_dashboard";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}",
            local_configuration.get_operation_host(operation_id),
            token = urlencode(token)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV1::model::SharedDashboard>(&local_content) {
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
            let local_entity: Option<UpdatePublicDashboardError> =
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
