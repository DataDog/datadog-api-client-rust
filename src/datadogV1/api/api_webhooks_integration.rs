// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateWebhooksIntegrationParams is a struct for passing parameters to the method [`CreateWebhooksIntegration`]
#[derive(Clone, Debug, Default)]
pub struct CreateWebhooksIntegrationParams {
    /// Create a webhooks integration request body.
    pub body: crate::datadogV1::model::WebhooksIntegration,
}

/// CreateWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`CreateWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug, Default)]
pub struct CreateWebhooksIntegrationCustomVariableParams {
    /// Define a custom variable request body.
    pub body: crate::datadogV1::model::WebhooksIntegrationCustomVariable,
}

/// DeleteWebhooksIntegrationParams is a struct for passing parameters to the method [`DeleteWebhooksIntegration`]
#[derive(Clone, Debug, Default)]
pub struct DeleteWebhooksIntegrationParams {
    /// The name of the webhook.
    pub webhook_name: String,
}

/// DeleteWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`DeleteWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug, Default)]
pub struct DeleteWebhooksIntegrationCustomVariableParams {
    /// The name of the custom variable.
    pub custom_variable_name: String,
}

/// GetWebhooksIntegrationParams is a struct for passing parameters to the method [`GetWebhooksIntegration`]
#[derive(Clone, Debug, Default)]
pub struct GetWebhooksIntegrationParams {
    /// The name of the webhook.
    pub webhook_name: String,
}

/// GetWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`GetWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug, Default)]
pub struct GetWebhooksIntegrationCustomVariableParams {
    /// The name of the custom variable.
    pub custom_variable_name: String,
}

/// UpdateWebhooksIntegrationParams is a struct for passing parameters to the method [`UpdateWebhooksIntegration`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWebhooksIntegrationParams {
    /// The name of the webhook.
    pub webhook_name: String,
    /// Update an existing Datadog-Webhooks integration.
    pub body: crate::datadogV1::model::WebhooksIntegrationUpdateRequest,
}

/// UpdateWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`UpdateWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWebhooksIntegrationCustomVariableParams {
    /// The name of the custom variable.
    pub custom_variable_name: String,
    /// Update an existing custom variable request body.
    pub body: crate::datadogV1::model::WebhooksIntegrationCustomVariableUpdateRequest,
}

