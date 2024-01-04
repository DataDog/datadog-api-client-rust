// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateDashboardParams is a struct for passing parameters to the method [`CreateDashboard`]
#[derive(Clone, Debug)]
pub struct CreateDashboardParams {
    /// Create a dashboard request body.
    pub body: crate::datadogV1::model::Dashboard,
}

/// CreatePublicDashboardParams is a struct for passing parameters to the method [`CreatePublicDashboard`]
#[derive(Clone, Debug)]
pub struct CreatePublicDashboardParams {
    /// Create a shared dashboard request body.
    pub body: crate::datadogV1::model::SharedDashboard,
}

/// DeleteDashboardParams is a struct for passing parameters to the method [`DeleteDashboard`]
#[derive(Clone, Debug)]
pub struct DeleteDashboardParams {
    /// The ID of the dashboard.
    pub dashboard_id: String,
}

/// DeleteDashboardsParams is a struct for passing parameters to the method [`DeleteDashboards`]
#[derive(Clone, Debug)]
pub struct DeleteDashboardsParams {
    /// Delete dashboards request body.
    pub body: crate::datadogV1::model::DashboardBulkDeleteRequest,
}

/// DeletePublicDashboardParams is a struct for passing parameters to the method [`DeletePublicDashboard`]
#[derive(Clone, Debug)]
pub struct DeletePublicDashboardParams {
    /// The token of the shared dashboard.
    pub token: String,
}

/// DeletePublicDashboardInvitationParams is a struct for passing parameters to the method [`DeletePublicDashboardInvitation`]
#[derive(Clone, Debug)]
pub struct DeletePublicDashboardInvitationParams {
    /// The token of the shared dashboard.
    pub token: String,
    /// Shared Dashboard Invitation deletion request body.
    pub body: crate::datadogV1::model::SharedDashboardInvites,
}

/// GetDashboardParams is a struct for passing parameters to the method [`GetDashboard`]
#[derive(Clone, Debug)]
pub struct GetDashboardParams {
    /// The ID of the dashboard.
    pub dashboard_id: String,
}

/// GetPublicDashboardParams is a struct for passing parameters to the method [`GetPublicDashboard`]
#[derive(Clone, Debug)]
pub struct GetPublicDashboardParams {
    /// The token of the shared dashboard. Generated when a dashboard is shared.
    pub token: String,
}

/// GetPublicDashboardInvitationsParams is a struct for passing parameters to the method [`GetPublicDashboardInvitations`]
#[derive(Clone, Debug)]
pub struct GetPublicDashboardInvitationsParams {
    /// Token of the shared dashboard for which to fetch invitations.
    pub token: String,
    /// The number of records to return in a single request.
    pub page_size: Option<i64>,
    /// The page to access (base 0).
    pub page_number: Option<i64>,
}

