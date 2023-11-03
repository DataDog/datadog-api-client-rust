// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateAzureIntegrationParams is a struct for passing parameters to the method [`CreateAzureIntegration`]
#[derive(Clone, Debug, Default)]
pub struct CreateAzureIntegrationParams {
    /// Create a Datadog-Azure integration for your Datadog account request body.
    pub body: crate::datadogV1::model::AzureAccount,
}

/// DeleteAzureIntegrationParams is a struct for passing parameters to the method [`DeleteAzureIntegration`]
#[derive(Clone, Debug, Default)]
pub struct DeleteAzureIntegrationParams {
    /// Delete a given Datadog-Azure integration request body.
    pub body: crate::datadogV1::model::AzureAccount,
}

/// UpdateAzureHostFiltersParams is a struct for passing parameters to the method [`UpdateAzureHostFilters`]
#[derive(Clone, Debug, Default)]
pub struct UpdateAzureHostFiltersParams {
    /// Update a Datadog-Azure integration's host filters request body.
    pub body: crate::datadogV1::model::AzureAccount,
}

/// UpdateAzureIntegrationParams is a struct for passing parameters to the method [`UpdateAzureIntegration`]
#[derive(Clone, Debug, Default)]
pub struct UpdateAzureIntegrationParams {
    /// Update a Datadog-Azure integration request body.
    pub body: crate::datadogV1::model::AzureAccount,
}

/// CreateAzureIntegrationError is a struct for typed errors of method [`CreateAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAzureIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAzureIntegrationError is a struct for typed errors of method [`DeleteAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAzureIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAzureIntegrationError is a struct for typed errors of method [`ListAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAzureIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAzureHostFiltersError is a struct for typed errors of method [`UpdateAzureHostFilters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAzureHostFiltersError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAzureIntegrationError is a struct for typed errors of method [`UpdateAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAzureIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct AzureIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for AzureIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl AzureIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a Datadog-Azure integration.
    ///
    /// Using the `POST` method updates your integration configuration by adding your new
    /// configuration to the existing one in your Datadog organization.
    ///
    /// Using the `PUT` method updates your integration configuration by replacing your
    /// current configuration with the new one sent to your Datadog organization.
    pub async fn create_azure_integration(
        &self,
        params: CreateAzureIntegrationParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<CreateAzureIntegrationError>,
    > {
        match self.create_azure_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Datadog-Azure integration.
    ///
    /// Using the `POST` method updates your integration configuration by adding your new
    /// configuration to the existing one in your Datadog organization.
    ///
    /// Using the `PUT` method updates your integration configuration by replacing your
    /// current configuration with the new one sent to your Datadog organization.
    pub async fn create_azure_integration_with_http_info(
        &self,
        params: CreateAzureIntegrationParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<CreateAzureIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/azure", local_configuration.base_path);
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateAzureIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a given Datadog-Azure integration from your Datadog account.
    pub async fn delete_azure_integration(
        &self,
        params: DeleteAzureIntegrationParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteAzureIntegrationError>,
    > {
        match self.delete_azure_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a given Datadog-Azure integration from your Datadog account.
    pub async fn delete_azure_integration_with_http_info(
        &self,
        params: DeleteAzureIntegrationParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteAzureIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/azure", local_configuration.base_path);
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteAzureIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all Datadog-Azure integrations configured in your Datadog account.
    pub async fn list_azure_integration(
        &self,
    ) -> Result<Option<Vec<crate::datadogV1::model::AzureAccount>>, Error<ListAzureIntegrationError>>
    {
        match self.list_azure_integration_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all Datadog-Azure integrations configured in your Datadog account.
    pub async fn list_azure_integration_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::AzureAccount>>,
        Error<ListAzureIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/azure", local_configuration.base_path);
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
            let local_entity: Option<Vec<crate::datadogV1::model::AzureAccount>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListAzureIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the defined list of host filters for a given Datadog-Azure integration.
    pub async fn update_azure_host_filters(
        &self,
        params: UpdateAzureHostFiltersParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateAzureHostFiltersError>,
    > {
        match self.update_azure_host_filters_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the defined list of host filters for a given Datadog-Azure integration.
    pub async fn update_azure_host_filters_with_http_info(
        &self,
        params: UpdateAzureHostFiltersParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateAzureHostFiltersError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/azure/host_filters",
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateAzureHostFiltersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a Datadog-Azure integration. Requires an existing `tenant_name` and `client_id`.
    /// Any other fields supplied will overwrite existing values. To overwrite `tenant_name` or `client_id`,
    /// use `new_tenant_name` and `new_client_id`. To leave a field unchanged, do not supply that field in the payload.
    pub async fn update_azure_integration(
        &self,
        params: UpdateAzureIntegrationParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateAzureIntegrationError>,
    > {
        match self.update_azure_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a Datadog-Azure integration. Requires an existing `tenant_name` and `client_id`.
    /// Any other fields supplied will overwrite existing values. To overwrite `tenant_name` or `client_id`,
    /// use `new_tenant_name` and `new_client_id`. To leave a field unchanged, do not supply that field in the payload.
    pub async fn update_azure_integration_with_http_info(
        &self,
        params: UpdateAzureIntegrationParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateAzureIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/azure", local_configuration.base_path);
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateAzureIntegrationError> =
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