/// CreateWebhooksIntegrationError is a struct for typed errors of method [`CreateWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhooksIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`CreateWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhooksIntegrationCustomVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteWebhooksIntegrationError is a struct for typed errors of method [`DeleteWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhooksIntegrationError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`DeleteWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhooksIntegrationCustomVariableError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetWebhooksIntegrationError is a struct for typed errors of method [`GetWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhooksIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`GetWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhooksIntegrationCustomVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateWebhooksIntegrationError is a struct for typed errors of method [`UpdateWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhooksIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`UpdateWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhooksIntegrationCustomVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct WebhooksIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for WebhooksIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl WebhooksIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Creates an endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn create_webhooks_integration(
        &self,
        params: CreateWebhooksIntegrationParams,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegration>,
        Error<CreateWebhooksIntegrationError>,
    > {
        match self
            .create_webhooks_integration_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates an endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn create_webhooks_integration_with_http_info(
        &self,
        params: CreateWebhooksIntegrationParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegration>,
        Error<CreateWebhooksIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks",
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
            let local_entity: Option<crate::datadogV1::model::WebhooksIntegration> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Creates an endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn create_webhooks_integration_custom_variable(
        &self,
        params: CreateWebhooksIntegrationCustomVariableParams,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<CreateWebhooksIntegrationCustomVariableError>,
    > {
        match self
            .create_webhooks_integration_custom_variable_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates an endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn create_webhooks_integration_custom_variable_with_http_info(
        &self,
        params: CreateWebhooksIntegrationCustomVariableParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<CreateWebhooksIntegrationCustomVariableError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables",
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
            let local_entity: Option<
                crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateWebhooksIntegrationCustomVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes the endpoint with the name `<WEBHOOK NAME>`.
    pub async fn delete_webhooks_integration(
        &self,
        params: DeleteWebhooksIntegrationParams,
    ) -> Result<Option<()>, Error<DeleteWebhooksIntegrationError>> {
        match self
            .delete_webhooks_integration_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes the endpoint with the name `<WEBHOOK NAME>`.
    pub async fn delete_webhooks_integration_with_http_info(
        &self,
        params: DeleteWebhooksIntegrationParams,
    ) -> Result<ResponseContent<()>, Error<DeleteWebhooksIntegrationError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let webhook_name = params.webhook_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks/{webhook_name}",
            local_configuration.base_path,
            webhook_name = urlencode(webhook_name)
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
            let local_entity: Option<DeleteWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn delete_webhooks_integration_custom_variable(
        &self,
        params: DeleteWebhooksIntegrationCustomVariableParams,
    ) -> Result<Option<()>, Error<DeleteWebhooksIntegrationCustomVariableError>> {
        match self
            .delete_webhooks_integration_custom_variable_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn delete_webhooks_integration_custom_variable_with_http_info(
        &self,
        params: DeleteWebhooksIntegrationCustomVariableParams,
    ) -> Result<ResponseContent<()>, Error<DeleteWebhooksIntegrationCustomVariableError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let custom_variable_name = params.custom_variable_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables/{custom_variable_name}",
            local_configuration.base_path,
            custom_variable_name = urlencode(custom_variable_name)
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
            let local_entity: Option<DeleteWebhooksIntegrationCustomVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Gets the content of the webhook with the name `<WEBHOOK_NAME>`.
    pub async fn get_webhooks_integration(
        &self,
        params: GetWebhooksIntegrationParams,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegration>,
        Error<GetWebhooksIntegrationError>,
    > {
        match self.get_webhooks_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Gets the content of the webhook with the name `<WEBHOOK_NAME>`.
    pub async fn get_webhooks_integration_with_http_info(
        &self,
        params: GetWebhooksIntegrationParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegration>,
        Error<GetWebhooksIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let webhook_name = params.webhook_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks/{webhook_name}",
            local_configuration.base_path,
            webhook_name = urlencode(webhook_name)
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
            let local_entity: Option<crate::datadogV1::model::WebhooksIntegration> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Shows the content of the custom variable with the name `<CUSTOM_VARIABLE_NAME>`.
    ///
    /// If the custom variable is secret, the value does not return in the
    /// response payload.
    pub async fn get_webhooks_integration_custom_variable(
        &self,
        params: GetWebhooksIntegrationCustomVariableParams,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<GetWebhooksIntegrationCustomVariableError>,
    > {
        match self
            .get_webhooks_integration_custom_variable_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Shows the content of the custom variable with the name `<CUSTOM_VARIABLE_NAME>`.
    ///
    /// If the custom variable is secret, the value does not return in the
    /// response payload.
    pub async fn get_webhooks_integration_custom_variable_with_http_info(
        &self,
        params: GetWebhooksIntegrationCustomVariableParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<GetWebhooksIntegrationCustomVariableError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let custom_variable_name = params.custom_variable_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables/{custom_variable_name}",
            local_configuration.base_path,
            custom_variable_name = urlencode(custom_variable_name)
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
            let local_entity: Option<
                crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetWebhooksIntegrationCustomVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates the endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn update_webhooks_integration(
        &self,
        params: UpdateWebhooksIntegrationParams,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegration>,
        Error<UpdateWebhooksIntegrationError>,
    > {
        match self
            .update_webhooks_integration_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates the endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn update_webhooks_integration_with_http_info(
        &self,
        params: UpdateWebhooksIntegrationParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegration>,
        Error<UpdateWebhooksIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let webhook_name = params.webhook_name;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks/{webhook_name}",
            local_configuration.base_path,
            webhook_name = urlencode(webhook_name)
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::WebhooksIntegration> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn update_webhooks_integration_custom_variable(
        &self,
        params: UpdateWebhooksIntegrationCustomVariableParams,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<UpdateWebhooksIntegrationCustomVariableError>,
    > {
        match self
            .update_webhooks_integration_custom_variable_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn update_webhooks_integration_custom_variable_with_http_info(
        &self,
        params: UpdateWebhooksIntegrationCustomVariableParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<UpdateWebhooksIntegrationCustomVariableError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let custom_variable_name = params.custom_variable_name;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables/{custom_variable_name}",
            local_configuration.base_path,
            custom_variable_name = urlencode(custom_variable_name)
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateWebhooksIntegrationCustomVariableError> =
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
