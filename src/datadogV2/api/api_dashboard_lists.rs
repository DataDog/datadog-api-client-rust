// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateDashboardListItemsParams is a struct for passing parameters to the method [`CreateDashboardListItems`]
#[derive(Clone, Debug, Default)]
pub struct CreateDashboardListItemsParams {
    /// ID of the dashboard list to add items to.
    pub dashboard_list_id: i64,
    /// Dashboards to add to the dashboard list.
    pub body: crate::datadogV2::model::DashboardListAddItemsRequest,
}

/// DeleteDashboardListItemsParams is a struct for passing parameters to the method [`DeleteDashboardListItems`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDashboardListItemsParams {
    /// ID of the dashboard list to delete items from.
    pub dashboard_list_id: i64,
    /// Dashboards to delete from the dashboard list.
    pub body: crate::datadogV2::model::DashboardListDeleteItemsRequest,
}

/// GetDashboardListItemsParams is a struct for passing parameters to the method [`GetDashboardListItems`]
#[derive(Clone, Debug, Default)]
pub struct GetDashboardListItemsParams {
    /// ID of the dashboard list to get items from.
    pub dashboard_list_id: i64,
}

/// UpdateDashboardListItemsParams is a struct for passing parameters to the method [`UpdateDashboardListItems`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDashboardListItemsParams {
    /// ID of the dashboard list to update items from.
    pub dashboard_list_id: i64,
    /// New dashboards of the dashboard list.
    pub body: crate::datadogV2::model::DashboardListUpdateItemsRequest,
}

/// CreateDashboardListItemsError is a struct for typed errors of method [`CreateDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardListItemsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardListItemsError is a struct for typed errors of method [`DeleteDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardListItemsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDashboardListItemsError is a struct for typed errors of method [`GetDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardListItemsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardListItemsError is a struct for typed errors of method [`UpdateDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardListItemsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct DashboardListsAPI {
    config: configuration::Configuration,
}

impl Default for DashboardListsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl DashboardListsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Add dashboards to an existing dashboard list.
    pub async fn create_dashboard_list_items(
        &self,
        params: CreateDashboardListItemsParams,
    ) -> Result<
        Option<crate::datadogV2::model::DashboardListAddItemsResponse>,
        Error<CreateDashboardListItemsError>,
    > {
        match self
            .create_dashboard_list_items_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Add dashboards to an existing dashboard list.
    pub async fn create_dashboard_list_items_with_http_info(
        &self,
        params: CreateDashboardListItemsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListAddItemsResponse>,
        Error<CreateDashboardListItemsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let dashboard_list_id = params.dashboard_list_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.base_path,
            dashboard_list_id = dashboard_list_id
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
            let local_entity: Option<crate::datadogV2::model::DashboardListAddItemsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateDashboardListItemsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete dashboards from an existing dashboard list.
    pub async fn delete_dashboard_list_items(
        &self,
        params: DeleteDashboardListItemsParams,
    ) -> Result<
        Option<crate::datadogV2::model::DashboardListDeleteItemsResponse>,
        Error<DeleteDashboardListItemsError>,
    > {
        match self
            .delete_dashboard_list_items_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete dashboards from an existing dashboard list.
    pub async fn delete_dashboard_list_items_with_http_info(
        &self,
        params: DeleteDashboardListItemsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListDeleteItemsResponse>,
        Error<DeleteDashboardListItemsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let dashboard_list_id = params.dashboard_list_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.base_path,
            dashboard_list_id = dashboard_list_id
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
            let local_entity: Option<crate::datadogV2::model::DashboardListDeleteItemsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteDashboardListItemsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Fetch the dashboard list’s dashboard definitions.
    pub async fn get_dashboard_list_items(
        &self,
        params: GetDashboardListItemsParams,
    ) -> Result<
        Option<crate::datadogV2::model::DashboardListItems>,
        Error<GetDashboardListItemsError>,
    > {
        match self.get_dashboard_list_items_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Fetch the dashboard list’s dashboard definitions.
    pub async fn get_dashboard_list_items_with_http_info(
        &self,
        params: GetDashboardListItemsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListItems>,
        Error<GetDashboardListItemsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let dashboard_list_id = params.dashboard_list_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.base_path,
            dashboard_list_id = dashboard_list_id
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
            let local_entity: Option<crate::datadogV2::model::DashboardListItems> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetDashboardListItemsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update dashboards of an existing dashboard list.
    pub async fn update_dashboard_list_items(
        &self,
        params: UpdateDashboardListItemsParams,
    ) -> Result<
        Option<crate::datadogV2::model::DashboardListUpdateItemsResponse>,
        Error<UpdateDashboardListItemsError>,
    > {
        match self
            .update_dashboard_list_items_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update dashboards of an existing dashboard list.
    pub async fn update_dashboard_list_items_with_http_info(
        &self,
        params: UpdateDashboardListItemsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListUpdateItemsResponse>,
        Error<UpdateDashboardListItemsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let dashboard_list_id = params.dashboard_list_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.base_path,
            dashboard_list_id = dashboard_list_id
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
            let local_entity: Option<crate::datadogV2::model::DashboardListUpdateItemsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateDashboardListItemsError> =
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