/// ListDashboardsParams is a struct for passing parameters to the method [`ListDashboards`]
#[derive(Clone, Debug)]
pub struct ListDashboardsParams {
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

/// RestoreDashboardsParams is a struct for passing parameters to the method [`RestoreDashboards`]
#[derive(Clone, Debug)]
pub struct RestoreDashboardsParams {
    /// Restore dashboards request body.
    pub body: crate::datadogV1::model::DashboardRestoreRequest,
}

/// SendPublicDashboardInvitationParams is a struct for passing parameters to the method [`SendPublicDashboardInvitation`]
#[derive(Clone, Debug)]
pub struct SendPublicDashboardInvitationParams {
    /// The token of the shared dashboard.
    pub token: String,
    /// Shared Dashboard Invitation request body.
    pub body: crate::datadogV1::model::SharedDashboardInvites,
}

/// UpdateDashboardParams is a struct for passing parameters to the method [`UpdateDashboard`]
#[derive(Clone, Debug)]
pub struct UpdateDashboardParams {
    /// The ID of the dashboard.
    pub dashboard_id: String,
    /// Update Dashboard request body.
    pub body: crate::datadogV1::model::Dashboard,
}

/// UpdatePublicDashboardParams is a struct for passing parameters to the method [`UpdatePublicDashboard`]
#[derive(Clone, Debug)]
pub struct UpdatePublicDashboardParams {
    /// The token of the shared dashboard.
    pub token: String,
    /// Update Dashboard request body.
    pub body: crate::datadogV1::model::SharedDashboardUpdateRequest,
}

/// CreateDashboardError is a struct for typed errors of method [`CreateDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreatePublicDashboardError is a struct for typed errors of method [`CreatePublicDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePublicDashboardError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardError is a struct for typed errors of method [`DeleteDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardsError is a struct for typed errors of method [`DeleteDashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePublicDashboardError is a struct for typed errors of method [`DeletePublicDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePublicDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePublicDashboardInvitationError is a struct for typed errors of method [`DeletePublicDashboardInvitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePublicDashboardInvitationError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDashboardError is a struct for typed errors of method [`GetDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPublicDashboardError is a struct for typed errors of method [`GetPublicDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicDashboardError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPublicDashboardInvitationsError is a struct for typed errors of method [`GetPublicDashboardInvitations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicDashboardInvitationsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListDashboardsError is a struct for typed errors of method [`ListDashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDashboardsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RestoreDashboardsError is a struct for typed errors of method [`RestoreDashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestoreDashboardsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SendPublicDashboardInvitationError is a struct for typed errors of method [`SendPublicDashboardInvitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPublicDashboardInvitationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardError is a struct for typed errors of method [`UpdateDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdatePublicDashboardError is a struct for typed errors of method [`UpdatePublicDashboard`]
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
    /// Refer to the following [documentation](https://docs.datadoghq.com/developers/metrics/type_modifiers/?tab=count#in-application-modifiers) for more information on these modifiers.
    pub async fn create_dashboard(
        &self,
        params: CreateDashboardParams,
    ) -> Result<Option<crate::datadogV1::model::Dashboard>, Error<CreateDashboardError>> {
        match self.create_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a dashboard using the specified options. When defining queries in your widgets, take note of which queries should have the `as_count()` or `as_rate()` modifiers appended.
    /// Refer to the following [documentation](https://docs.datadoghq.com/developers/metrics/type_modifiers/?tab=count#in-application-modifiers) for more information on these modifiers.
    pub async fn create_dashboard_with_http_info(
        &self,
        params: CreateDashboardParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::Dashboard>, Error<CreateDashboardError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/dashboard", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::Dashboard> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreatePublicDashboardParams,
    ) -> Result<Option<crate::datadogV1::model::SharedDashboard>, Error<CreatePublicDashboardError>>
    {
        match self.create_public_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Share a specified private dashboard, generating a URL at which it can be publicly viewed.
    pub async fn create_public_dashboard_with_http_info(
        &self,
        params: CreatePublicDashboardParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboard>,
        Error<CreatePublicDashboardError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/dashboard/public", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::SharedDashboard> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteDashboardParams,
    ) -> Result<Option<crate::datadogV1::model::DashboardDeleteResponse>, Error<DeleteDashboardError>>
    {
        match self.delete_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a dashboard using the specified ID.
    pub async fn delete_dashboard_with_http_info(
        &self,
        params: DeleteDashboardParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::DashboardDeleteResponse>,
        Error<DeleteDashboardError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let dashboard_id = params.dashboard_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.base_path,
            dashboard_id = urlencode(dashboard_id)
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
            let local_entity: Option<crate::datadogV1::model::DashboardDeleteResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteDashboardsParams,
    ) -> Result<Option<()>, Error<DeleteDashboardsError>> {
        match self.delete_dashboards_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete dashboards using the specified IDs. If there are any failures, no dashboards will be deleted (partial success is not allowed).
    pub async fn delete_dashboards_with_http_info(
        &self,
        params: DeleteDashboardsParams,
    ) -> Result<ResponseContent<()>, Error<DeleteDashboardsError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/dashboard", local_configuration.base_path);
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
        params: DeletePublicDashboardParams,
    ) -> Result<
        Option<crate::datadogV1::model::DeleteSharedDashboardResponse>,
        Error<DeletePublicDashboardError>,
    > {
        match self.delete_public_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Revoke the public URL for a dashboard (rendering it private) associated with the specified token.
    pub async fn delete_public_dashboard_with_http_info(
        &self,
        params: DeletePublicDashboardParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::DeleteSharedDashboardResponse>,
        Error<DeletePublicDashboardError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let token = params.token;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}",
            local_configuration.base_path,
            token = urlencode(token)
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
            let local_entity: Option<crate::datadogV1::model::DeleteSharedDashboardResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeletePublicDashboardInvitationParams,
    ) -> Result<Option<()>, Error<DeletePublicDashboardInvitationError>> {
        match self
            .delete_public_dashboard_invitation_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Revoke previously sent invitation emails and active sessions used to access a given shared dashboard for specific email addresses.
    pub async fn delete_public_dashboard_invitation_with_http_info(
        &self,
        params: DeletePublicDashboardInvitationParams,
    ) -> Result<ResponseContent<()>, Error<DeletePublicDashboardInvitationError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let token = params.token;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}/invitation",
            local_configuration.base_path,
            token = urlencode(token)
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
        params: GetDashboardParams,
    ) -> Result<Option<crate::datadogV1::model::Dashboard>, Error<GetDashboardError>> {
        match self.get_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a dashboard using the specified ID.
    pub async fn get_dashboard_with_http_info(
        &self,
        params: GetDashboardParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::Dashboard>, Error<GetDashboardError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let dashboard_id = params.dashboard_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.base_path,
            dashboard_id = urlencode(dashboard_id)
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
            let local_entity: Option<crate::datadogV1::model::Dashboard> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetPublicDashboardParams,
    ) -> Result<Option<crate::datadogV1::model::SharedDashboard>, Error<GetPublicDashboardError>>
    {
        match self.get_public_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Fetch an existing shared dashboard's sharing metadata associated with the specified token.
    pub async fn get_public_dashboard_with_http_info(
        &self,
        params: GetPublicDashboardParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboard>,
        Error<GetPublicDashboardError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let token = params.token;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}",
            local_configuration.base_path,
            token = urlencode(token)
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
            let local_entity: Option<crate::datadogV1::model::SharedDashboard> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetPublicDashboardInvitationsParams,
    ) -> Result<
        Option<crate::datadogV1::model::SharedDashboardInvites>,
        Error<GetPublicDashboardInvitationsError>,
    > {
        match self
            .get_public_dashboard_invitations_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Describe the invitations that exist for the given shared dashboard (paginated).
    pub async fn get_public_dashboard_invitations_with_http_info(
        &self,
        params: GetPublicDashboardInvitationsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboardInvites>,
        Error<GetPublicDashboardInvitationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let token = params.token;
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}/invitation",
            local_configuration.base_path,
            token = urlencode(token)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page_size", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder = local_req_builder.query(&[("page_number", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::SharedDashboardInvites> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: ListDashboardsParams,
    ) -> Result<Option<crate::datadogV1::model::DashboardSummary>, Error<ListDashboardsError>> {
        match self.list_dashboards_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all dashboards.
    ///
    /// **Note**: This query will only return custom created or cloned dashboards.
    /// This query will not return preset dashboards.
    pub async fn list_dashboards_with_http_info(
        &self,
        params: ListDashboardsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::DashboardSummary>,
        Error<ListDashboardsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_shared = params.filter_shared;
        let filter_deleted = params.filter_deleted;
        let count = params.count;
        let start = params.start;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/dashboard", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = filter_shared {
            local_req_builder =
                local_req_builder.query(&[("filter[shared]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_deleted {
            local_req_builder =
                local_req_builder.query(&[("filter[deleted]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = count {
            local_req_builder = local_req_builder.query(&[("count", &local_str.to_string())]);
        };
        if let Some(ref local_str) = start {
            local_req_builder = local_req_builder.query(&[("start", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::DashboardSummary> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: RestoreDashboardsParams,
    ) -> Result<Option<()>, Error<RestoreDashboardsError>> {
        match self.restore_dashboards_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Restore dashboards using the specified IDs. If there are any failures, no dashboards will be restored (partial success is not allowed).
    pub async fn restore_dashboards_with_http_info(
        &self,
        params: RestoreDashboardsParams,
    ) -> Result<ResponseContent<()>, Error<RestoreDashboardsError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/dashboard", local_configuration.base_path);
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
        params: SendPublicDashboardInvitationParams,
    ) -> Result<
        Option<crate::datadogV1::model::SharedDashboardInvites>,
        Error<SendPublicDashboardInvitationError>,
    > {
        match self
            .send_public_dashboard_invitation_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Send emails to specified email addresses containing links to access a given authenticated shared dashboard. Email addresses must already belong to the authenticated shared dashboard's share_list.
    pub async fn send_public_dashboard_invitation_with_http_info(
        &self,
        params: SendPublicDashboardInvitationParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboardInvites>,
        Error<SendPublicDashboardInvitationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let token = params.token;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}/invitation",
            local_configuration.base_path,
            token = urlencode(token)
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
            let local_entity: Option<crate::datadogV1::model::SharedDashboardInvites> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateDashboardParams,
    ) -> Result<Option<crate::datadogV1::model::Dashboard>, Error<UpdateDashboardError>> {
        match self.update_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a dashboard using the specified ID.
    pub async fn update_dashboard_with_http_info(
        &self,
        params: UpdateDashboardParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::Dashboard>, Error<UpdateDashboardError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let dashboard_id = params.dashboard_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.base_path,
            dashboard_id = urlencode(dashboard_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV1::model::Dashboard> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdatePublicDashboardParams,
    ) -> Result<Option<crate::datadogV1::model::SharedDashboard>, Error<UpdatePublicDashboardError>>
    {
        match self.update_public_dashboard_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a shared dashboard associated with the specified token.
    pub async fn update_public_dashboard_with_http_info(
        &self,
        params: UpdatePublicDashboardParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SharedDashboard>,
        Error<UpdatePublicDashboardError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let token = params.token;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/public/{token}",
            local_configuration.base_path,
            token = urlencode(token)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV1::model::SharedDashboard> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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