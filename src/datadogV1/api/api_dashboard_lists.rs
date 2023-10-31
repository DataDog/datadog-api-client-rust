// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateDashboardListParams is a struct for passing parameters to the method [`CreateDashboardList`]
#[derive(Clone, Debug, Default)]
pub struct CreateDashboardListParams {
    /// Create a dashboard list request body.
    pub body: crate::datadogV1::model::DashboardList,
}

/// DeleteDashboardListParams is a struct for passing parameters to the method [`DeleteDashboardList`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDashboardListParams {
    /// ID of the dashboard list to delete.
    pub list_id: i64,
}

/// GetDashboardListParams is a struct for passing parameters to the method [`GetDashboardList`]
#[derive(Clone, Debug, Default)]
pub struct GetDashboardListParams {
    /// ID of the dashboard list to fetch.
    pub list_id: i64,
}

/// UpdateDashboardListParams is a struct for passing parameters to the method [`UpdateDashboardList`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDashboardListParams {
    /// ID of the dashboard list to update.
    pub list_id: i64,
    /// Update a dashboard list request body.
    pub body: crate::datadogV1::model::DashboardList,
}

/// CreateDashboardListError is a struct for typed errors of method [`CreateDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardListError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardListError is a struct for typed errors of method [`DeleteDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardListError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDashboardListError is a struct for typed errors of method [`GetDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardListError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListDashboardListsError is a struct for typed errors of method [`ListDashboardLists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDashboardListsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardListError is a struct for typed errors of method [`UpdateDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardListError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
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

    /// Create an empty dashboard list.
    pub async fn create_dashboard_list(
        &self,
        params: CreateDashboardListParams,
    ) -> Result<Option<crate::datadogV1::model::DashboardList>, Error<CreateDashboardListError>> {
        match self.create_dashboard_list_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an empty dashboard list.
    pub async fn create_dashboard_list_with_http_info(
        &self,
        params: CreateDashboardListParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::DashboardList>, Error<CreateDashboardListError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/dashboard/lists/manual", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::DashboardList> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateDashboardListError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a dashboard list.
    pub async fn delete_dashboard_list(
        &self,
        params: DeleteDashboardListParams,
    ) -> Result<Option<crate::datadogV1::model::DashboardListDeleteResponse>, Error<DeleteDashboardListError>> {
        match self.delete_dashboard_list_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a dashboard list.
    pub async fn delete_dashboard_list_with_http_info(
        &self,
        params: DeleteDashboardListParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::DashboardListDeleteResponse>, Error<DeleteDashboardListError>>
    {
        let local_configuration = &self.config;

        // unbox the parameters
        let list_id = params.list_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/lists/manual/{list_id}",
            local_configuration.base_path,
            list_id = list_id
        );
        let mut local_req_builder = local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::DashboardListDeleteResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteDashboardListError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Fetch an existing dashboard list's definition.
    pub async fn get_dashboard_list(
        &self,
        params: GetDashboardListParams,
    ) -> Result<Option<crate::datadogV1::model::DashboardList>, Error<GetDashboardListError>> {
        match self.get_dashboard_list_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Fetch an existing dashboard list's definition.
    pub async fn get_dashboard_list_with_http_info(
        &self,
        params: GetDashboardListParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::DashboardList>, Error<GetDashboardListError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let list_id = params.list_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/lists/manual/{list_id}",
            local_configuration.base_path,
            list_id = list_id
        );
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::DashboardList> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetDashboardListError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Fetch all of your existing dashboard list definitions.
    pub async fn list_dashboard_lists(
        &self,
    ) -> Result<Option<crate::datadogV1::model::DashboardListListResponse>, Error<ListDashboardListsError>> {
        match self.list_dashboard_lists_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Fetch all of your existing dashboard list definitions.
    pub async fn list_dashboard_lists_with_http_info(
        &self,
    ) -> Result<ResponseContent<crate::datadogV1::model::DashboardListListResponse>, Error<ListDashboardListsError>>
    {
        let local_configuration = &self.config;

        // unbox the parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/dashboard/lists/manual", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::DashboardListListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListDashboardListsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the name of a dashboard list.
    pub async fn update_dashboard_list(
        &self,
        params: UpdateDashboardListParams,
    ) -> Result<Option<crate::datadogV1::model::DashboardList>, Error<UpdateDashboardListError>> {
        match self.update_dashboard_list_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the name of a dashboard list.
    pub async fn update_dashboard_list_with_http_info(
        &self,
        params: UpdateDashboardListParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::DashboardList>, Error<UpdateDashboardListError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let list_id = params.list_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/lists/manual/{list_id}",
            local_configuration.base_path,
            list_id = list_id
        );
        let mut local_req_builder = local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::DashboardList> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateDashboardListError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
