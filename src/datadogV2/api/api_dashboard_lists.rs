// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;

use reqwest;
use serde::{Deserialize, Serialize};

/// CreateDashboardListItemsError is a struct for typed errors of method [`DashboardListsAPI::create_dashboard_list_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardListItemsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardListItemsError is a struct for typed errors of method [`DashboardListsAPI::delete_dashboard_list_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardListItemsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDashboardListItemsError is a struct for typed errors of method [`DashboardListsAPI::get_dashboard_list_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardListItemsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardListItemsError is a struct for typed errors of method [`DashboardListsAPI::update_dashboard_list_items`]
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
        dashboard_list_id: i64,
        body: crate::datadogV2::model::DashboardListAddItemsRequest,
    ) -> Result<
        crate::datadogV2::model::DashboardListAddItemsResponse,
        Error<CreateDashboardListItemsError>,
    > {
        match self
            .create_dashboard_list_items_with_http_info(dashboard_list_id, body)
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

    /// Add dashboards to an existing dashboard list.
    pub async fn create_dashboard_list_items_with_http_info(
        &self,
        dashboard_list_id: i64,
        body: crate::datadogV2::model::DashboardListAddItemsRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListAddItemsResponse>,
        Error<CreateDashboardListItemsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_dashboard_list_items";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.get_operation_host(operation_id),
            dashboard_list_id = dashboard_list_id
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
            match serde_json::from_str::<crate::datadogV2::model::DashboardListAddItemsResponse>(
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
        dashboard_list_id: i64,
        body: crate::datadogV2::model::DashboardListDeleteItemsRequest,
    ) -> Result<
        crate::datadogV2::model::DashboardListDeleteItemsResponse,
        Error<DeleteDashboardListItemsError>,
    > {
        match self
            .delete_dashboard_list_items_with_http_info(dashboard_list_id, body)
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

    /// Delete dashboards from an existing dashboard list.
    pub async fn delete_dashboard_list_items_with_http_info(
        &self,
        dashboard_list_id: i64,
        body: crate::datadogV2::model::DashboardListDeleteItemsRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListDeleteItemsResponse>,
        Error<DeleteDashboardListItemsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_dashboard_list_items";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.get_operation_host(operation_id),
            dashboard_list_id = dashboard_list_id
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
            match serde_json::from_str::<crate::datadogV2::model::DashboardListDeleteItemsResponse>(
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
        dashboard_list_id: i64,
    ) -> Result<crate::datadogV2::model::DashboardListItems, Error<GetDashboardListItemsError>>
    {
        match self
            .get_dashboard_list_items_with_http_info(dashboard_list_id)
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

    /// Fetch the dashboard list’s dashboard definitions.
    pub async fn get_dashboard_list_items_with_http_info(
        &self,
        dashboard_list_id: i64,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListItems>,
        Error<GetDashboardListItemsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_dashboard_list_items";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.get_operation_host(operation_id),
            dashboard_list_id = dashboard_list_id
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
            match serde_json::from_str::<crate::datadogV2::model::DashboardListItems>(
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
        dashboard_list_id: i64,
        body: crate::datadogV2::model::DashboardListUpdateItemsRequest,
    ) -> Result<
        crate::datadogV2::model::DashboardListUpdateItemsResponse,
        Error<UpdateDashboardListItemsError>,
    > {
        match self
            .update_dashboard_list_items_with_http_info(dashboard_list_id, body)
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

    /// Update dashboards of an existing dashboard list.
    pub async fn update_dashboard_list_items_with_http_info(
        &self,
        dashboard_list_id: i64,
        body: crate::datadogV2::model::DashboardListUpdateItemsRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DashboardListUpdateItemsResponse>,
        Error<UpdateDashboardListItemsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_dashboard_list_items";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
            local_configuration.get_operation_host(operation_id),
            dashboard_list_id = dashboard_list_id
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
            match serde_json::from_str::<crate::datadogV2::model::DashboardListUpdateItemsResponse>(
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
