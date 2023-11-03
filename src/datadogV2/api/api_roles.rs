// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListPermissionsError is a struct for typed errors of method [`ListPermissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPermissionsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct RolesAPI {
    config: configuration::Configuration,
}

impl Default for RolesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl RolesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Returns a list of all permissions, including name, description, and ID.
    pub async fn list_permissions(
        &self,
    ) -> Result<Option<crate::datadogV2::model::PermissionsResponse>, Error<ListPermissionsError>>
    {
        match self.list_permissions_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns a list of all permissions, including name, description, and ID.
    pub async fn list_permissions_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PermissionsResponse>,
        Error<ListPermissionsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/permissions", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::PermissionsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListPermissionsError> =
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
